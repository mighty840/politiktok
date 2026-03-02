//! F16: Voter Question Anticipation
//!
//! Predicts likely voter questions based on current events, local issues,
//! and trending topics, preparing candidates with ready responses.

use dioxus::prelude::*;

/// A single anticipated voter question with metadata and suggested response.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnticipatedQuestion {
    pub question: String,
    pub likelihood: String,
    pub topic: String,
    pub suggested_answer: String,
    pub preparation_notes: String,
}

/// A complete question anticipation report for an event.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuestionReport {
    pub id: String,
    pub context: String,
    pub event_type: String,
    pub questions: Vec<AnticipatedQuestion>,
    pub created_at: Option<String>,
}

/// Anticipate voter questions for an upcoming event.
///
/// Uses AI to generate the top 10 most likely voter questions based on
/// the event context, type, and current hot topics. Each question includes
/// a likelihood rating, topic category, suggested answer, and preparation notes.
#[server(endpoint = "call-intel/anticipate-questions")]
pub async fn anticipate_questions(
    context: String,
    event_type: String,
    hot_topics: String,
) -> Result<QuestionReport, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if context.trim().is_empty() {
        return Err(ServerFnError::new("Context cannot be empty"));
    }
    if event_type.trim().is_empty() {
        return Err(ServerFnError::new("Event type cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let system_prompt = "\
You are an expert political strategist and debate preparation coach. \
Your job is to anticipate the most likely questions voters, journalists, \
or moderators will ask at political events.\n\n\
Generate exactly 10 anticipated questions. Return a JSON array where each element has:\n\
- \"question\": The anticipated question text\n\
- \"likelihood\": One of \"high\", \"medium\", or \"low\"\n\
- \"topic\": The topic category (e.g., \"Economy\", \"Healthcare\", \"Education\")\n\
- \"suggested_answer\": A well-crafted suggested answer (2-4 sentences)\n\
- \"preparation_notes\": Brief notes on how to prepare for this question\n\n\
Order questions from most likely to least likely. \
Ensure at least 3 are \"high\" likelihood, at least 3 are \"medium\", and the rest can be \"low\".\n\n\
Return ONLY the JSON array, no markdown fences or extra text.";

    let topics_section = if hot_topics.trim().is_empty() {
        String::new()
    } else {
        format!("\n\nCurrent hot topics:\n{hot_topics}")
    };

    let user_prompt = format!(
        "Event type: {event_type}\n\n\
Context and background:\n{context}{topics_section}\n\n\
Generate 10 anticipated voter questions for this event."
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let start = std::time::Instant::now();

    let response = llm
        .generate(&messages, None, Some(0.7), Some(2048))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM error: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "question_anticipation",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    // Parse the LLM JSON response
    let raw = response.content.trim();
    let json_str = if raw.starts_with("```") {
        raw.trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
    } else {
        raw
    };

    let parsed: Vec<serde_json::Value> = serde_json::from_str(json_str)
        .map_err(|e| ServerFnError::new(format!("Failed to parse LLM response as JSON: {e}")))?;

    let questions: Vec<AnticipatedQuestion> = parsed
        .iter()
        .map(|q| AnticipatedQuestion {
            question: q["question"]
                .as_str()
                .unwrap_or("Unknown question")
                .to_string(),
            likelihood: q["likelihood"].as_str().unwrap_or("medium").to_string(),
            topic: q["topic"].as_str().unwrap_or("General").to_string(),
            suggested_answer: q["suggested_answer"]
                .as_str()
                .unwrap_or("No suggested answer available")
                .to_string(),
            preparation_notes: q["preparation_notes"]
                .as_str()
                .unwrap_or("No preparation notes available")
                .to_string(),
        })
        .collect();

    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    tracing::info!(
        id = %id,
        event_type = %event_type,
        questions_count = questions.len(),
        latency_ms = latency_ms,
        "Voter questions anticipated"
    );

    Ok(QuestionReport {
        id,
        context,
        event_type,
        questions,
        created_at: Some(now),
    })
}

/// Generate a preparation checklist from anticipated questions.
///
/// Takes a list of anticipated questions and produces a consolidated
/// preparation checklist with actionable steps.
#[server(endpoint = "call-intel/preparation-checklist")]
pub async fn generate_preparation_checklist(
    questions: Vec<AnticipatedQuestion>,
) -> Result<String, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if questions.is_empty() {
        return Err(ServerFnError::new(
            "At least one anticipated question is required",
        ));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let system_prompt = "\
You are an expert political debate and event preparation coach. \
Given a list of anticipated voter questions, create a concise, actionable \
preparation checklist. Group items by topic area. Each item should be a \
specific, concrete action the candidate or team should take before the event.\n\n\
Format the checklist as plain text with clear headings and bullet points.";

    let questions_text = questions
        .iter()
        .enumerate()
        .map(|(i, q)| {
            format!(
                "{}. [{}] ({}): {}\n   Suggested answer: {}\n   Notes: {}",
                i + 1,
                q.likelihood,
                q.topic,
                q.question,
                q.suggested_answer,
                q.preparation_notes
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    let user_prompt = format!(
        "Create a preparation checklist based on these anticipated questions:\n\n{questions_text}"
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        },
        LlmMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let start = std::time::Instant::now();

    let response = llm
        .generate(&messages, None, Some(0.5), Some(1500))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM error: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "question_anticipation",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    tracing::info!(latency_ms = latency_ms, "Preparation checklist generated");

    Ok(response.content)
}
