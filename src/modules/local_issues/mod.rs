//! F17: Hyper-Local Issue Mapping
//!
//! Maps neighborhood-level issues by aggregating local news, public records,
//! and community feedback into actionable geographic intelligence.

use dioxus::prelude::*;

/// A single local issue identified during analysis.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LocalIssue {
    pub title: String,
    pub description: String,
    pub severity: String,
    pub affected_demographics: Vec<String>,
    pub suggested_talking_points: Vec<String>,
}

/// A complete report of local issues for an area.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LocalIssueReport {
    pub id: String,
    pub area_description: String,
    pub issues: Vec<LocalIssue>,
    pub overall_priorities: Vec<String>,
    pub created_at: Option<String>,
}

/// Detailed talking points for a specific local issue.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalkingPoints {
    pub issue_title: String,
    pub area: String,
    pub points: Vec<String>,
}

/// Analyze local issues for a given area.
///
/// Uses the LLM to identify neighborhood-level issues based on area
/// demographics and recent news, returning issues with severity ratings
/// and suggested talking points.
#[server(endpoint = "local-issues/analyze")]
pub async fn analyze_local_issues(
    area_description: String,
    demographics: Option<String>,
    recent_news: Option<String>,
) -> Result<LocalIssueReport, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if area_description.trim().is_empty() {
        return Err(ServerFnError::new("Area description cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let demographics_section = demographics
        .as_deref()
        .filter(|d| !d.trim().is_empty())
        .map(|d| format!("\nDemographics:\n{d}"))
        .unwrap_or_default();

    let news_section = recent_news
        .as_deref()
        .filter(|n| !n.trim().is_empty())
        .map(|n| format!("\nRecent local news:\n{n}"))
        .unwrap_or_default();

    let system_prompt = "\
You are an expert political analyst specializing in hyper-local community issues. \
You identify neighborhood-level concerns and map them to actionable political intelligence.\n\n\
Your response MUST be valid JSON with exactly two keys:\n\
- \"issues\": an array of issue objects, each with:\n\
  - \"title\": short issue name\n\
  - \"description\": 2-3 sentence description of the issue\n\
  - \"severity\": one of \"critical\", \"high\", \"moderate\", or \"low\"\n\
  - \"affected_demographics\": array of demographic groups affected\n\
  - \"suggested_talking_points\": array of 2-4 talking point strings\n\
- \"overall_priorities\": an array of 3-5 strings summarizing the top priorities for the area.\n\n\
Do NOT include any text outside the JSON object."
        .to_string();

    let user_prompt = format!(
        "Analyze the following area and identify the key local issues that a political \
         campaign should address.\n\n\
         Area: {area_description}\
         {demographics_section}\
         {news_section}\n\n\
         Identify 3-6 local issues sorted by severity. For each issue, provide a \
         clear title, description, severity level, affected demographics, and \
         suggested talking points for a candidate."
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: system_prompt,
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let start = std::time::Instant::now();

    let response = llm
        .generate(&messages, None, Some(0.6), Some(3000))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM analysis error: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "local_issues_analyze",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    let parsed: serde_json::Value = serde_json::from_str(&response.content)
        .map_err(|_| ServerFnError::new("Failed to parse local issues response as JSON"))?;

    let issues: Vec<LocalIssue> = parsed
        .get("issues")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    let overall_priorities: Vec<String> = parsed
        .get("overall_priorities")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let report = LocalIssueReport {
        id: uuid::Uuid::new_v4().to_string(),
        area_description,
        issues,
        overall_priorities,
        created_at: Some(now),
    };

    tracing::info!(
        id = %report.id,
        issue_count = report.issues.len(),
        latency_ms = latency_ms,
        "Local issues analyzed"
    );

    Ok(report)
}

/// Generate detailed talking points for a specific local issue.
///
/// Given an issue title and area, produces an in-depth set of talking
/// points a candidate can use when engaging with constituents.
#[server(endpoint = "local-issues/talking-points")]
pub async fn generate_local_talking_points(
    issue_title: String,
    area: String,
) -> Result<TalkingPoints, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if issue_title.trim().is_empty() {
        return Err(ServerFnError::new("Issue title cannot be empty"));
    }
    if area.trim().is_empty() {
        return Err(ServerFnError::new("Area cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let system_prompt = "\
You are an expert political strategist who crafts compelling talking points \
for local community issues. Your talking points are specific, empathetic, and \
action-oriented.\n\n\
Your response MUST be valid JSON with exactly one key:\n\
- \"points\": an array of 5-8 detailed talking point strings. Each should be \
  1-3 sentences and include a mix of empathy, data awareness, and proposed solutions.\n\n\
Do NOT include any text outside the JSON object."
        .to_string();

    let user_prompt = format!(
        "Generate detailed talking points for a political candidate addressing \
         the following local issue.\n\n\
         Issue: {issue_title}\n\
         Area: {area}\n\n\
         The talking points should:\n\
         - Acknowledge the community's concerns\n\
         - Reference local context and impact\n\
         - Propose actionable solutions\n\
         - Be usable in door-to-door canvassing or town halls"
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: system_prompt,
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let start = std::time::Instant::now();

    let response = llm
        .generate(&messages, None, Some(0.6), Some(1500))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM talking points error: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "local_issues_talking_points",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    let parsed: serde_json::Value = serde_json::from_str(&response.content)
        .map_err(|_| ServerFnError::new("Failed to parse talking points response as JSON"))?;

    let points: Vec<String> = parsed
        .get("points")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    tracing::info!(
        issue = %issue_title,
        area = %area,
        point_count = points.len(),
        latency_ms = latency_ms,
        "Local talking points generated"
    );

    Ok(TalkingPoints {
        issue_title,
        area,
        points,
    })
}
