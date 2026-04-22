#![cfg_attr(not(feature = "server"), allow(dead_code))]
//! F20: Regulatory Plain-Language Monitor
//!
//! Monitors regulatory changes and translates dense legal and regulatory
//! text into plain-language summaries for campaign staff and voters.

use dioxus::prelude::*;

/// A single regulatory update entry.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RegulatoryUpdate {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub effective_date: Option<String>,
    pub impact_assessment: Option<String>,
    pub urgency: String,
    pub action_required: Option<String>,
    pub source_name: Option<String>,
    pub created_at: String,
}

/// A brief covering multiple regulatory updates.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RegulatoryBrief {
    pub id: String,
    pub updates: Vec<RegulatoryUpdate>,
    pub analysis: String,
    pub created_at: String,
}

const IMPACT_SYSTEM_PROMPT: &str = "\
You are a regulatory analyst. Given a regulatory update and context, provide a comprehensive \
impact assessment.

Return your analysis as a JSON object with these fields:
- impact_assessment: A detailed assessment of who is affected and how (2-4 paragraphs)
- urgency: One of \"urgent\", \"important\", \"routine\"
- action_required: A specific recommended action or response
- summary: A 1-2 sentence plain-language summary

Return ONLY the JSON object, no other text.";

const BRIEF_SYSTEM_PROMPT: &str = "\
You are a policy briefing writer. Given a set of regulatory updates, generate a concise \
executive brief that synthesizes the information into actionable intelligence.

Your brief should:
- Prioritize by urgency and impact
- Identify common themes across updates
- Highlight items requiring immediate action
- Provide strategic recommendations

Write the brief in clear, professional prose using markdown formatting. \
Keep it concise but thorough.";

/// List regulatory updates, optionally filtered by urgency.
#[server(endpoint = "regulatory/list-updates")]
pub async fn list_regulatory_updates(
    urgency_filter: Option<String>,
) -> Result<Vec<RegulatoryUpdate>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    let rows = if let Some(ref urgency) = urgency_filter {
        if urgency.is_empty() || urgency == "all" {
            sqlx::query(
                r#"SELECT
                    id::text, title, summary, effective_date,
                    impact_assessment, urgency, action_required,
                    source_name,
                    to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
                FROM regulatory_updates
                ORDER BY created_at DESC
                LIMIT 50"#,
            )
            .fetch_all(pool)
            .await
        } else {
            sqlx::query(
                r#"SELECT
                    id::text, title, summary, effective_date,
                    impact_assessment, urgency, action_required,
                    source_name,
                    to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
                FROM regulatory_updates
                WHERE urgency = $1
                ORDER BY created_at DESC
                LIMIT 50"#,
            )
            .bind(urgency)
            .fetch_all(pool)
            .await
        }
    } else {
        sqlx::query(
            r#"SELECT
                id::text, title, summary, effective_date,
                impact_assessment, urgency, action_required,
                source_name,
                to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
            FROM regulatory_updates
            ORDER BY created_at DESC
            LIMIT 50"#,
        )
        .fetch_all(pool)
        .await
    }
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let updates: Vec<RegulatoryUpdate> = rows
        .iter()
        .map(|row| RegulatoryUpdate {
            id: row.get::<String, _>("id"),
            title: row.get::<String, _>("title"),
            summary: row.get::<String, _>("summary"),
            effective_date: row.get::<Option<String>, _>("effective_date"),
            impact_assessment: row.get::<Option<String>, _>("impact_assessment"),
            urgency: row.get::<String, _>("urgency"),
            action_required: row.get::<Option<String>, _>("action_required"),
            source_name: row.get::<Option<String>, _>("source_name"),
            created_at: row
                .get::<Option<String>, _>("created_at")
                .unwrap_or_default(),
        })
        .collect();

    Ok(updates)
}

