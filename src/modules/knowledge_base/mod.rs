#![cfg_attr(not(feature = "server"), allow(dead_code))]
//! F25: Internal Knowledge Base Q&A
//!
//! Provides a searchable internal knowledge base with AI-powered Q&A
//! for campaign staff to quickly find policies, procedures, and talking points.
//! Uses RAG (Retrieval-Augmented Generation) with Qdrant vector store.

use dioxus::prelude::*;

/// Default Qdrant collection name for knowledge base documents.
const COLLECTION_NAME: &str = "knowledge_base";

/// Default vector dimension (matches common embedding models).
const VECTOR_SIZE: u64 = 1536;

/// Number of top chunks to retrieve for RAG context.
const TOP_K: usize = 5;

/// Minimum similarity score threshold for retrieved chunks.
const SCORE_THRESHOLD: f32 = 0.3;

/// Chunk size in words for document splitting.
const CHUNK_SIZE: usize = 300;

/// Overlap in words between consecutive chunks.
const CHUNK_OVERLAP: usize = 50;

/// An answer generated from the knowledge base.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KBAnswer {
    pub id: String,
    pub question: String,
    pub answer: String,
    pub sources: Vec<String>,
    pub confidence: f32,
    pub created_at: String,
}

/// A document stored in the knowledge base.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KBDocument {
    pub id: String,
    pub title: String,
    pub category: String,
    pub content_preview: String,
    pub created_at: String,
}

/// System prompt for knowledge base Q&A.
const SYSTEM_PROMPT: &str = "\
You are an internal knowledge base assistant for a political campaign. Your job is to \
answer staff questions accurately using the provided context from internal documents.

Use ONLY the provided context to answer questions. If the context does not contain \
enough information to answer, say so honestly and suggest what kind of document \
might contain the answer.

When citing information, reference the source document title. Keep answers clear, \
concise, and actionable for campaign staff.

You MUST respond with valid JSON matching this exact schema:
{
  \"answer\": \"Your detailed answer here, referencing source documents\",
  \"sources\": [\"Document Title 1\", \"Document Title 2\"],
  \"confidence\": 0.85
}

The confidence score should be between 0.0 and 1.0:
- 0.8-1.0: Answer is well-supported by the context
- 0.5-0.8: Answer is partially supported, some inference required
- 0.0-0.5: Limited context available, answer may be incomplete";

/// Ask a question against the knowledge base using RAG.
///
/// Embeds the question, searches Qdrant for relevant chunks, then uses
/// LLM to generate an answer grounded in the retrieved context.
#[server(endpoint = "knowledge-base/ask")]
pub async fn ask_knowledge_base(
    question: String,
    category: String,
) -> Result<KBAnswer, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{EmbeddingClient, LlmClient, ServerState, VectorStoreClient};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if question.trim().is_empty() {
        return Err(ServerFnError::new("Question is required"));
    }

    let pool = state.db.pool();

    // Embed the question
    let embedder = EmbeddingClient::new(
        &state.embedding_config.base_url,
        &state.embedding_config.model,
    );

    let query_embedding = embedder
        .embed_text(&question)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Search for relevant chunks in Qdrant
    let vs = VectorStoreClient::new(&state.vector_store_config.url);
    let results = vs
        .search(
            COLLECTION_NAME,
            query_embedding,
            TOP_K,
            Some(SCORE_THRESHOLD),
        )
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Filter by category if provided
    let filtered_results: Vec<_> = if category.is_empty() || category == "All" {
        results
    } else {
        results
            .into_iter()
            .filter(|r| {
                r.payload
                    .get("category")
                    .and_then(|c| c.as_str())
                    .map(|c| c == category)
                    .unwrap_or(true) // include if no category metadata
            })
            .collect()
    };

    // Build context from retrieved chunks
    let mut context_parts = Vec::new();
    let mut source_titles = Vec::new();

    for result in &filtered_results {
        let title = result.payload["title"].as_str().unwrap_or("Unknown");
        let chunk_text = result.payload["chunk_text"].as_str().unwrap_or("");
        let score = result.score;

        context_parts.push(format!(
            "[Source: {title} (relevance: {score:.2})]\n{chunk_text}"
        ));
        if !source_titles.contains(&title.to_string()) {
            source_titles.push(title.to_string());
        }
    }

    let context = if context_parts.is_empty() {
        "No relevant documents were found in the knowledge base for this question.".to_string()
    } else {
        context_parts.join("\n\n---\n\n")
    };

    // Build LLM messages
    let category_hint = if !category.is_empty() && category != "All" {
        format!("\nCategory filter: {category}")
    } else {
        String::new()
    };

    let user_prompt = format!(
        "Context from knowledge base documents:\n\n{context}\n\n---\n\n\
        Question: {question}{category_hint}"
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: SYSTEM_PROMPT.to_string(),
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    // Generate answer
    let start = std::time::Instant::now();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let response = llm
        .generate(&messages, None, Some(0.3), Some(1024))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "knowledge_base",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    // Parse the LLM JSON response
    let raw = response.content.trim();
    let json_str = if let Some(start_idx) = raw.find('{') {
        if let Some(end_idx) = raw.rfind('}') {
            &raw[start_idx..=end_idx]
        } else {
            raw
        }
    } else {
        raw
    };

    let parsed: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| ServerFnError::new(format!("Failed to parse LLM response as JSON: {e}")))?;

    let answer = parsed
        .get("answer")
        .and_then(|v| v.as_str())
        .unwrap_or(&response.content)
        .to_string();

    let sources: Vec<String> = parsed
        .get("sources")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or(source_titles);

    let confidence = parsed.get("confidence").and_then(|v| v.as_f64()).unwrap_or(
        if filtered_results.is_empty() {
            0.1
        } else {
            0.7
        },
    ) as f32;

    let answer_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tracing::info!(
        answer_id = %answer_id,
        sources = sources.len(),
        confidence = confidence,
        latency_ms = latency_ms,
        "Knowledge base question answered"
    );

    Ok(KBAnswer {
        id: answer_id,
        question,
        answer,
        sources,
        confidence,
        created_at: now,
    })
}

