//! F02: Policy Chatbot for Citizens
//!
//! Provides an interactive chatbot that explains policy positions to citizens
//! in plain language, answering questions and clarifying platform details.
//!
//! Uses RAG (Retrieval-Augmented Generation) to ground responses in actual
//! policy documents stored in a Qdrant vector store.

use dioxus::prelude::*;

use crate::models::document::{ChatMessage, ChatSession, Document};

/// Default Qdrant collection name for policy documents.
const COLLECTION_NAME: &str = "policy_documents";

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

/// System prompt for the policy chatbot.
const SYSTEM_PROMPT: &str = "\
You are a helpful policy assistant for a political campaign. Your job is to answer \
citizens' questions about policy positions accurately and in plain language.

Use ONLY the provided context to answer questions. If the context does not contain \
enough information to answer, say so honestly. Do not make up policies or positions.

When citing information, reference the source document title.

Keep answers concise, factual, and non-partisan in tone.";

/// Ingest a document into the policy knowledge base.
///
/// Chunks the document text, embeds each chunk, stores vectors in Qdrant,
/// and records metadata in the documents table.
#[server(endpoint = "policy-chatbot/ingest-document")]
pub async fn ingest_document(
    title: String,
    content: String,
    collection: String,
) -> Result<String, ServerFnError> {
    use crate::infrastructure::{
        chunk_text, content_hash, require_user, EmbeddingClient, ServerState, VectorStoreClient,
    };
    use crate::infrastructure::vector_store::VectorPoint;
    use dioxus::fullstack::FullstackContext;

    let user = require_user().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();
    let collection = if collection.is_empty() { COLLECTION_NAME.to_string() } else { collection };
    let hash = content_hash(&content);

    // Check for duplicate content
    let existing: Option<String> = sqlx::query_scalar(
        "SELECT id::text FROM documents WHERE content_hash = $1 AND collection_name = $2",
    )
    .bind(&hash)
    .bind(&collection)
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
        return Err(ServerFnError::new("Document content is empty or too short to chunk"));
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

    vs.ensure_collection(&collection, vector_size)
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
                    "chunk_index": i,
                    "chunk_text": chunk_texts[i],
                    "collection": collection,
                }),
            }
        })
        .collect();

    vs.upsert(&collection, points)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Record in database (no ingested_by column — the migration only has ingested_at)
    sqlx::query(
        "INSERT INTO documents (id, title, content_hash, collection_name, chunk_count, status)
         VALUES ($1::uuid, $2, $3, $4, $5, 'active')",
    )
    .bind(&doc_id)
    .bind(&title)
    .bind(&hash)
    .bind(&collection)
    .bind(chunk_count)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    tracing::info!(
        doc_id = %doc_id,
        title = %title,
        chunks = chunk_count,
        "Policy document ingested"
    );

    Ok(doc_id)
}

/// List all documents in a collection.
#[server(endpoint = "policy-chatbot/list-documents")]
pub async fn list_documents(collection: String) -> Result<Vec<Document>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();
    let collection = if collection.is_empty() { COLLECTION_NAME.to_string() } else { collection };

    let rows = sqlx::query(
        r#"SELECT
            id::text,
            title,
            source_path,
            content_hash,
            collection_name,
            chunk_count,
            COALESCE(tags, '{}') AS tags,
            to_char(ingested_at, 'YYYY-MM-DD HH24:MI:SS') AS ingested_at,
            status
        FROM documents
        WHERE collection_name = $1 AND status = 'active'
        ORDER BY ingested_at DESC"#,
    )
    .bind(&collection)
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let documents: Vec<Document> = rows
        .iter()
        .map(|row| Document {
            id: row.get::<String, _>("id"),
            title: row.get::<String, _>("title"),
            source_path: row.get::<Option<String>, _>("source_path"),
            content_hash: row.get::<Option<String>, _>("content_hash"),
            collection_name: row.get::<String, _>("collection_name"),
            chunk_count: row.get::<i32, _>("chunk_count"),
            tags: row.get::<Vec<String>, _>("tags"),
            ingested_at: row.get::<Option<String>, _>("ingested_at"),
            status: row.get::<String, _>("status"),
        })
        .collect();

    Ok(documents)
}

