#![cfg_attr(not(feature = "server"), allow(dead_code))]
//! F05: Opposition Research & Debate Briefing
//!
//! Compiles and analyzes opposition records, statements, and positions
//! to generate comprehensive debate preparation briefings.

use dioxus::prelude::*;

use crate::models::candidate::{Contradiction, Opponent};

/// System prompt for generating debate briefings.
const BRIEFING_SYSTEM_PROMPT: &str = "\
You are a political opposition research analyst. Given an opponent's profile and policy positions, \
generate a structured debate briefing. Be factual, analytical, and thorough.

Your briefing MUST include these sections with markdown headers:
## Background
## Key Vulnerabilities
## Policy Weaknesses
## Recommended Attack Lines
## Defense Preparation

Keep the tone professional and analytical. Base your analysis only on the provided information.";

/// System prompt for detecting contradictions.
const CONTRADICTION_SYSTEM_PROMPT: &str = "\
You are a political research analyst specializing in finding contradictions in public statements \
and policy positions. Given an opponent's policy positions, identify any contradictions, \
flip-flops, or inconsistencies.

Return your findings as a JSON array of objects with these fields:
- statement_a: The first contradictory statement or position
- statement_b: The second contradictory statement or position
- topic: The policy topic area
- confidence: A confidence score between 0.0 and 1.0
- source_a: Source or context for statement A (or null)
- source_b: Source or context for statement B (or null)

If no contradictions are found, return an empty array: []
Return ONLY the JSON array, no other text.";

/// Create a new opponent profile.
#[server(endpoint = "opposition/create-opponent")]
pub async fn create_opponent(
    name: String,
    party: String,
    district: String,
    bio: String,
    policy_positions: String,
) -> Result<Opponent, ServerFnError> {
    use crate::infrastructure::{require_user, ServerState};
    use dioxus::fullstack::FullstackContext;

    let _user = require_user()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();
    let id = uuid::Uuid::new_v4().to_string();

    // Parse policy_positions as JSON, falling back to a JSON string
    let positions_json: serde_json::Value = serde_json::from_str(&policy_positions)
        .unwrap_or_else(|_| serde_json::Value::String(policy_positions.clone()));

    sqlx::query(
        r#"INSERT INTO candidates (id, name, role, district, bio, policy_positions)
           VALUES ($1::uuid, $2, 'opponent', $3, $4, $5)"#,
    )
    .bind(&id)
    .bind(&name)
    .bind(&district)
    .bind(&bio)
    .bind(&positions_json)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    tracing::info!(id = %id, name = %name, "Opponent created");

    Ok(Opponent {
        id,
        name,
        party: Some(party),
        district: Some(district),
        bio: Some(bio),
        policy_positions: positions_json,
        contradictions: Vec::new(),
        created_at: None,
    })
}

/// List all opponents.
#[server(endpoint = "opposition/list-opponents")]
pub async fn list_opponents() -> Result<Vec<Opponent>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    let rows = sqlx::query(
        r#"SELECT
            id::text,
            name,
            district,
            bio,
            COALESCE(policy_positions, '{}'::jsonb) AS policy_positions,
            to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
        FROM candidates
        WHERE role = 'opponent'
        ORDER BY created_at DESC"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let opponents: Vec<Opponent> = rows
        .iter()
        .map(|row| {
            let positions: serde_json::Value = row.get("policy_positions");
            // Extract party from policy_positions metadata if stored there, otherwise None
            let party = positions
                .get("party")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            Opponent {
                id: row.get::<String, _>("id"),
                name: row.get::<String, _>("name"),
                party,
                district: row.get::<Option<String>, _>("district"),
                bio: row.get::<Option<String>, _>("bio"),
                policy_positions: positions,
                contradictions: Vec::new(),
                created_at: row.get::<Option<String>, _>("created_at"),
            }
        })
        .collect();

    Ok(opponents)
}

/// Get a single opponent by ID.
#[server(endpoint = "opposition/get-opponent")]
pub async fn get_opponent(id: String) -> Result<Opponent, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    let row = sqlx::query(
        r#"SELECT
            id::text,
            name,
            district,
            bio,
            COALESCE(policy_positions, '{}'::jsonb) AS policy_positions,
            to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
        FROM candidates
        WHERE id::text = $1 AND role = 'opponent'"#,
    )
    .bind(&id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
    .ok_or_else(|| ServerFnError::new("Opponent not found"))?;

    let positions: serde_json::Value = row.get("policy_positions");
    let party = positions
        .get("party")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(Opponent {
        id: row.get::<String, _>("id"),
        name: row.get::<String, _>("name"),
        party,
        district: row.get::<Option<String>, _>("district"),
        bio: row.get::<Option<String>, _>("bio"),
        policy_positions: positions,
        contradictions: Vec::new(),
        created_at: row.get::<Option<String>, _>("created_at"),
    })
}

