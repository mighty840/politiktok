#![cfg_attr(not(feature = "server"), allow(dead_code))]
//! F22: Disinformation Early Warning
//!
//! Detects emerging disinformation campaigns targeting candidates or
//! policies, enabling rapid response before narratives take hold.

use dioxus::prelude::*;

/// A single disinformation indicator found in content.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DisinfoIndicator {
    pub indicator_type: String,
    pub description: String,
    pub confidence: f64,
}

/// Complete disinformation analysis result.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DisinfoAnalysis {
    pub id: String,
    pub content: String,
    pub risk_level: String,
    pub indicators: Vec<DisinfoIndicator>,
    pub recommended_response: String,
    pub created_at: String,
}

/// Generated counter-messaging response.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CounterResponse {
    pub id: String,
    pub target_audience: String,
    pub messaging: String,
    pub key_points: Vec<String>,
    pub created_at: String,
}

const DISINFO_ANALYSIS_SYSTEM_PROMPT: &str = "\
You are a disinformation detection expert. Analyze the provided content for indicators of \
disinformation, misinformation, or coordinated inauthentic behavior.

Look for these types of indicators:
- Emotional manipulation and fear-mongering
- False or misleading statistics
- Out-of-context quotes or images
- Logical fallacies (straw man, ad hominem, false dichotomy, etc.)
- Conspiracy theory language patterns
- Bot-like or coordinated posting patterns
- Source credibility issues
- Cherry-picked or outdated information

Return your analysis as a JSON object with these fields:
- risk_level: One of \"high\", \"medium\", \"low\"
- indicators: An array of objects, each with:
  - indicator_type: The type of indicator (e.g., \"emotional_manipulation\", \"false_statistics\", \
\"logical_fallacy\", \"source_credibility\", \"coordination_signals\", \"cherry_picking\")
  - description: Specific explanation of the indicator found
  - confidence: A confidence score between 0.0 and 1.0
- recommended_response: A strategic recommendation for how to respond

Return ONLY the JSON object, no other text.";

const COUNTER_MESSAGING_SYSTEM_PROMPT: &str = "\
You are a strategic communications expert specializing in countering disinformation. \
Given disinformation content and a target audience, generate effective counter-messaging.

Your counter-messaging should:
- Be factual and evidence-based
- Avoid repeating or amplifying the disinformation
- Use clear, simple language appropriate for the audience
- Include specific facts that debunk false claims
- Provide a positive alternative narrative
- Be respectful and non-condescending

Return your response as a JSON object with these fields:
- messaging: The full counter-messaging text (2-3 paragraphs)
- key_points: An array of 3-5 bullet-point talking points

Return ONLY the JSON object, no other text.";

/// Analyze content for disinformation indicators.
#[server(endpoint = "disinfo/analyze-disinfo")]
pub async fn analyze_disinfo(
    content: String,
    source_context: String,
) -> Result<DisinfoAnalysis, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if content.trim().is_empty() {
        return Err(ServerFnError::new("Content cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let context_section = if source_context.trim().is_empty() {
        String::new()
    } else {
        format!("\n\nSource context:\n{source_context}")
    };

    let user_prompt = format!(
        "Analyze the following content for disinformation indicators:\n\n{content}{context_section}"
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: DISINFO_ANALYSIS_SYSTEM_PROMPT.to_string(),
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let start = std::time::Instant::now();

    let response = llm
        .generate(&messages, None, Some(0.2), Some(2500))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "disinfo_warning",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    let raw_content = response.content.trim();
    let json_str = if raw_content.starts_with("```") {
        let stripped = raw_content
            .trim_start_matches("```json")
            .trim_start_matches("```");
        stripped
            .rfind("```")
            .map(|end| &stripped[..end])
            .unwrap_or(stripped)
            .trim()
    } else {
        raw_content
    };

    #[derive(serde::Deserialize)]
    struct DisinfoResponse {
        risk_level: String,
        indicators: Vec<DisinfoIndicator>,
        recommended_response: String,
    }

    let parsed: DisinfoResponse = serde_json::from_str(json_str).map_err(|e| {
        tracing::warn!(raw = %raw_content, "Failed to parse disinfo response: {e}");
        ServerFnError::new(format!("Failed to parse disinformation analysis: {e}"))
    })?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tracing::info!(
        risk_level = %parsed.risk_level,
        indicators = parsed.indicators.len(),
        latency_ms = latency_ms,
        "Disinformation analysis completed"
    );

    Ok(DisinfoAnalysis {
        id: uuid::Uuid::new_v4().to_string(),
        content,
        risk_level: parsed.risk_level,
        indicators: parsed.indicators,
        recommended_response: parsed.recommended_response,
        created_at: now,
    })
}

/// Generate counter-messaging for disinformation content.
#[server(endpoint = "disinfo/generate-response")]
pub async fn generate_response(
    disinfo_content: String,
    target_audience: String,
) -> Result<CounterResponse, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if disinfo_content.trim().is_empty() {
        return Err(ServerFnError::new("Disinformation content cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let audience_label = if target_audience.trim().is_empty() {
        "general public".to_string()
    } else {
        target_audience.clone()
    };

    let user_prompt = format!(
        "Generate counter-messaging for the following disinformation content.\n\n\
         Target audience: {audience_label}\n\n\
         Disinformation content:\n{disinfo_content}"
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: COUNTER_MESSAGING_SYSTEM_PROMPT.to_string(),
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let start = std::time::Instant::now();

    let response = llm
        .generate(&messages, None, Some(0.5), Some(2000))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "disinfo_warning",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    let raw_content = response.content.trim();
    let json_str = if raw_content.starts_with("```") {
        let stripped = raw_content
            .trim_start_matches("```json")
            .trim_start_matches("```");
        stripped
            .rfind("```")
            .map(|end| &stripped[..end])
            .unwrap_or(stripped)
            .trim()
    } else {
        raw_content
    };

    #[derive(serde::Deserialize)]
    struct CounterMsgResponse {
        messaging: String,
        key_points: Vec<String>,
    }

    let parsed: CounterMsgResponse = serde_json::from_str(json_str).map_err(|e| {
        tracing::warn!(raw = %raw_content, "Failed to parse counter-messaging response: {e}");
        ServerFnError::new(format!("Failed to parse counter-messaging: {e}"))
    })?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tracing::info!(
        audience = %audience_label,
        key_points = parsed.key_points.len(),
        latency_ms = latency_ms,
        "Counter-messaging generated"
    );

    Ok(CounterResponse {
        id: uuid::Uuid::new_v4().to_string(),
        target_audience: audience_label,
        messaging: parsed.messaging,
        key_points: parsed.key_points,
        created_at: now,
    })
}