/// Analyze the impact of a regulatory update using LLM.
#[server(endpoint = "regulatory/analyze-impact")]
pub async fn analyze_regulatory_impact(
    update_text: String,
    context: String,
) -> Result<RegulatoryUpdate, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if update_text.trim().is_empty() {
        return Err(ServerFnError::new("Regulatory update text cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let context_section = if context.trim().is_empty() {
        String::new()
    } else {
        format!("\n\nAdditional context:\n{context}")
    };

    let user_prompt =
        format!("Analyze the following regulatory update:\n\n{update_text}{context_section}");

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: IMPACT_SYSTEM_PROMPT.to_string(),
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
        "regulatory_monitor",
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
    struct ImpactResponse {
        impact_assessment: String,
        urgency: String,
        action_required: String,
        summary: String,
    }

    let parsed: ImpactResponse = serde_json::from_str(json_str).map_err(|e| {
        tracing::warn!(raw = %content, "Failed to parse impact response: {e}");
        ServerFnError::new(format!("Failed to parse impact analysis: {e}"))
    })?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tracing::info!(
        urgency = %parsed.urgency,
        latency_ms = latency_ms,
        "Regulatory impact analysis completed"
    );

    Ok(RegulatoryUpdate {
        id: uuid::Uuid::new_v4().to_string(),
        title: update_text.chars().take(80).collect::<String>(),
        summary: parsed.summary,
        effective_date: None,
        impact_assessment: Some(parsed.impact_assessment),
        urgency: parsed.urgency,
        action_required: Some(parsed.action_required),
        source_name: None,
        created_at: now,
    })
}

/// Generate a regulatory brief from a set of update IDs.
#[server(endpoint = "regulatory/generate-brief")]
pub async fn generate_regulatory_brief(
    update_ids: Vec<String>,
) -> Result<RegulatoryBrief, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if update_ids.is_empty() {
        return Err(ServerFnError::new("At least one update ID is required"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    // Fetch updates from DB
    let placeholders: Vec<String> = update_ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("${}", i + 1))
        .collect();
    let placeholder_str = placeholders.join(", ");

    let query_str = format!(
        r#"SELECT
            id::text, title, summary, effective_date,
            impact_assessment, urgency, action_required,
            source_name,
            to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
        FROM regulatory_updates
        WHERE id::text IN ({placeholder_str})
        ORDER BY created_at DESC"#,
    );

    let mut query = sqlx::query(&query_str);
    for id in &update_ids {
        query = query.bind(id);
    }

    let rows = query
        .fetch_all(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let updates: Vec<RegulatoryUpdate> = rows
        .iter()
        .map(|row| RegulatoryUpdate {
            id: row.get::<String, _>("id"),
            title: row.get::<String, _>("title"),
            summary: row.get::<String, _>("summary"),
            effective_date: row.get::<Option<String>, _>("effective_date"),
            impact_assessment: row.get::<Option<String>, _>("impact_assessment"),
            urgency: row.get::<String, _>("urgency"),
            action_required: row.get::<Option<String>, _>("action_required"),
            source_name: row.get::<Option<String>, _>("source_name"),
            created_at: row
                .get::<Option<String>, _>("created_at")
                .unwrap_or_default(),
        })
        .collect();

    if updates.is_empty() {
        return Err(ServerFnError::new("No updates found for the provided IDs"));
    }

    // Build prompt from updates
    let updates_text = updates
        .iter()
        .enumerate()
        .map(|(i, u)| {
            format!(
                "Update #{}: {}\nUrgency: {}\nSummary: {}\nImpact: {}\nAction: {}",
                i + 1,
                u.title,
                u.urgency,
                u.summary,
                u.impact_assessment.as_deref().unwrap_or("N/A"),
                u.action_required.as_deref().unwrap_or("N/A"),
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n---\n\n");

    let user_prompt = format!(
        "Generate an executive brief for the following {} regulatory updates:\n\n{updates_text}",
        updates.len()
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: BRIEF_SYSTEM_PROMPT.to_string(),
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let start = std::time::Instant::now();

    let response = llm
        .generate(&messages, None, Some(0.4), Some(2500))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "regulatory_monitor",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tracing::info!(
        update_count = updates.len(),
        latency_ms = latency_ms,
        "Regulatory brief generated"
    );

    Ok(RegulatoryBrief {
        id: uuid::Uuid::new_v4().to_string(),
        updates,
        analysis: response.content,
        created_at: now,
    })
}
