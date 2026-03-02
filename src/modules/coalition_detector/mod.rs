//! F11: Coalition Tension Detector
//!
//! Detects emerging tensions within political coalitions by analyzing
//! public statements, voting patterns, and communication shifts.

use dioxus::prelude::*;

/// A segment within a political coalition.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CoalitionSegment {
    pub name: String,
    pub description: String,
    pub key_priorities: Vec<String>,
}

/// A detected tension between two coalition segments.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tension {
    pub segment_a: String,
    pub segment_b: String,
    pub issue: String,
    pub severity: String,
    pub explanation: String,
}

/// Full tension analysis result for a policy text across coalition segments.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TensionAnalysis {
    pub id: String,
    pub policy_text: String,
    pub segments: Vec<CoalitionSegment>,
    pub tensions: Vec<Tension>,
    pub overall_stress_score: f64,
    pub recommendations: Vec<String>,
    pub created_at: Option<String>,
}

/// Return a set of hardcoded default coalition segments.
#[server(endpoint = "coalition/default-segments")]
pub async fn get_default_segments() -> Result<Vec<CoalitionSegment>, ServerFnError> {
    Ok(vec![
        CoalitionSegment {
            name: "Labor Unions".into(),
            description: "Organized labor groups advocating for worker rights, fair wages, and workplace protections.".into(),
            key_priorities: vec![
                "Higher minimum wage".into(),
                "Union protections".into(),
                "Workplace safety regulations".into(),
                "Healthcare benefits".into(),
            ],
        },
        CoalitionSegment {
            name: "Environmental Groups".into(),
            description: "Organizations focused on climate action, conservation, and environmental justice.".into(),
            key_priorities: vec![
                "Carbon emissions reduction".into(),
                "Renewable energy investment".into(),
                "Environmental regulations".into(),
                "Green infrastructure".into(),
            ],
        },
        CoalitionSegment {
            name: "Business Community".into(),
            description: "Chambers of commerce, trade associations, and business owners focused on economic growth.".into(),
            key_priorities: vec![
                "Tax reduction".into(),
                "Deregulation".into(),
                "Free trade agreements".into(),
                "Business-friendly policies".into(),
            ],
        },
        CoalitionSegment {
            name: "Religious Conservatives".into(),
            description: "Faith-based groups emphasizing traditional values and religious liberty.".into(),
            key_priorities: vec![
                "Religious freedom protections".into(),
                "Traditional family values".into(),
                "Faith-based education".into(),
                "Moral legislation".into(),
            ],
        },
        CoalitionSegment {
            name: "Youth Activists".into(),
            description: "Young political organizers focused on progressive social change and generational equity.".into(),
            key_priorities: vec![
                "Student debt relief".into(),
                "Climate justice".into(),
                "Social equity".into(),
                "Digital rights".into(),
            ],
        },
        CoalitionSegment {
            name: "Rural Communities".into(),
            description: "Agricultural and rural interest groups focused on rural development and farming issues.".into(),
            key_priorities: vec![
                "Agricultural subsidies".into(),
                "Rural broadband access".into(),
                "Land use rights".into(),
                "Small town economic development".into(),
            ],
        },
    ])
}

/// Analyze how a policy text would create tensions among selected coalition segments.
#[server(endpoint = "coalition/analyze-tensions")]
pub async fn analyze_tensions(
    policy_text: String,
    segments: Vec<CoalitionSegment>,
) -> Result<TensionAnalysis, ServerFnError> {
    use crate::infrastructure::{LlmClient, LlmMessage, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if policy_text.trim().is_empty() {
        return Err(ServerFnError::new("Policy text cannot be empty"));
    }
    if segments.len() < 2 {
        return Err(ServerFnError::new(
            "Select at least two coalition segments to analyze tensions",
        ));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    // Build segment descriptions for the prompt
    let segments_desc: String = segments
        .iter()
        .map(|s| {
            format!(
                "- {} ({}): priorities = [{}]",
                s.name,
                s.description,
                s.key_priorities.join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let messages = vec![
        LlmMessage {
            role: "system".into(),
            content: concat!(
                "You are a political coalition analyst. Given a policy text and a set of coalition ",
                "segments, analyze how the policy would affect each segment and identify tensions ",
                "between segment pairs.\n\n",
                "Respond with ONLY valid JSON in this exact format:\n",
                "{\n",
                "  \"tensions\": [\n",
                "    {\n",
                "      \"segment_a\": \"<exact segment name>\",\n",
                "      \"segment_b\": \"<exact segment name>\",\n",
                "      \"issue\": \"<brief description of tension point>\",\n",
                "      \"severity\": \"low\" | \"medium\" | \"high\" | \"critical\",\n",
                "      \"explanation\": \"<detailed explanation of why this tension exists>\"\n",
                "    }\n",
                "  ],\n",
                "  \"overall_stress_score\": <0.0 to 1.0>,\n",
                "  \"recommendations\": [\"<recommendation1>\", \"<recommendation2>\"]\n",
                "}\n\n",
                "Identify all meaningful tension pairs. The overall_stress_score reflects how much ",
                "the policy strains the coalition as a whole (0 = no stress, 1 = coalition breaking). ",
                "Provide 3-5 actionable recommendations for managing tensions."
            )
            .into(),
        },
        LlmMessage {
            role: "user".into(),
            content: format!(
                "Policy text:\n{policy_text}\n\nCoalition segments:\n{segments_desc}\n\n\
                Analyze the tensions this policy would create among these coalition segments."
            ),
        },
    ];

    let start = std::time::Instant::now();
    let response = llm
        .generate(&messages, None, Some(0.3), Some(2048))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM tension analysis failed: {e}")))?;
    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "coalition_detector",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    let parsed: serde_json::Value = serde_json::from_str(&response.content)
        .map_err(|e| ServerFnError::new(format!("Failed to parse LLM response: {e}")))?;

    let tensions: Vec<Tension> = parsed["tensions"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| {
                    Some(Tension {
                        segment_a: v["segment_a"].as_str()?.to_string(),
                        segment_b: v["segment_b"].as_str()?.to_string(),
                        issue: v["issue"].as_str()?.to_string(),
                        severity: v["severity"]
                            .as_str()
                            .unwrap_or("medium")
                            .to_string(),
                        explanation: v["explanation"].as_str()?.to_string(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let overall_stress_score = parsed["overall_stress_score"].as_f64().unwrap_or(0.0);

    let recommendations: Vec<String> = parsed["recommendations"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    Ok(TensionAnalysis {
        id: uuid::Uuid::new_v4().to_string(),
        policy_text,
        segments,
        tensions,
        overall_stress_score,
        recommendations,
        created_at: Some(now),
    })
}
