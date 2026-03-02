//! F10: Narrative Contagion Model
//!
//! Models how political narratives spread through networks, predicting
//! viral potential and identifying key amplification vectors.

use dioxus::prelude::*;

/// A single narrative identified within a text.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Narrative {
    pub theme: String,
    pub framing: String,
    pub target_audience: String,
    pub virality_score: f64,
    pub emotional_triggers: Vec<String>,
}

/// Full analysis result for a submitted text.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NarrativeAnalysis {
    pub id: String,
    pub input_text: String,
    pub narratives: Vec<Narrative>,
    pub spread_prediction: String,
    pub recommended_responses: Vec<String>,
    pub created_at: Option<String>,
}

/// Analyze a piece of political text for embedded narratives, framing,
/// virality potential, and emotional triggers.
#[server(endpoint = "narrative/analyze")]
pub async fn analyze_narrative(text: String) -> Result<NarrativeAnalysis, ServerFnError> {
    use crate::infrastructure::{LlmClient, LlmMessage, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if text.trim().is_empty() {
        return Err(ServerFnError::new("Input text cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let messages = vec![
        LlmMessage {
            role: "system".into(),
            content: concat!(
                "You are a political narrative analyst. Given a piece of text (news article, ",
                "social media post, speech excerpt, etc.), identify embedded political narratives.\n\n",
                "Respond with ONLY valid JSON in this exact format:\n",
                "{\n",
                "  \"narratives\": [\n",
                "    {\n",
                "      \"theme\": \"<short theme label>\",\n",
                "      \"framing\": \"<how the issue is framed>\",\n",
                "      \"target_audience\": \"<who this narrative appeals to>\",\n",
                "      \"virality_score\": <0.0 to 1.0>,\n",
                "      \"emotional_triggers\": [\"<trigger1>\", \"<trigger2>\"]\n",
                "    }\n",
                "  ],\n",
                "  \"spread_prediction\": \"<paragraph predicting how these narratives might spread>\",\n",
                "  \"recommended_responses\": [\"<response1>\", \"<response2>\"]\n",
                "}\n\n",
                "Identify 1-5 narratives. Be specific and analytical."
            )
            .into(),
        },
        LlmMessage {
            role: "user".into(),
            content: text.clone(),
        },
    ];

    let start = std::time::Instant::now();
    let response = llm
        .generate(&messages, None, Some(0.3), Some(2048))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM analysis failed: {e}")))?;
    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "narrative_contagion",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    let parsed: serde_json::Value = serde_json::from_str(&response.content)
        .map_err(|e| ServerFnError::new(format!("Failed to parse LLM response: {e}")))?;

    let narratives: Vec<Narrative> = parsed["narratives"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| {
                    Some(Narrative {
                        theme: v["theme"].as_str()?.to_string(),
                        framing: v["framing"].as_str()?.to_string(),
                        target_audience: v["target_audience"].as_str()?.to_string(),
                        virality_score: v["virality_score"].as_f64().unwrap_or(0.0),
                        emotional_triggers: v["emotional_triggers"]
                            .as_array()
                            .map(|a| {
                                a.iter()
                                    .filter_map(|s| s.as_str().map(String::from))
                                    .collect()
                            })
                            .unwrap_or_default(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let spread_prediction = parsed["spread_prediction"]
        .as_str()
        .unwrap_or("No prediction available.")
        .to_string();

    let recommended_responses: Vec<String> = parsed["recommended_responses"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    Ok(NarrativeAnalysis {
        id: uuid::Uuid::new_v4().to_string(),
        input_text: text,
        narratives,
        spread_prediction,
        recommended_responses,
        created_at: Some(now),
    })
}

/// Predict how a specific narrative theme might spread on a given platform.
#[server(endpoint = "narrative/predict-spread")]
pub async fn predict_spread(
    narrative_theme: String,
    platform: String,
) -> Result<String, ServerFnError> {
    use crate::infrastructure::{LlmClient, LlmMessage, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if narrative_theme.trim().is_empty() {
        return Err(ServerFnError::new("Narrative theme cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let messages = vec![
        LlmMessage {
            role: "system".into(),
            content: concat!(
                "You are a political media analyst specializing in narrative spread patterns. ",
                "Given a narrative theme and a platform, predict how this narrative would spread. ",
                "Cover: likely amplification vectors, speed of spread, key demographics that would ",
                "engage, potential mutations of the narrative, and risk assessment. ",
                "Be specific and analytical. Respond in 2-4 paragraphs of plain text."
            )
            .into(),
        },
        LlmMessage {
            role: "user".into(),
            content: format!(
                "Narrative theme: {narrative_theme}\nPlatform: {platform}\n\n\
                Predict how this narrative would spread on this platform."
            ),
        },
    ];

    let start = std::time::Instant::now();
    let response = llm
        .generate(&messages, None, Some(0.5), Some(1024))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM prediction failed: {e}")))?;
    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "narrative_contagion",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    Ok(response.content)
}

/// Suggest counter-narrative messaging for a given narrative theme.
#[server(endpoint = "narrative/counter")]
pub async fn suggest_counter_narrative(
    narrative_theme: String,
) -> Result<Vec<String>, ServerFnError> {
    use crate::infrastructure::{LlmClient, LlmMessage, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if narrative_theme.trim().is_empty() {
        return Err(ServerFnError::new("Narrative theme cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let messages = vec![
        LlmMessage {
            role: "system".into(),
            content: concat!(
                "You are a strategic communications expert specializing in counter-narrative development. ",
                "Given a political narrative theme, suggest effective counter-messaging strategies.\n\n",
                "Respond with ONLY a valid JSON array of strings, each being a specific counter-narrative ",
                "suggestion. Provide 3-6 suggestions. Example:\n",
                "[\"Counter-suggestion 1\", \"Counter-suggestion 2\", \"Counter-suggestion 3\"]"
            )
            .into(),
        },
        LlmMessage {
            role: "user".into(),
            content: format!(
                "Narrative theme: {narrative_theme}\n\n\
                Suggest effective counter-narratives for this theme."
            ),
        },
    ];

    let start = std::time::Instant::now();
    let response = llm
        .generate(&messages, None, Some(0.5), Some(1024))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM counter-narrative failed: {e}")))?;
    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "narrative_contagion",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    let suggestions: Vec<String> = serde_json::from_str(&response.content).map_err(|e| {
        ServerFnError::new(format!("Failed to parse counter-narrative response: {e}"))
    })?;

    Ok(suggestions)
}