/// Ingest a document into the knowledge base.
///
/// Chunks the document text, embeds each chunk, and stores vectors in Qdrant.
/// Also records document metadata in the documents database table.
#[server(endpoint = "knowledge-base/ingest-document")]
pub async fn ingest_kb_document(
    title: String,
    content: String,
    category: String,
) -> Result<String, ServerFnError> {
    use crate::infrastructure::vector_store::VectorPoint;
    use crate::infrastructure::{
        chunk_text, content_hash, EmbeddingClient, ServerState, VectorStoreClient,
    };
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if title.trim().is_empty() {
        return Err(ServerFnError::new("Document title is required"));
    }
    if content.trim().is_empty() {
        return Err(ServerFnError::new("Document content is required"));
    }

    let pool = state.db.pool();
    let hash = content_hash(&content);
    let category = if category.is_empty() {
        "General".to_string()
    } else {
        category
    };

    // Check for duplicate content
    let existing: Option<String> = sqlx::query_scalar(
        "SELECT id::text FROM documents WHERE content_hash = $1 AND collection_name = $2",
    )
    .bind(&hash)
    .bind(COLLECTION_NAME)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    if let Some(existing_id) = existing {
        return Ok(existing_id);
    }

    // Chunk the document
    let chunks = chunk_text(&content, CHUNK_SIZE, CHUNK_OVERLAP);
    let chunk_count = chunks.len() as i32;

    if chunks.is_empty() {
        return Err(ServerFnError::new(
            "Document content is empty or too short to chunk",
        ));
    }

    // Embed all chunks
    let embedder = EmbeddingClient::new(
        &state.embedding_config.base_url,
        &state.embedding_config.model,
    );

    let chunk_texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();
    let embeddings = embedder
        .embed_batch(&chunk_texts)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Store in Qdrant
    let vs = VectorStoreClient::new(&state.vector_store_config.url);
    let vector_size = embeddings
        .first()
        .map(|v| v.len() as u64)
        .unwrap_or(VECTOR_SIZE);

    vs.ensure_collection(COLLECTION_NAME, vector_size)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let doc_id = uuid::Uuid::new_v4().to_string();
    let points: Vec<VectorPoint> = embeddings
        .into_iter()
        .enumerate()
        .map(|(i, vector)| {
            let point_id = format!("{doc_id}_chunk_{i}");
            VectorPoint {
                id: point_id,
                vector,
                payload: serde_json::json!({
                    "document_id": doc_id,
                    "title": title,
                    "category": category,
                    "chunk_index": i,
                    "chunk_text": chunk_texts[i],
                    "collection": COLLECTION_NAME,
                }),
            }
        })
        .collect();

    vs.upsert(COLLECTION_NAME, points)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Record in database
    sqlx::query(
        "INSERT INTO documents (id, title, content_hash, collection_name, chunk_count, status)
         VALUES ($1::uuid, $2, $3, $4, $5, 'active')",
    )
    .bind(&doc_id)
    .bind(&title)
    .bind(&hash)
    .bind(COLLECTION_NAME)
    .bind(chunk_count)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    tracing::info!(
        doc_id = %doc_id,
        title = %title,
        category = %category,
        chunks = chunk_count,
        "Knowledge base document ingested"
    );

    Ok(doc_id)
}

/// List documents in the knowledge base, optionally filtered by category.
#[server(endpoint = "knowledge-base/list-documents")]
pub async fn list_kb_documents(category: String) -> Result<Vec<KBDocument>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    let rows = if category.is_empty() || category == "All" {
        sqlx::query(
            r#"SELECT
                id::text,
                title,
                COALESCE(tags[1], 'General') AS category,
                LEFT(content_hash, 20) AS content_preview,
                to_char(ingested_at, 'YYYY-MM-DD HH24:MI:SS') AS ingested_at
            FROM documents
            WHERE collection_name = $1 AND status = 'active'
            ORDER BY ingested_at DESC"#,
        )
        .bind(COLLECTION_NAME)
        .fetch_all(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
    } else {
        sqlx::query(
            r#"SELECT
                id::text,
                title,
                COALESCE(tags[1], 'General') AS category,
                LEFT(content_hash, 20) AS content_preview,
                to_char(ingested_at, 'YYYY-MM-DD HH24:MI:SS') AS ingested_at
            FROM documents
            WHERE collection_name = $1 AND status = 'active'
            ORDER BY ingested_at DESC"#,
        )
        .bind(COLLECTION_NAME)
        .fetch_all(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
    };

    let documents: Vec<KBDocument> = rows
        .iter()
        .map(|row| KBDocument {
            id: row.get::<String, _>("id"),
            title: row.get::<String, _>("title"),
            category: row.get::<String, _>("category"),
            content_preview: row.get::<String, _>("content_preview"),
            created_at: row
                .get::<Option<String>, _>("ingested_at")
                .unwrap_or_default(),
        })
        .collect();

    Ok(documents)
}
