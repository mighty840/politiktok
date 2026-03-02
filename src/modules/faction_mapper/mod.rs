//! F19: Internal Faction & Consensus Mapper
//!
//! Maps internal party factions and identifies consensus opportunities
//! by analyzing voting records, public statements, and policy positions.

use dioxus::prelude::*;

/// A political faction within a party or movement.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Faction {
    pub name: String,
    pub ideology: String,
    pub key_figures: Vec<String>,
    pub positions: Vec<String>,
    pub influence_score: f64,
}

/// Complete faction analysis for a given political context.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FactionAnalysis {
    pub id: String,
    pub context: String,
    pub factions: Vec<Faction>,
    pub alliances: Vec<(String, String)>,
    pub conflicts: Vec<(String, String, String)>,
    pub created_at: String,
}

/// Consensus mapping result for a policy proposal across factions.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConsensusMap {
    pub proposal: String,
    pub supporters: Vec<String>,
    pub opponents: Vec<String>,
    pub swing_factions: Vec<String>,
    pub analysis: String,
}

const FACTION_SYSTEM_PROMPT: &str = "\
You are a political analyst specializing in internal party dynamics. Given a party context and known \
political figures, identify the internal factions, their ideological positions, and relationships.

Return your analysis as a JSON object with these fields:
- factions: An array of objects, each with:
  - name: The faction's common name or label
  - ideology: Brief description of ideological position
  - key_figures: Array of notable members
  - positions: Array of key policy positions
  - influence_score: A score from 0.0 to 1.0 representing relative influence
- alliances: An array of [faction_a_name, faction_b_name] pairs that tend to align
- conflicts: An array of [faction_a_name, faction_b_name, issue] triples describing key conflicts

Return ONLY the JSON object, no other text.";

const CONSENSUS_SYSTEM_PROMPT: &str = "\
You are a political strategist analyzing faction dynamics around a specific policy proposal. \
Given a set of factions and a policy proposal, determine which factions support it, oppose it, \
and which could be persuaded.

Return your analysis as a JSON object with these fields:
- supporters: Array of faction names that would support the proposal
- opponents: Array of faction names that would oppose the proposal
- swing_factions: Array of faction names that could go either way
- analysis: A 2-4 sentence strategic analysis of how to build consensus

Return ONLY the JSON object, no other text.";

/// Analyze internal factions within a party or political context.
#[server(endpoint = "faction-mapper/analyze-factions")]
pub async fn analyze_factions(
    party_context: String,
    known_figures: String,
) -> Result<FactionAnalysis, ServerFnError> {
    use crate::infrastructure::{LlmClient, ServerState};
    use crate::infrastructure::llm::LlmMessage;
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if party_context.trim().is_empty() {
        return Err(ServerFnError::new("Party context cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let figures_section = if known_figures.trim().is_empty() {
        String::new()
    } else {
        format!("\n\nKnown figures:\n{known_figures}")
    };

    let user_prompt = format!(
        "Analyze the internal factions for the following political context:\n\n{party_context}{figures_section}"
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: FACTION_SYSTEM_PROMPT.to_string(),
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let start = std::time::Instant::now();

    let response = llm
        .generate(&messages, None, Some(0.4), Some(3000))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "faction_mapper",
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
    struct FactionResponse {
        factions: Vec<Faction>,
        alliances: Vec<(String, String)>,
        conflicts: Vec<(String, String, String)>,
    }

    let parsed: FactionResponse = serde_json::from_str(json_str).map_err(|e| {
        tracing::warn!(raw = %content, "Failed to parse faction response: {e}");
        ServerFnError::new(format!("Failed to parse faction analysis: {e}"))
    })?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tracing::info!(
        factions = parsed.factions.len(),
        alliances = parsed.alliances.len(),
        conflicts = parsed.conflicts.len(),
        latency_ms = latency_ms,
        "Faction analysis completed"
    );

    Ok(FactionAnalysis {
        id: uuid::Uuid::new_v4().to_string(),
        context: party_context,
        factions: parsed.factions,
        alliances: parsed.alliances,
        conflicts: parsed.conflicts,
        created_at: now,
    })
}

/// Map consensus and conflict across factions for a specific policy proposal.
#[server(endpoint = "faction-mapper/map-consensus")]
pub async fn map_consensus(
    factions_json: String,
    policy_proposal: String,
) -> Result<ConsensusMap, ServerFnError> {
    use crate::infrastructure::{LlmClient, ServerState};
    use crate::infrastructure::llm::LlmMessage;
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if policy_proposal.trim().is_empty() {
        return Err(ServerFnError::new("Policy proposal cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let user_prompt = format!(
        "Given these factions:\n{factions_json}\n\n\
         Analyze consensus and conflict for this policy proposal:\n{policy_proposal}"
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: CONSENSUS_SYSTEM_PROMPT.to_string(),
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
        "faction_mapper",
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
    struct ConsensusResponse {
        supporters: Vec<String>,
        opponents: Vec<String>,
        swing_factions: Vec<String>,
        analysis: String,
    }

    let parsed: ConsensusResponse = serde_json::from_str(json_str).map_err(|e| {
        tracing::warn!(raw = %content, "Failed to parse consensus response: {e}");
        ServerFnError::new(format!("Failed to parse consensus map: {e}"))
    })?;

    tracing::info!(
        supporters = parsed.supporters.len(),
        opponents = parsed.opponents.len(),
        latency_ms = latency_ms,
        "Consensus mapping completed"
    );

    Ok(ConsensusMap {
        proposal: policy_proposal,
        supporters: parsed.supporters,
        opponents: parsed.opponents,
        swing_factions: parsed.swing_factions,
        analysis: parsed.analysis,
    })
}
