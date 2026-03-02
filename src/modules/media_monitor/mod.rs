//! F21: Media Bias & Coverage Monitor
//!
//! Tracks media coverage across outlets, detecting bias patterns and
//! coverage gaps to inform media strategy and rapid response.

use dioxus::prelude::*;

/// Analysis result for a single media article.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MediaAnalysis {
    pub id: String,
    pub article_text: String,
    pub source: String,
    pub bias_assessment: String,
    pub key_claims: Vec<String>,
    pub fact_check_notes: Vec<String>,
    pub coverage_tone: String,
    pub created_at: String,
}

/// Comparison of coverage across multiple sources on the same topic.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CoverageComparison {
    pub topic: String,
    pub source_analyses: Vec<SourceCoverageNote>,
    pub overall_assessment: String,
    pub bias_spectrum: String,
    pub created_at: String,
}

/// Per-source coverage note within a comparison.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SourceCoverageNote {
    pub source: String,
    pub tone: String,
    pub framing: String,
    pub key_omissions: Vec<String>,
}

const MEDIA_ANALYSIS_SYSTEM_PROMPT: &str = "\
You are a media analysis expert specializing in detecting bias in political news coverage. \
Given an article and its source, analyze it for bias, identify key claims, and assess the tone.

Return your analysis as a JSON object with these fields:
- bias_assessment: A description of detected bias (e.g., \"left-leaning\", \"right-leaning\", \
\"centrist\", \"sensationalist\") with explanation
- key_claims: An array of the main factual claims made in the article
- fact_check_notes: An array of notes about claims that should be verified, with context
- coverage_tone: One of \"positive\", \"negative\", \"neutral\", \"mixed\", \"alarmist\", \"dismissive\"

Return ONLY the JSON object, no other text.";

const COVERAGE_COMPARISON_SYSTEM_PROMPT: &str = "\
You are a media analysis expert. Given multiple articles from different sources covering the \
same topic, compare how each source covers it and identify differences in framing, tone, and \
emphasis.

Return your analysis as a JSON object with these fields:
- source_analyses: An array of objects, each with:
  - source: The source name
  - tone: The overall tone of coverage
  - framing: How the source frames the topic
  - key_omissions: Array of notable facts or perspectives the source omits
- overall_assessment: A 2-3 sentence summary of the coverage landscape
- bias_spectrum: A description of where coverage falls on the political spectrum

Return ONLY the JSON object, no other text.";

/// Analyze a media article for bias, claims, and tone.
#[server(endpoint = "media-monitor/analyze-media")]
pub async fn analyze_media(
    article_text: String,
    source_name: String,
) -> Result<MediaAnalysis, ServerFnError> {
    use crate::infrastructure::{LlmClient, ServerState};
    use crate::infrastructure::llm::LlmMessage;
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if article_text.trim().is_empty() {
        return Err(ServerFnError::new("Article text cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let source_label = if source_name.trim().is_empty() {
        "Unknown Source".to_string()
    } else {
        source_name.clone()
    };

    let user_prompt = format!(
        "Analyze the following article from {source_label}:\n\n{article_text}"
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: MEDIA_ANALYSIS_SYSTEM_PROMPT.to_string(),
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let start = std::time::Instant::now();

    let response = llm
        .generate(&messages, None, Some(0.3), Some(2000))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "media_monitor",
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
    struct MediaResponse {
        bias_assessment: String,
        key_claims: Vec<String>,
        fact_check_notes: Vec<String>,
        coverage_tone: String,
    }

    let parsed: MediaResponse = serde_json::from_str(json_str).map_err(|e| {
        tracing::warn!(raw = %content, "Failed to parse media analysis response: {e}");
        ServerFnError::new(format!("Failed to parse media analysis: {e}"))
    })?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tracing::info!(
        source = %source_label,
        tone = %parsed.coverage_tone,
        claims = parsed.key_claims.len(),
        latency_ms = latency_ms,
        "Media analysis completed"
    );

    Ok(MediaAnalysis {
        id: uuid::Uuid::new_v4().to_string(),
        article_text,
        source: source_label,
        bias_assessment: parsed.bias_assessment,
        key_claims: parsed.key_claims,
        fact_check_notes: parsed.fact_check_notes,
        coverage_tone: parsed.coverage_tone,
        created_at: now,
    })
}

/// Compare coverage of a topic across multiple sources.
#[server(endpoint = "media-monitor/compare-coverage")]
pub async fn compare_coverage(
    topic: String,
    articles: Vec<(String, String)>,
) -> Result<CoverageComparison, ServerFnError> {
    use crate::infrastructure::{LlmClient, ServerState};
    use crate::infrastructure::llm::LlmMessage;
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if topic.trim().is_empty() {
        return Err(ServerFnError::new("Topic cannot be empty"));
    }
    if articles.len() < 2 {
        return Err(ServerFnError::new(
            "At least two articles are required for comparison",
        ));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let articles_text = articles
        .iter()
        .enumerate()
        .map(|(i, (source, text))| {
            format!("--- Source #{}: {} ---\n{}", i + 1, source, text)
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    let user_prompt = format!(
        "Compare how these sources cover the topic \"{topic}\":\n\n{articles_text}"
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: COVERAGE_COMPARISON_SYSTEM_PROMPT.to_string(),
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
        "media_monitor",
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
    struct ComparisonResponse {
        source_analyses: Vec<SourceCoverageNote>,
        overall_assessment: String,
        bias_spectrum: String,
    }

    let parsed: ComparisonResponse = serde_json::from_str(json_str).map_err(|e| {
        tracing::warn!(raw = %content, "Failed to parse coverage comparison: {e}");
        ServerFnError::new(format!("Failed to parse coverage comparison: {e}"))
    })?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tracing::info!(
        topic = %topic,
        sources = parsed.source_analyses.len(),
        latency_ms = latency_ms,
        "Coverage comparison completed"
    );

    Ok(CoverageComparison {
        topic,
        source_analyses: parsed.source_analyses,
        overall_assessment: parsed.overall_assessment,
        bias_spectrum: parsed.bias_spectrum,
        created_at: now,
    })
}
