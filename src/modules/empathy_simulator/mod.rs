//! F09: Audience Empathy Simulator
//!
//! Simulates how different audience segments would perceive and react to
//! political messaging, helping refine communication strategies.

use dioxus::prelude::*;

/// A persona representing a specific audience segment.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Persona {
    pub id: String,
    pub name: String,
    pub demographic: String,
    pub concerns: Vec<String>,
    pub values: Vec<String>,
    pub communication_style: String,
}

/// A simulated reaction from a specific persona to a policy text.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PersonaReaction {
    pub persona: Persona,
    pub reaction: String,
    pub sentiment: String, // positive, negative, neutral, mixed
    pub key_concerns: Vec<String>,
    pub persuasion_score: f64, // 0-1
}

/// The full result of simulating reactions across multiple personas.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SimulationResult {
    pub id: String,
    pub policy_text: String,
    pub reactions: Vec<PersonaReaction>,
    pub aggregate_sentiment: String,
    pub red_flags: Vec<String>,
    pub created_at: Option<String>,
}

/// Return a hardcoded set of 6 diverse default personas.
#[server(endpoint = "empathy-simulator/default-personas")]
pub async fn get_default_personas() -> Result<Vec<Persona>, ServerFnError> {
    Ok(vec![
        Persona {
            id: "working-parent".to_string(),
            name: "Working Parent".to_string(),
            demographic: "35-45, dual-income household, suburban, two school-age children".to_string(),
            concerns: vec![
                "Childcare costs".to_string(),
                "School quality".to_string(),
                "Work-life balance".to_string(),
                "Healthcare affordability".to_string(),
                "Housing prices".to_string(),
            ],
            values: vec![
                "Family stability".to_string(),
                "Economic opportunity".to_string(),
                "Public education".to_string(),
                "Community safety".to_string(),
            ],
            communication_style: "Practical and results-oriented. Responds to concrete examples and specific numbers. Skeptical of vague promises. Wants to know exactly how a policy will affect their family's daily life and budget.".to_string(),
        },
        Persona {
            id: "retired-veteran".to_string(),
            name: "Retired Veteran".to_string(),
            demographic: "65-75, military veteran, fixed income, small town".to_string(),
            concerns: vec![
                "VA healthcare access".to_string(),
                "Social Security stability".to_string(),
                "National security".to_string(),
                "Rising cost of living".to_string(),
                "Government accountability".to_string(),
            ],
            values: vec![
                "Patriotism".to_string(),
                "Duty and service".to_string(),
                "Fiscal responsibility".to_string(),
                "Respect for institutions".to_string(),
            ],
            communication_style: "Values directness and honesty. Dislikes political spin. Appreciates acknowledgment of service and sacrifice. Prefers straightforward language over jargon. Wants to see follow-through on promises.".to_string(),
        },
        Persona {
            id: "college-student".to_string(),
            name: "College Student".to_string(),
            demographic: "18-22, first-generation college student, urban campus, part-time job".to_string(),
            concerns: vec![
                "Student loan debt".to_string(),
                "Job market prospects".to_string(),
                "Climate change".to_string(),
                "Social justice".to_string(),
                "Mental health resources".to_string(),
            ],
            values: vec![
                "Equity and inclusion".to_string(),
                "Environmental sustainability".to_string(),
                "Innovation".to_string(),
                "Authenticity".to_string(),
            ],
            communication_style: "Digital-native, values authenticity over polish. Engages with data-backed arguments but also emotional narratives. Skeptical of establishment rhetoric. Responds to bold vision and systemic change proposals.".to_string(),
        },
        Persona {
            id: "small-business-owner".to_string(),
            name: "Small Business Owner".to_string(),
            demographic: "40-55, owns a local restaurant, employs 12 people, semi-urban".to_string(),
            concerns: vec![
                "Regulatory burden".to_string(),
                "Tax policy".to_string(),
                "Healthcare costs for employees".to_string(),
                "Minimum wage changes".to_string(),
                "Supply chain stability".to_string(),
            ],
            values: vec![
                "Entrepreneurship".to_string(),
                "Local community".to_string(),
                "Self-reliance".to_string(),
                "Economic freedom".to_string(),
            ],
            communication_style: "Bottom-line focused. Wants to know the real-world impact on small businesses specifically, not just big corporations. Frustrated by one-size-fits-all policies. Appreciates when policymakers show they understand the challenges of running a small business.".to_string(),
        },
        Persona {
            id: "healthcare-worker".to_string(),
            name: "Healthcare Worker".to_string(),
            demographic: "30-40, registered nurse, works at a community hospital, suburban".to_string(),
            concerns: vec![
                "Staffing shortages".to_string(),
                "Burnout and mental health".to_string(),
                "Patient care quality".to_string(),
                "Insurance bureaucracy".to_string(),
                "Workplace safety".to_string(),
            ],
            values: vec![
                "Compassion".to_string(),
                "Evidence-based policy".to_string(),
                "Worker protections".to_string(),
                "Universal access to care".to_string(),
            ],
            communication_style: "Evidence-driven and empathetic. Respects policy grounded in data and research. Frustrated by politicization of healthcare. Wants practical solutions that address frontline worker experiences. Values being listened to, not talked at.".to_string(),
        },
        Persona {
            id: "rural-farmer".to_string(),
            name: "Rural Farmer".to_string(),
            demographic: "50-65, third-generation farmer, 500-acre operation, deeply rural county".to_string(),
            concerns: vec![
                "Farm subsidies and trade policy".to_string(),
                "Water rights and access".to_string(),
                "Broadband internet availability".to_string(),
                "Land use regulations".to_string(),
                "Extreme weather and crop insurance".to_string(),
            ],
            values: vec![
                "Independence".to_string(),
                "Stewardship of the land".to_string(),
                "Tradition".to_string(),
                "Local governance".to_string(),
            ],
            communication_style: "Plain-spoken and skeptical of Washington insiders. Values actions over words. Deeply connected to the land and community. Dislikes being talked down to or treated as a stereotype. Wants to see policymakers actually visit rural areas and listen.".to_string(),
        },
    ])
}

