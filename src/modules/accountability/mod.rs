#![cfg_attr(not(feature = "server"), allow(dead_code))]
//! F08: Manifesto Accountability Engine
//!
//! Tracks elected officials' actions against their campaign promises and
//! manifesto commitments, generating accountability reports.
//!
//! Uses LLM to extract commitments from manifesto text and classify
//! evidence against those commitments.

use dioxus::prelude::*;

/// A single policy commitment extracted from a manifesto or speech.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Commitment {
    pub id: String,
    pub text: String,
    pub topic: Option<String>,
    pub strength: Option<String>,
    pub date: Option<String>,
    pub status: String,
    pub evidence_count: i64,
    pub created_at: Option<String>,
}

/// A piece of evidence evaluated against a commitment.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Evidence {
    pub id: String,
    pub commitment_id: String,
    pub classification: Option<String>, // "fulfilled", "broken", "partial", "unrelated"
    pub confidence: f64,
    pub explanation: Option<String>,
    pub created_at: Option<String>,
}

/// Summary statistics for the accountability dashboard.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountabilitySummary {
    pub total_commitments: i64,
    pub fulfilled_pct: f64,
    pub broken_pct: f64,
    pub pending_pct: f64,
    pub partial_pct: f64,
}

/// System prompt for extracting commitments from manifesto text.
const EXTRACT_PROMPT: &str = "\
You are an expert political analyst. Your task is to extract specific, measurable \
policy commitments from the given text.

For each commitment, provide:
- \"text\": The exact promise or commitment statement
- \"topic\": The policy area (e.g. healthcare, economy, education, environment, security, infrastructure, social)
- \"strength\": How strong the commitment is: \"strong\" (definite promise), \"moderate\" (likely intent), or \"weak\" (vague aspiration)