/// Delete a document and its associated Qdrant vectors.
#[server(endpoint = "policy-chatbot/delete-document")]
pub async fn delete_document(doc_id: String) -> Result<(), ServerFnError> {
    use crate::infrastructure::{require_user, ServerState, VectorStoreClient};
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let _user = require_user().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    // Fetch document to get collection and chunk count
    let doc = sqlx::query(
        "SELECT collection_name, chunk_count FROM documents WHERE id::text = $1",
    )
    .bind(&doc_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
    .ok_or_else(|| ServerFnError::new("Document not found"))?;

    let collection_name: String = doc.get("collection_name");
    let chunk_count: i32 = doc.get("chunk_count");

    // Build vector point IDs to delete
    let point_ids: Vec<String> = (0..chunk_count)
        .map(|i| format!("{doc_id}_chunk_{i}"))
        .collect();

    // Delete from Qdrant
    let vs = VectorStoreClient::new(&state.vector_store_config.url);
    vs.delete(&collection_name, &point_ids)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Mark as deleted in database
    sqlx::query("UPDATE documents SET status = 'deleted' WHERE id::text = $1")
        .bind(&doc_id)
        .execute(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    tracing::info!(doc_id = %doc_id, "Policy document deleted");

    Ok(())
}

/// RAG-based chat: embed the question, retrieve relevant chunks, generate an answer.
#[server(endpoint = "policy-chatbot/chat")]
pub async fn chat(
    question: String,
    session_id: String,
    collection: String,
) -> Result<ChatMessage, ServerFnError> {
    use crate::infrastructure::{
        require_user, EmbeddingClient, LlmClient, ServerState, VectorStoreClient,
    };
    use crate::infrastructure::llm::LlmMessage;
    use dioxus::fullstack::FullstackContext;

    let user = require_user().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();
    let collection = if collection.is_empty() { COLLECTION_NAME.to_string() } else { collection };

    // Save user question
    let user_msg_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO chat_messages (id, session_id, role, content)
         VALUES ($1::uuid, $2::uuid, 'user', $3)",
    )
    .bind(&user_msg_id)
    .bind(&session_id)
    .bind(&question)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    // Embed the question
    let embedder = EmbeddingClient::new(
        &state.embedding_config.base_url,
        &state.embedding_config.model,
    );

    let query_embedding = embedder
        .embed_text(&question)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Search for relevant chunks
    let vs = VectorStoreClient::new(&state.vector_store_config.url);
    let results = vs
        .search(&collection, query_embedding, TOP_K, Some(SCORE_THRESHOLD))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Build context from retrieved chunks
    let mut context_parts = Vec::new();
    let mut sources = Vec::new();

    for result in &results {
        let title = result.payload["title"].as_str().unwrap_or("Unknown");
        let chunk_text = result.payload["chunk_text"].as_str().unwrap_or("");
        let score = result.score;

        context_parts.push(format!("[Source: {title} (relevance: {score:.2})]\n{chunk_text}"));
        sources.push(serde_json::json!({
            "title": title,
            "score": score,
            "document_id": result.payload["document_id"],
            "chunk_index": result.payload["chunk_index"],
        }));
    }

    let context = if context_parts.is_empty() {
        "No relevant policy documents were found for this question.".to_string()
    } else {
        context_parts.join("\n\n---\n\n")
    };

    // Build LLM messages
    let user_prompt = format!(
        "Context from policy documents:\n\n{context}\n\n---\n\nQuestion: {question}"
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

    // Log LLM usage
    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "policy_chatbot",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    // Save assistant response
    let assistant_msg_id = uuid::Uuid::new_v4().to_string();
    let sources_json = serde_json::to_value(&sources)
        .unwrap_or_else(|_| serde_json::Value::Null);

    sqlx::query(
        "INSERT INTO chat_messages (id, session_id, role, content, sources)
         VALUES ($1::uuid, $2::uuid, 'assistant', $3, $4)",
    )
    .bind(&assistant_msg_id)
    .bind(&session_id)
    .bind(&response.content)
    .bind(&sources_json)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    // Update session last_active
    sqlx::query("UPDATE chat_sessions SET last_active = NOW() WHERE id::text = $1")
        .bind(&session_id)
        .execute(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    Ok(ChatMessage {
        id: assistant_msg_id,
        session_id,
        role: "assistant".to_string(),
        content: response.content,
        sources: Some(sources_json),
        created_at: None,
    })
}

/// Create a new chat session for the current user.
#[server(endpoint = "policy-chatbot/create-session")]
pub async fn create_chat_session() -> Result<ChatSession, ServerFnError> {
    use crate::infrastructure::{require_user, ServerState};
    use dioxus::fullstack::FullstackContext;

    let user = require_user().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();
    let session_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO chat_sessions (id, user_id, session_type, metadata)
         VALUES ($1::uuid, $2, 'policy_chatbot', '{}'::jsonb)",
    )
    .bind(&session_id)
    .bind(&user.sub)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    Ok(ChatSession {
        id: session_id,
        user_id: Some(user.sub),
        session_type: "policy_chatbot".to_string(),
        created_at: None,
        last_active: None,
        metadata: serde_json::json!({}),
    })
}

/// List all chat sessions for the current user.
#[server(endpoint = "policy-chatbot/list-sessions")]
pub async fn list_chat_sessions() -> Result<Vec<ChatSession>, ServerFnError> {
    use crate::infrastructure::{require_user, ServerState};
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let user = require_user().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    let rows = sqlx::query(
        r#"SELECT
            id::text,
            user_id,
            session_type,
            to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at,
            to_char(last_active, 'YYYY-MM-DD HH24:MI:SS') AS last_active,
            COALESCE(metadata, '{}'::jsonb) AS metadata
        FROM chat_sessions
        WHERE user_id = $1 AND session_type = 'policy_chatbot'
        ORDER BY last_active DESC NULLS LAST"#,
    )
    .bind(&user.sub)
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let sessions: Vec<ChatSession> = rows
        .iter()
        .map(|row| ChatSession {
            id: row.get::<String, _>("id"),
            user_id: row.get::<Option<String>, _>("user_id"),
            session_type: row.get::<String, _>("session_type"),
            created_at: row.get::<Option<String>, _>("created_at"),
            last_active: row.get::<Option<String>, _>("last_active"),
            metadata: row.get::<serde_json::Value, _>("metadata"),
        })
        .collect();

    Ok(sessions)
}

/// Get all messages for a chat session.
#[server(endpoint = "policy-chatbot/get-messages")]
pub async fn get_chat_messages(session_id: String) -> Result<Vec<ChatMessage>, ServerFnError> {
    use crate::infrastructure::{require_user, ServerState};
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let _user = require_user().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    let rows = sqlx::query(
        r#"SELECT
            id::text,
            session_id::text,
            role,
            content,
            sources,
            to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
        FROM chat_messages
        WHERE session_id::text = $1
        ORDER BY created_at ASC"#,
    )
    .bind(&session_id)
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let messages: Vec<ChatMessage> = rows
        .iter()
        .map(|row| ChatMessage {
            id: row.get::<String, _>("id"),
            session_id: row.get::<String, _>("session_id"),
            role: row.get::<String, _>("role"),
            content: row.get::<String, _>("content"),
            sources: row.get::<Option<serde_json::Value>, _>("sources"),
            created_at: row.get::<Option<String>, _>("created_at"),
        })
        .collect();

    Ok(messages)
}
