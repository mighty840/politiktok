//! F24: Meeting Summarizer & Action Tracker
//!
//! Summarizes political meetings, committee sessions, and strategy calls,
//! extracting action items and tracking their completion.
//! Uses LLM to extract structured summaries from meeting transcripts.

use dioxus::prelude::*;

/// An action item extracted from a meeting.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ActionItem {
    pub description: String,
    pub assignee: String,
    pub deadline: String,
    pub status: String,
}

/// A complete meeting summary with extracted insights.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MeetingSummary {
    pub id: String,
    pub title: String,
    pub transcript: String,
    pub summary: String,
    pub key_decisions: Vec<String>,
    pub action_items: Vec<ActionItem>,
    pub attendees: Vec<String>,
    pub created_at: String,
}

/// System prompt for meeting summarization.
const SYSTEM_PROMPT: &str = "\
You are an expert meeting analyst for a political campaign. Your job is to analyze \
meeting transcripts and produce structured, actionable summaries.

You MUST respond with valid JSON matching this exact schema:
{
  \"summary\": \"A concise 2-4 paragraph summary of the meeting covering main topics discussed, \
overall outcomes, and key themes\",
  \"key_decisions\": [
    \"Decision 1 that was made during the meeting\",
    \"Decision 2 that was made during the meeting\"
  ],
  \"action_items\": [
    {
      \"description\": \"What needs to be done\",
      \"assignee\": \"Person responsible (use name from attendees if possible, or 'Unassigned')\",
      \"deadline\": \"Deadline if mentioned, otherwise 'TBD'\",
      \"status\": \"pending\"
    }
  ]
}

Guidelines:
- Summarize the main discussion points, not every detail
- Identify all concrete decisions that were made
- Extract every action item mentioned, even implied ones
- Assign action items to specific attendees when possible
- Include deadlines when explicitly or implicitly mentioned
- Keep the summary professional and objective
- If the transcript is unclear, note any ambiguities";

/// Summarize a meeting transcript using LLM analysis.
///
/// Extracts a concise summary, key decisions, and action items from
/// the provided transcript text.
#[server(endpoint = "meeting-summarizer/summarize")]
pub async fn summarize_meeting(
    title: String,
    transcript: String,
    attendees: Vec<String>,
) -> Result<MeetingSummary, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if title.trim().is_empty() {
        return Err(ServerFnError::new("Meeting title is required"));
    }
    if transcript.trim().is_empty() {
        return Err(ServerFnError::new("Meeting transcript is required"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let attendee_list = if attendees.is_empty() {
        "Not specified".to_string()
    } else {
        attendees.join(", ")
    };

    let user_prompt = format!(
        "Meeting Title: {title}\n\
        Attendees: {attendee_list}\n\n\
        Transcript:\n{transcript}\n\n\
        Please analyze this meeting transcript and provide a structured summary."
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

    let start = std::time::Instant::now();

    let response = llm
        .generate(&messages, None, Some(0.3), Some(2048))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM error: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "meeting_summarizer",
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

    let summary = parsed
        .get("summary")
        .and_then(|v| v.as_str())
        .unwrap_or("No summary generated")
        .to_string();

    let key_decisions: Vec<String> = parsed
        .get("key_decisions")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let action_items: Vec<ActionItem> = parsed
        .get("action_items")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|item| ActionItem {
                    description: item["description"]
                        .as_str()
                        .unwrap_or("No description")
                        .to_string(),
                    assignee: item["assignee"]
                        .as_str()
                        .unwrap_or("Unassigned")
                        .to_string(),
                    deadline: item["deadline"].as_str().unwrap_or("TBD").to_string(),
                    status: item["status"].as_str().unwrap_or("pending").to_string(),
                })
                .collect()
        })
        .unwrap_or_default();

    let meeting_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tracing::info!(
        meeting_id = %meeting_id,
        title = %title,
        decisions = key_decisions.len(),
        action_items = action_items.len(),
        latency_ms = latency_ms,
        "Meeting summarized"
    );

    Ok(MeetingSummary {
        id: meeting_id,
        title,
        transcript,
        summary,
        key_decisions,
        action_items,
        attendees,
        created_at: now,
    })
}

/// List all meeting summaries.
///
/// Currently returns an empty list as database persistence for meeting summaries
/// has not been implemented yet.
#[server(endpoint = "meeting-summarizer/list-summaries")]
pub async fn list_meeting_summaries() -> Result<Vec<MeetingSummary>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;

    let _state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    Ok(Vec::new())
}
