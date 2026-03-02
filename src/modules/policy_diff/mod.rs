//! F18: Policy Diff
//!
//! Compares two policy documents side-by-side, identifying changes in language,
//! intent, and impact using LLM-powered analysis.

use dioxus::prelude::*;

/// A single change detected between two policy documents.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiffChange {
    pub section: String,
    pub change_type: String,
    pub old_text: String,
    pub new_text: String,
    pub significance: String,
}

/// The result of comparing two policy documents.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PolicyDiff {
    pub id: String,
    pub doc_a_title: String,
    pub doc_b_title: String,
    pub changes: Vec<DiffChange>,
    pub summary: String,
    pub created_at: String,
}

const DIFF_SYSTEM_PROMPT: &str = "\
You are a policy analysis expert. Given two policy documents, compare them and identify all meaningful differences.

Return your analysis as a JSON object with these fields:
- changes: An array of objects, each with:
  - section: The section or topic area of the change
  - change_type: One of \"added\", \"removed\", \"modified\", \"reworded\"
  - old_text: The relevant text from Document A (empty string if added)
  - new_text: The relevant text from Document B (empty string if removed)
  - significance: One of \"high\", \"medium\", \"low\"
- summary: A 2-4 sentence overall summary of the differences

Return ONLY the JSON object, no other text.";

/// Compare two policy documents using LLM analysis.
#[server(endpoint = "policy-diff/diff-policies")]
pub async fn diff_policies(
    doc_a_text: String,
    doc_a_title: String,
    doc_b_text: String,
    doc_b_title: String,
) -> Result<PolicyDiff, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if doc_a_text.trim().is_empty() || doc_b_text.trim().is_empty() {
        return Err(ServerFnError::new("Both documents must have content"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let user_prompt = format!(
        "Compare these two policy documents:\n\n\
         --- Document A: {doc_a_title} ---\n{doc_a_text}\n\n\
         --- Document B: {doc_b_title} ---\n{doc_b_text}"
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: DIFF_SYSTEM_PROMPT.to_string(),
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let start = std::time::Instant::now();

    let response = llm
        .generate(&messages, None, Some(0.3), Some(3000))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "policy_diff",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    let content = response.content.trim();
    let json_str = if content.starts_with("```") {
        let stripped = content
            .trim_start_matches("```json")
            .trim_start_matches("```");
        stripped
            .rfind("```")
            .map(|end| &stripped[..end])
            .unwrap_or(stripped)
            .trim()
    } else {
        content
    };

    #[derive(serde::Deserialize)]
    struct DiffResponse {
        changes: Vec<DiffChange>,
        summary: String,
    }

    let parsed: DiffResponse = serde_json::from_str(json_str).map_err(|e| {
        tracing::warn!(raw = %content, "Failed to parse diff response: {e}");
        ServerFnError::new(format!("Failed to parse diff analysis: {e}"))
    })?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tracing::info!(
        changes = parsed.changes.len(),
        latency_ms = latency_ms,
        "Policy diff completed"
    );

    Ok(PolicyDiff {
        id: uuid::Uuid::new_v4().to_string(),
        doc_a_title,
        doc_b_title,
        changes: parsed.changes,
        summary: parsed.summary,
        created_at: now,
    })
}
