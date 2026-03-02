//! F12: Candidate Briefing Generator
//!
//! Produces concise, structured briefings for candidates covering relevant news,
//! schedule context, talking points, and emerging issues. Each briefing is organized
//! into prioritized sections for quick consumption before events.

use dioxus::prelude::*;

/// A complete candidate briefing document with prioritized sections.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Briefing {
    pub id: String,
    pub title: String,
    pub sections: Vec<BriefingSection>,
    pub created_at: Option<String>,
}

/// A single section within a briefing, with a priority indicator.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BriefingSection {
    pub heading: String,
    pub content: String,
    pub priority: String, // "high", "medium", "low"
}

/// Generate a structured briefing for a candidate based on event context.
///
/// Uses the LLM to produce a multi-section briefing document tailored to the
/// event type, topics, audience, and additional context provided.
#[server(endpoint = "briefing/generate")]
pub async fn generate_briefing(
    event_type: String,
    topics: Vec<String>,
    audience: String,
    context: String,
) -> Result<Briefing, ServerFnError> {
    use crate::infrastructure::{LlmClient, ServerState};
    use crate::infrastructure::llm::LlmMessage;
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    if event_type.trim().is_empty() {
        return Err(ServerFnError::new("Event type is required"));
    }

    let topics_str = if topics.is_empty() {
        "general campaign topics".to_string()
    } else {
        topics.join(", ")
    };

    let audience_str = if audience.trim().is_empty() {
        "general audience".to_string()
    } else {
        audience.clone()
    };

    let context_str = if context.trim().is_empty() {
        "No additional context provided.".to_string()
    } else {
        context.clone()
    };

    let system_prompt = format!(
        "You are an expert political strategist and briefing writer. Your job is to produce \
concise, actionable briefing documents for political candidates before their events.\n\n\
You MUST respond with valid JSON in exactly this format:\n\
{{\n\
  \"title\": \"Briefing title\",\n\
  \"sections\": [\n\
    {{\n\
      \"heading\": \"Section heading\",\n\
      \"content\": \"Section content with key points, talking points, and advice.\",\n\
      \"priority\": \"high\"\n\
    }}\n\
  ]\n\
}}\n\n\
Priority levels: \"high\" (must read), \"medium\" (should read), \"low\" (nice to know).\n\n\
Generate 5-8 sections covering:\n\
- Key talking points for the event\n\
- Audience analysis and what they care about\n\
- Potential tough questions and suggested responses\n\
- Recent news or developments relevant to the topics\n\
- Do's and don'ts for this type of event\n\
- Background data or statistics to reference\n\
- Suggested opening and closing remarks\n\n\
Each section should be 2-4 paragraphs. Be specific and actionable."
    );

    let user_prompt = format!(
        "Generate a candidate briefing for the following event:\n\n\
Event Type: {event_type}\n\
Topics: {topics_str}\n\
Audience: {audience_str}\n\
Additional Context: {context_str}"
    );

    let llm_messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: system_prompt,
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let start = std::time::Instant::now();

    let response = llm
        .generate(&llm_messages, None, Some(0.5), Some(2500))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM error: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "candidate_briefing",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    // Parse JSON response
    let content = response.content.trim();
    let json_str = if let Some(start_idx) = content.find('{') {
        if let Some(end_idx) = content.rfind('}') {
            &content[start_idx..=end_idx]
        } else {
            content
        }
    } else {
        content
    };

    #[derive(serde::Deserialize)]
    struct BriefingResponse {
        title: String,
        sections: Vec<BriefingSection>,
    }

    let parsed: BriefingResponse = serde_json::from_str(json_str).map_err(|e| {
        ServerFnError::new(format!(
            "Failed to parse briefing JSON: {e}. Raw response: {content}"
        ))
    })?;

    let briefing_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tracing::info!(
        briefing_id = %briefing_id,
        event_type = %event_type,
        sections = parsed.sections.len(),
        latency_ms = latency_ms,
        "Candidate briefing generated"
    );

    Ok(Briefing {
        id: briefing_id,
        title: parsed.title,
        sections: parsed.sections,
        created_at: Some(now),
    })
}

/// List saved briefings.
///
/// Currently returns an empty list as database persistence has not been
/// implemented yet. This endpoint exists to establish the API contract.
#[server(endpoint = "briefing/list")]
pub async fn list_briefings() -> Result<Vec<Briefing>, ServerFnError> {
    Ok(Vec::new())
}