Return a JSON array of objects. Example:
[
  {\"text\": \"We will build 10,000 new homes by 2025\", \"topic\": \"housing\", \"strength\": \"strong\"},
  {\"text\": \"We aim to reduce carbon emissions\", \"topic\": \"environment\", \"strength\": \"weak\"}
]

Only return the JSON array, no other text.";

/// System prompt for classifying evidence against a commitment.
const CLASSIFY_PROMPT: &str = "\
You are an expert political fact-checker. You will be given a policy commitment and \
a piece of evidence. Classify whether the evidence indicates the commitment was:

- \"fulfilled\": Clear evidence the promise was kept
- \"broken\": Clear evidence the promise was broken or contradicted
- \"partial\": Some progress but not fully delivered
- \"unrelated\": The evidence is not relevant to this commitment

Provide your response as a JSON object:
{\"classification\": \"fulfilled|broken|partial|unrelated\", \"confidence\": 0.0-1.0, \"explanation\": \"brief explanation\"}

Only return the JSON object, no other text.";

/// Extract commitments from a manifesto or policy document using LLM analysis.
///
/// Sends the document text to the LLM, parses extracted commitments, and
/// stores them in the database.
#[server(endpoint = "accountability/extract-commitments")]
pub async fn extract_commitments(
    document_text: String,
    document_title: String,
) -> Result<Vec<Commitment>, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    if document_text.trim().is_empty() {
        return Err(ServerFnError::new("Document text is empty"));
    }

    // Store the source document reference
    let doc_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO documents (id, title, content_hash, collection_name, chunk_count, status)
         VALUES ($1::uuid, $2, $3, 'accountability', 0, 'active')",
    )
    .bind(&doc_id)
    .bind(&document_title)
    .bind(crate::infrastructure::content_hash(&document_text))
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    // Send to LLM for extraction
    let user_prompt = format!(
        "Extract all policy commitments from the following document titled \"{document_title}\":\n\n{document_text}"
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: EXTRACT_PROMPT.to_string(),
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let start = std::time::Instant::now();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let response = llm
        .generate(&messages, None, Some(0.2), Some(4096))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "accountability",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    // Parse the JSON response
    let content = response.content.trim();
    // Try to find JSON array in the response (handle markdown code blocks)
    let json_str = if let Some(start) = content.find('[') {
        if let Some(end) = content.rfind(']') {
            &content[start..=end]
        } else {
            content
        }
    } else {
        content
    };

    let extracted: Vec<serde_json::Value> = serde_json::from_str(json_str)
        .map_err(|e| ServerFnError::new(format!("Failed to parse LLM response as JSON: {e}")))?;

    let mut commitments = Vec::new();

    for item in &extracted {
        let commitment_id = uuid::Uuid::new_v4().to_string();
        let text = item["text"].as_str().unwrap_or("").to_string();
        let topic = item["topic"].as_str().map(|s| s.to_string());
        let strength = item["strength"].as_str().map(|s| s.to_string());

        if text.is_empty() {
            continue;
        }

        sqlx::query(
            "INSERT INTO commitments (id, text, topic, strength, source_document_id, date, status, created_at)
             VALUES ($1::uuid, $2, $3, $4, $5::uuid, NOW(), 'active', NOW())",
        )
        .bind(&commitment_id)
        .bind(&text)
        .bind(&topic)
        .bind(&strength)
        .bind(&doc_id)
        .execute(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error inserting commitment: {e}")))?;

        commitments.push(Commitment {
            id: commitment_id,
            text,
            topic,
            strength,
            date: None,
            status: "active".to_string(),
            evidence_count: 0,
            created_at: None,
        });
    }

    tracing::info!(
        doc_title = %document_title,
        count = commitments.len(),
        "Extracted commitments from document"
    );

    Ok(commitments)
}

/// List all commitments, optionally filtered by topic and/or status.
#[server(endpoint = "accountability/list-commitments")]
pub async fn list_commitments(
    topic_filter: Option<String>,
    status_filter: Option<String>,
) -> Result<Vec<Commitment>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    // Build query dynamically based on filters
    let mut query = String::from(
        r#"SELECT
            c.id::text,
            c.text,
            c.topic,
            c.strength,
            to_char(c.date, 'YYYY-MM-DD') AS date,
            c.status,
            COALESCE(COUNT(ce.id), 0) AS evidence_count,
            to_char(c.created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
        FROM commitments c
        LEFT JOIN commitment_evidence ce ON ce.commitment_id = c.id
        WHERE 1=1"#,
    );

    let mut bind_idx = 1;
    let mut binds: Vec<String> = Vec::new();

    if let Some(ref topic) = topic_filter {
        if !topic.is_empty() {
            query.push_str(&format!(" AND c.topic = ${bind_idx}"));
            bind_idx += 1;
            binds.push(topic.clone());
        }
    }

    if let Some(ref status) = status_filter {
        if !status.is_empty() {
            query.push_str(&format!(" AND c.status = ${bind_idx}"));
            // bind_idx is not used after this point in the function
            let _ = bind_idx;
            binds.push(status.clone());
        }
    }

    query.push_str(
        " GROUP BY c.id, c.text, c.topic, c.strength, c.date, c.status, c.created_at
         ORDER BY c.created_at DESC",
    );

    let mut q = sqlx::query(&query);
    for b in &binds {
        q = q.bind(b);
    }

    let rows = q
        .fetch_all(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let commitments: Vec<Commitment> = rows
        .iter()
        .map(|row| Commitment {
            id: row.get::<String, _>("id"),
            text: row.get::<String, _>("text"),
            topic: row.get::<Option<String>, _>("topic"),
            strength: row.get::<Option<String>, _>("strength"),
            date: row.get::<Option<String>, _>("date"),
            status: row.get::<String, _>("status"),
            evidence_count: row.get::<i64, _>("evidence_count"),
            created_at: row.get::<Option<String>, _>("created_at"),
        })
        .collect();

    Ok(commitments)
}

/// Get a single commitment by ID along with all its evidence.
#[server(endpoint = "accountability/get-commitment")]
pub async fn get_commitment(id: String) -> Result<(Commitment, Vec<Evidence>), ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    // Fetch commitment
    let row = sqlx::query(
        r#"SELECT
            c.id::text,
            c.text,
            c.topic,
            c.strength,
            to_char(c.date, 'YYYY-MM-DD') AS date,
            c.status,
            COALESCE((SELECT COUNT(*) FROM commitment_evidence WHERE commitment_id = c.id), 0) AS evidence_count,
            to_char(c.created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
        FROM commitments c
        WHERE c.id::text = $1"#,
    )
    .bind(&id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
    .ok_or_else(|| ServerFnError::new("Commitment not found"))?;

    let commitment = Commitment {
        id: row.get::<String, _>("id"),
        text: row.get::<String, _>("text"),
        topic: row.get::<Option<String>, _>("topic"),
        strength: row.get::<Option<String>, _>("strength"),
        date: row.get::<Option<String>, _>("date"),
        status: row.get::<String, _>("status"),
        evidence_count: row.get::<i64, _>("evidence_count"),
        created_at: row.get::<Option<String>, _>("created_at"),
    };

    // Fetch evidence
    let evidence_rows = sqlx::query(
        r#"SELECT
            id::text,
            commitment_id::text,
            classification,
            confidence,
            explanation,
            to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
        FROM commitment_evidence
        WHERE commitment_id::text = $1
        ORDER BY created_at DESC"#,
    )
    .bind(&id)
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let evidence: Vec<Evidence> = evidence_rows
        .iter()
        .map(|row| Evidence {
            id: row.get::<String, _>("id"),
            commitment_id: row.get::<String, _>("commitment_id"),
            classification: row.get::<Option<String>, _>("classification"),
            confidence: row.get::<f64, _>("confidence"),
            explanation: row.get::<Option<String>, _>("explanation"),
            created_at: row.get::<Option<String>, _>("created_at"),
        })
        .collect();

    Ok((commitment, evidence))
}

/// Add evidence for a commitment using LLM classification.
///
/// Loads the commitment text, sends both commitment and evidence to the LLM
/// for classification, and stores the result.
#[server(endpoint = "accountability/add-evidence")]
pub async fn add_evidence(
    commitment_id: String,
    evidence_text: String,
) -> Result<Evidence, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    if evidence_text.trim().is_empty() {
        return Err(ServerFnError::new("Evidence text is empty"));
    }

    // Load commitment text
    let commitment_row = sqlx::query("SELECT text FROM commitments WHERE id::text = $1")
        .bind(&commitment_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
        .ok_or_else(|| ServerFnError::new("Commitment not found"))?;

    let commitment_text: String = commitment_row.get("text");

    // Send to LLM for classification
    let user_prompt = format!("Commitment: \"{commitment_text}\"\n\nEvidence: \"{evidence_text}\"");

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: CLASSIFY_PROMPT.to_string(),
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let start = std::time::Instant::now();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let response = llm
        .generate(&messages, None, Some(0.1), Some(512))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "accountability",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    // Parse the classification response
    let content = response.content.trim();
    let json_str = if let Some(start) = content.find('{') {
        if let Some(end) = content.rfind('}') {
            &content[start..=end]
        } else {
            content
        }
    } else {
        content
    };

    let parsed: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| ServerFnError::new(format!("Failed to parse LLM classification: {e}")))?;

    let classification = parsed["classification"]
        .as_str()
        .unwrap_or("unrelated")
        .to_string();
    let confidence = parsed["confidence"].as_f64().unwrap_or(0.5);
    let explanation = parsed["explanation"].as_str().map(|s| s.to_string());

    // Store evidence
    let evidence_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO commitment_evidence (id, commitment_id, classification, confidence, explanation, created_at)
         VALUES ($1::uuid, $2::uuid, $3, $4, $5, NOW())",
    )
    .bind(&evidence_id)
    .bind(&commitment_id)
    .bind(&classification)
    .bind(confidence)
    .bind(&explanation)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error inserting evidence: {e}")))?;

    // Update commitment status based on latest evidence
    let new_status = match classification.as_str() {
        "fulfilled" if confidence >= 0.7 => "fulfilled",
        "broken" if confidence >= 0.7 => "broken",
        "partial" => "partial",
        _ => "active",
    };

    sqlx::query("UPDATE commitments SET status = $1 WHERE id::text = $2")
        .bind(new_status)
        .bind(&commitment_id)
        .execute(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error updating status: {e}")))?;

    tracing::info!(
        evidence_id = %evidence_id,
        commitment_id = %commitment_id,
        classification = %classification,
        confidence = %confidence,
        "Evidence classified for commitment"
    );

    Ok(Evidence {
        id: evidence_id,
        commitment_id,
        classification: Some(classification),
        confidence,
        explanation,
        created_at: None,
    })
}

/// Get an overall accountability summary with percentages.
#[server(endpoint = "accountability/get-summary")]
pub async fn get_accountability_summary() -> Result<AccountabilitySummary, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    let row = sqlx::query(
        r#"SELECT
            COUNT(*) AS total,
            COUNT(*) FILTER (WHERE status = 'fulfilled') AS fulfilled,
            COUNT(*) FILTER (WHERE status = 'broken') AS broken,
            COUNT(*) FILTER (WHERE status = 'partial') AS partial,
            COUNT(*) FILTER (WHERE status = 'active') AS pending
        FROM commitments"#,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let total: i64 = row.get("total");
    let fulfilled: i64 = row.get("fulfilled");
    let broken: i64 = row.get("broken");
    let partial: i64 = row.get("partial");

    let (fulfilled_pct, broken_pct, partial_pct, pending_pct) = if total > 0 {
        let t = total as f64;
        (
            (fulfilled as f64 / t) * 100.0,
            (broken as f64 / t) * 100.0,
            (partial as f64 / t) * 100.0,
            ((total - fulfilled - broken - partial) as f64 / t) * 100.0,
        )
    } else {
        (0.0, 0.0, 0.0, 0.0)
    };

    Ok(AccountabilitySummary {
        total_commitments: total,
        fulfilled_pct,
        broken_pct,
        pending_pct,
        partial_pct,
    })
}

/// Get commitments that are at risk: those with "broken" status or low-confidence evidence.
#[server(endpoint = "accountability/get-at-risk")]
pub async fn get_at_risk_commitments() -> Result<Vec<Commitment>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    let rows = sqlx::query(
        r#"SELECT DISTINCT
            c.id::text,
            c.text,
            c.topic,
            c.strength,
            to_char(c.date, 'YYYY-MM-DD') AS date,
            c.status,
            COALESCE((SELECT COUNT(*) FROM commitment_evidence WHERE commitment_id = c.id), 0) AS evidence_count,
            to_char(c.created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
        FROM commitments c
        LEFT JOIN commitment_evidence ce ON ce.commitment_id = c.id
        WHERE c.status = 'broken'
           OR (ce.classification = 'broken' AND ce.confidence >= 0.5)
           OR (ce.classification = 'partial' AND ce.confidence >= 0.5)
        ORDER BY c.created_at DESC"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let commitments: Vec<Commitment> = rows
        .iter()
        .map(|row| Commitment {
            id: row.get::<String, _>("id"),
            text: row.get::<String, _>("text"),
            topic: row.get::<Option<String>, _>("topic"),
            strength: row.get::<Option<String>, _>("strength"),
            date: row.get::<Option<String>, _>("date"),
            status: row.get::<String, _>("status"),
            evidence_count: row.get::<i64, _>("evidence_count"),
            created_at: row.get::<Option<String>, _>("created_at"),
        })
        .collect();

    Ok(commitments)
}