/// Build the system prompt used for simulating a persona's reaction.
fn build_persona_system_prompt(persona: &Persona) -> String {
    format!(
        "You are simulating the reaction of a specific person to a political policy proposal. \
You must respond AS this person, reflecting their worldview, concerns, and communication style.\n\n\
Persona: {name}\n\
Demographic: {demo}\n\
Key concerns: {concerns}\n\
Core values: {values}\n\
Communication style: {style}\n\n\
You must respond with a valid JSON object (no markdown fences, no extra text) with these exact fields:\n\
- \"reaction\": A 2-4 sentence first-person reaction to the policy, written in the persona's voice and communication style.\n\
- \"sentiment\": One of \"positive\", \"negative\", \"neutral\", or \"mixed\".\n\
- \"key_concerns\": An array of 1-3 specific concerns this persona would have about the policy.\n\
- \"persuasion_score\": A number from 0.0 to 1.0 indicating how persuasive this policy would be to this persona (0 = not at all, 1 = extremely persuasive).\n\n\
Respond ONLY with the JSON object.",
        name = persona.name,
        demo = persona.demographic,
        concerns = persona.concerns.join(", "),
        values = persona.values.join(", "),
        style = persona.communication_style,
    )
}

/// Parse the LLM JSON response for a persona reaction.
fn parse_persona_reaction(
    persona: &Persona,
    raw: &str,
) -> PersonaReaction {
    // Try to parse JSON; fall back gracefully if the LLM returns imperfect JSON.
    let cleaned = raw.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(cleaned) {
        PersonaReaction {
            persona: persona.clone(),
            reaction: json["reaction"]
                .as_str()
                .unwrap_or("No reaction generated.")
                .to_string(),
            sentiment: json["sentiment"]
                .as_str()
                .unwrap_or("neutral")
                .to_string(),
            key_concerns: json["key_concerns"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            persuasion_score: json["persuasion_score"]
                .as_f64()
                .unwrap_or(0.5)
                .clamp(0.0, 1.0),
        }
    } else {
        // Fallback: use the raw text as the reaction
        PersonaReaction {
            persona: persona.clone(),
            reaction: raw.to_string(),
            sentiment: "neutral".to_string(),
            key_concerns: vec![],
            persuasion_score: 0.5,
        }
    }
}

/// Compute the aggregate sentiment from a list of individual sentiments.
fn compute_aggregate_sentiment(reactions: &[PersonaReaction]) -> String {
    if reactions.is_empty() {
        return "neutral".to_string();
    }

    let mut positive = 0;
    let mut negative = 0;
    let mut neutral = 0;
    let mut mixed = 0;

    for r in reactions {
        match r.sentiment.as_str() {
            "positive" => positive += 1,
            "negative" => negative += 1,
            "mixed" => mixed += 1,
            _ => neutral += 1,
        }
    }

    let total = reactions.len();
    if positive > total / 2 {
        "positive".to_string()
    } else if negative > total / 2 {
        "negative".to_string()
    } else if mixed + neutral >= positive.max(negative) {
        "mixed".to_string()
    } else {
        "mixed".to_string()
    }
}

/// Extract red flags from the set of persona reactions.
///
/// Red flags are concerns that appear across multiple personas or
/// reactions with very low persuasion scores.
fn extract_red_flags(reactions: &[PersonaReaction]) -> Vec<String> {
    let mut flags = Vec::new();

    // Flag any persona with very low persuasion score
    for r in reactions {
        if r.persuasion_score < 0.3 {
            flags.push(format!(
                "Very low persuasion for {}: {:.0}% -- this group may be alienated",
                r.persona.name,
                r.persuasion_score * 100.0,
            ));
        }
    }

    // Flag strongly negative sentiments
    let negative_count = reactions.iter().filter(|r| r.sentiment == "negative").count();
    if negative_count > reactions.len() / 2 {
        flags.push("Majority of personas reacted negatively -- consider revising messaging".to_string());
    }

    // Find concerns that appear across multiple personas
    let mut concern_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for r in reactions {
        for c in &r.key_concerns {
            let normalized = c.to_lowercase();
            *concern_counts.entry(normalized).or_insert(0) += 1;
        }
    }
    for (concern, count) in &concern_counts {
        if *count >= 2 {
            flags.push(format!(
                "Shared concern across {} personas: \"{}\"",
                count, concern,
            ));
        }
    }

    flags
}

/// Simulate persona reactions to a given policy text.
///
/// For each selected persona, calls the LLM to generate a simulated reaction,
/// then aggregates the results with overall sentiment and red flags.
#[server(endpoint = "empathy-simulator/simulate")]
pub async fn simulate_reactions(
    policy_text: String,
    personas: Vec<Persona>,
) -> Result<SimulationResult, ServerFnError> {
    use crate::infrastructure::{LlmClient, ServerState};
    use crate::infrastructure::llm::LlmMessage;
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if policy_text.trim().is_empty() {
        return Err(ServerFnError::new("Policy text cannot be empty"));
    }
    if personas.is_empty() {
        return Err(ServerFnError::new("At least one persona must be selected"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let sim_id = uuid::Uuid::new_v4().to_string();
    let mut reactions = Vec::new();

    for persona in &personas {
        let system_prompt = build_persona_system_prompt(persona);
        let user_prompt = format!(
            "Please read the following policy proposal and react to it as described in your persona:\n\n\
---\n{}\n---",
            policy_text,
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
            .generate(&messages, None, Some(0.7), Some(512))
            .await
            .map_err(|e| {
                ServerFnError::new(format!(
                    "LLM error for persona '{}': {e}",
                    persona.name
                ))
            })?;

        let latency_ms = start.elapsed().as_millis() as i32;

        // Log LLM usage
        let _ = crate::infrastructure::log_llm_usage(
            pool,
            "empathy_simulator",
            &state.llm_config.model,
            response.prompt_tokens,
            response.completion_tokens,
            latency_ms,
        )
        .await;

        let reaction = parse_persona_reaction(persona, &response.content);
        reactions.push(reaction);

        tracing::info!(
            sim_id = %sim_id,
            persona = %persona.name,
            latency_ms = latency_ms,
            "Empathy simulation generated for persona"
        );
    }

    let aggregate_sentiment = compute_aggregate_sentiment(&reactions);
    let red_flags = extract_red_flags(&reactions);
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    Ok(SimulationResult {
        id: sim_id,
        policy_text,
        reactions,
        aggregate_sentiment,
        red_flags,
        created_at: Some(now),
    })
}