/// A structured debate briefing returned by the LLM.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DebateBriefing {
    pub opponent_id: String,
    pub opponent_name: String,
    pub content: String,
    pub topics: Vec<String>,
    pub focus: String,
}

/// Generate a structured debate briefing for an opponent.
#[server(endpoint = "opposition/generate-briefing")]
pub async fn generate_briefing(
    opponent_id: String,
    topics: String,
    focus: String,
) -> Result<DebateBriefing, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{require_user, LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let _user = require_user()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    // Load opponent
    let row = sqlx::query(
        r#"SELECT
            id::text, name, district, bio,
            COALESCE(policy_positions, '{}'::jsonb) AS policy_positions
        FROM candidates
        WHERE id::text = $1 AND role = 'opponent'"#,
    )
    .bind(&opponent_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
    .ok_or_else(|| ServerFnError::new("Opponent not found"))?;

    let name: String = row.get("name");
    let district: Option<String> = row.get("district");
    let bio: Option<String> = row.get("bio");
    let positions: serde_json::Value = row.get("policy_positions");

    let topic_list: Vec<String> = topics
        .split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();

    let topics_str = if topic_list.is_empty() {
        "all relevant topics".to_string()
    } else {
        topic_list.join(", ")
    };

    let focus_str = if focus.is_empty() {
        "general debate preparation".to_string()
    } else {
        focus.clone()
    };

    let positions_pretty =
        serde_json::to_string_pretty(&positions).unwrap_or_else(|_| positions.to_string());

    let user_prompt = format!(
        "Generate a debate briefing for the following opponent:\n\n\
         Name: {name}\n\
         District: {district}\n\
         Bio: {bio}\n\n\
         Policy Positions:\n{positions_pretty}\n\n\
         Focus areas: {topics_str}\n\
         Briefing focus: {focus_str}",
        district = district.as_deref().unwrap_or("Unknown"),
        bio = bio.as_deref().unwrap_or("No bio available"),
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: BRIEFING_SYSTEM_PROMPT.to_string(),
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
        .generate(&messages, None, Some(0.4), Some(2048))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "opposition_research",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    tracing::info!(
        opponent_id = %opponent_id,
        name = %name,
        latency_ms = latency_ms,
        "Debate briefing generated"
    );

    Ok(DebateBriefing {
        opponent_id,
        opponent_name: name,
        content: response.content,
        topics: topic_list,
        focus: focus_str,
    })
}

/// Detect contradictions in an opponent's policy positions using LLM analysis.
#[server(endpoint = "opposition/detect-contradictions")]
pub async fn detect_contradictions(
    opponent_id: String,
) -> Result<Vec<Contradiction>, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{require_user, LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let _user = require_user()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    // Load opponent
    let row = sqlx::query(
        r#"SELECT
            id::text, name,
            COALESCE(policy_positions, '{}'::jsonb) AS policy_positions
        FROM candidates
        WHERE id::text = $1 AND role = 'opponent'"#,
    )
    .bind(&opponent_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
    .ok_or_else(|| ServerFnError::new("Opponent not found"))?;

    let name: String = row.get("name");
    let positions: serde_json::Value = row.get("policy_positions");

    let positions_pretty =
        serde_json::to_string_pretty(&positions).unwrap_or_else(|_| positions.to_string());

    let user_prompt = format!(
        "Analyze the following policy positions for {name} and identify any contradictions, \
         flip-flops, or inconsistencies:\n\n{positions_pretty}"
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: CONTRADICTION_SYSTEM_PROMPT.to_string(),
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
        .generate(&messages, None, Some(0.2), Some(2048))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "opposition_research",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    // Parse the LLM response as a JSON array of contradictions
    let content = response.content.trim();

    // Strip markdown code fences if present
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

    let contradictions: Vec<Contradiction> = serde_json::from_str(json_str).map_err(|e| {
        tracing::warn!(
            opponent_id = %opponent_id,
            raw = %content,
            "Failed to parse contradiction response: {e}"
        );
        ServerFnError::new(format!(
            "Failed to parse contradiction analysis. The LLM returned an unexpected format: {e}"
        ))
    })?;

    tracing::info!(
        opponent_id = %opponent_id,
        name = %name,
        count = contradictions.len(),
        latency_ms = latency_ms,
        "Contradictions detected"
    );

    Ok(contradictions)
}
