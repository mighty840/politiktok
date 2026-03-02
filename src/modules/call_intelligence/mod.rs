//! F13: Constituent Call Intelligence
//!
//! Analyzes constituent call data to extract trends, common concerns,
//! and sentiment, providing actionable intelligence for representatives.

use dioxus::prelude::*;

/// Analysis of a single constituent call transcript.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CallAnalysis {
    pub id: String,
    pub transcript: String,
    pub summary: String,
    pub sentiment: String,
    pub key_issues: Vec<String>,
    pub action_items: Vec<String>,
    pub caller_satisfaction: f64,
    pub created_at: Option<String>,
}

/// Analyze a constituent call transcript using AI.
///
/// Extracts a summary, sentiment classification, key issues raised,
/// action items, and an estimated caller satisfaction score.
#[server(endpoint = "call-intel/analyze")]
pub async fn analyze_call(transcript: String) -> Result<CallAnalysis, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if transcript.trim().is_empty() {
        return Err(ServerFnError::new("Transcript cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let system_prompt = "\
You are an expert political constituent services analyst. Analyze the following \
call transcript between a constituent and a representative's office.\n\n\
Return your analysis as a JSON object with exactly these fields:\n\
- \"summary\": A concise 2-3 sentence summary of the call\n\
- \"sentiment\": One of \"positive\", \"negative\", \"neutral\", or \"mixed\"\n\
- \"key_issues\": An array of the main issues/topics raised (strings)\n\
- \"action_items\": An array of follow-up actions needed (strings)\n\
- \"caller_satisfaction\": A number from 0.0 to 1.0 estimating caller satisfaction\n\n\
Return ONLY the JSON object, no markdown fences or extra text.";

    let user_prompt = format!("Analyze this constituent call transcript:\n\n{transcript}");

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let start = std::time::Instant::now();

    let response = llm
        .generate(&messages, None, Some(0.3), Some(1024))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM error: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "call_intelligence",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    // Parse the LLM JSON response
    let raw = response.content.trim();
    // Strip markdown fences if present
    let json_str = if raw.starts_with("```") {
        raw.trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
    } else {
        raw
    };

    let parsed: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| ServerFnError::new(format!("Failed to parse LLM response as JSON: {e}")))?;

    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let summary = parsed["summary"]
        .as_str()
        .unwrap_or("No summary available")
        .to_string();
    let sentiment = parsed["sentiment"]
        .as_str()
        .unwrap_or("neutral")
        .to_string();
    let key_issues: Vec<String> = parsed["key_issues"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let action_items = parsed["action_items"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let caller_satisfaction = parsed["caller_satisfaction"].as_f64().unwrap_or(0.5);

    tracing::info!(
        id = %id,
        sentiment = %sentiment,
        issues_count = key_issues.len(),
        latency_ms = latency_ms,
        "Call transcript analyzed"
    );

    Ok(CallAnalysis {
        id,
        transcript,
        summary,
        sentiment,
        key_issues,
        action_items,
        caller_satisfaction,
        created_at: Some(now),
    })
}

/// Get aggregated call trends (stub).
///
/// Returns an empty vec for now. Future implementation will aggregate
/// call analyses over time to surface recurring themes and trends.
#[server(endpoint = "call-intel/trends")]
pub async fn get_call_trends() -> Result<Vec<CallAnalysis>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;

    let _state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    Ok(Vec::new())
}
