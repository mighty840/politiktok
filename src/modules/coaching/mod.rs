//! F14: Candidate Coaching & Debate Rehearsal
//!
//! Provides AI-driven coaching for candidates including debate rehearsal,
//! presentation feedback, and communication skills development.
//!
//! Sessions are maintained in-memory via the messages vector passed between
//! client and server — no database persistence is required.

use dioxus::prelude::*;

/// A coaching session containing configuration and conversation history.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CoachingSession {
    pub id: String,
    pub mode: String,       // "journalist", "debate", "townhall"
    pub topics: Vec<String>,
    pub difficulty: String,  // "easy", "medium", "hard"
    pub messages: Vec<CoachingMessage>,
    pub created_at: Option<String>,
}

/// A single message in a coaching conversation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CoachingMessage {
    pub role: String, // "interviewer" or "candidate"
    pub content: String,
}

/// AI-generated feedback report on a coaching session.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CoachingFeedback {
    pub overall_score: f64,
    pub strengths: Vec<String>,
    pub areas_for_improvement: Vec<String>,
    pub specific_feedback: Vec<String>,
}

/// Build the system prompt for coaching based on mode and difficulty.
fn build_coaching_system_prompt(mode: &str, topics: &[String], difficulty: &str) -> String {
    let persona = match mode {
        "journalist" => "\
You are a seasoned political journalist conducting a tough but fair press interview. \
Ask probing follow-up questions, challenge vague answers, and press for specifics. \
You should ask about policy details, past voting records, and controversial positions.",

        "debate" => "\
You are a skilled debate opponent in a political debate. \
Challenge the candidate's positions with counterarguments, point out inconsistencies, \
and press them to defend their stance. Use rhetorical techniques to test their composure.",

        "townhall" => "\
You are a concerned citizen at a town hall meeting. \
Ask questions that real voters would ask — about local issues, cost of living, \
healthcare, education, and community safety. Be emotional and personal in your questions. \
Follow up if the candidate gives a generic answer.",

        _ => "\
You are a political interviewer. Ask tough but fair questions and follow up on vague answers.",
    };

    let difficulty_instruction = match difficulty {
        "easy" => "Be relatively friendly and give the candidate time to explain. \
Accept reasonable answers without too much pushback.",
        "hard" => "Be very aggressive in your questioning. Interrupt with follow-ups, \
point out contradictions immediately, and do not let the candidate deflect. \
Use loaded questions and create pressure.",
        _ => "Be moderately challenging. Push back on vague answers but give credit \
for substantive responses. Ask clarifying questions when needed.",
    };

    let topics_str = if topics.is_empty() {
        "general political topics".to_string()
    } else {
        topics.join(", ")
    };

    format!(
        "{persona}\n\n\
Difficulty level: {difficulty_instruction}\n\n\
Focus your questions on these topics: {topics_str}\n\n\
Important rules:\n\
- Ask ONE question at a time\n\
- Keep your questions concise (1-3 sentences)\n\
- React to the candidate's previous answer before asking the next question\n\
- Stay in character throughout the conversation\n\
- Do not break the fourth wall or offer coaching advice during the session"
    )
}

/// Start a new coaching session with an opening question from the AI interviewer.
///
/// Creates a new session ID, generates the first question based on mode/topics/difficulty,
/// and returns the session with the opening message.
#[server(endpoint = "coaching/start")]
pub async fn start_coaching_session(
    mode: String,
    topics: Vec<String>,
    difficulty: String,
) -> Result<CoachingSession, ServerFnError> {
    use crate::infrastructure::{LlmClient, ServerState};
    use crate::infrastructure::llm::LlmMessage;
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();
    let system_prompt = build_coaching_system_prompt(&mode, &topics, &difficulty);

    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: system_prompt,
        },
        LlmMessage {
            role: "user".to_string(),
            content: "Begin the session. Ask your first question.".to_string(),
        },
    ];

    let start = std::time::Instant::now();

    let response = llm
        .generate(&messages, None, Some(0.7), Some(512))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM error: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "coaching",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    let session_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let opening_message = CoachingMessage {
        role: "interviewer".to_string(),
        content: response.content,
    };

    tracing::info!(
        session_id = %session_id,
        mode = %mode,
        difficulty = %difficulty,
        "Coaching session started"
    );

    Ok(CoachingSession {
        id: session_id,
        mode,
        topics,
        difficulty,
        messages: vec![opening_message],
        created_at: Some(now),
    })
}

/// Respond to a coaching session with a candidate answer, returning the updated messages
/// with the interviewer's follow-up question appended.
///
/// The full message history is passed from the client to maintain context without DB storage.
#[server(endpoint = "coaching/respond")]
pub async fn respond_to_coaching(
    session_id: String,
    mode: String,
    topics: Vec<String>,
    difficulty: String,
    messages: Vec<CoachingMessage>,
    candidate_response: String,
) -> Result<Vec<CoachingMessage>, ServerFnError> {
    use crate::infrastructure::{LlmClient, ServerState};
    use crate::infrastructure::llm::LlmMessage;
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    if candidate_response.trim().is_empty() {
        return Err(ServerFnError::new("Response cannot be empty"));
    }

    let system_prompt = build_coaching_system_prompt(&mode, &topics, &difficulty);

    // Build full LLM conversation from coaching messages
    let mut llm_messages = vec![LlmMessage {
        role: "system".to_string(),
        content: system_prompt,
    }];

    // Add the "Begin" prompt as the first user turn
    llm_messages.push(LlmMessage {
        role: "user".to_string(),
        content: "Begin the session. Ask your first question.".to_string(),
    });

    // Map coaching messages to LLM messages
    for msg in &messages {
        let role = match msg.role.as_str() {
            "interviewer" => "assistant",
            "candidate" => "user",
            _ => continue,
        };
        llm_messages.push(LlmMessage {
            role: role.to_string(),
            content: msg.content.clone(),
        });
    }

    // Add the new candidate response
    llm_messages.push(LlmMessage {
        role: "user".to_string(),
        content: candidate_response.clone(),
    });

    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let start = std::time::Instant::now();

    let response = llm
        .generate(&llm_messages, None, Some(0.7), Some(512))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM error: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "coaching",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    // Build updated message list
    let mut updated_messages = messages;
    updated_messages.push(CoachingMessage {
        role: "candidate".to_string(),
        content: candidate_response,
    });
    updated_messages.push(CoachingMessage {
        role: "interviewer".to_string(),
        content: response.content,
    });

    tracing::info!(
        session_id = %session_id,
        message_count = updated_messages.len(),
        latency_ms = latency_ms,
        "Coaching response generated"
    );

    Ok(updated_messages)
}

/// Analyze a full coaching conversation and generate a detailed feedback report.
///
/// Evaluates the candidate's performance across the entire session, providing
/// an overall score, strengths, areas for improvement, and specific feedback.
#[server(endpoint = "coaching/feedback")]
pub async fn get_coaching_feedback(
    messages: Vec<CoachingMessage>,
    mode: String,
) -> Result<CoachingFeedback, ServerFnError> {
    use crate::infrastructure::{LlmClient, ServerState};
    use crate::infrastructure::llm::LlmMessage;
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    if messages.is_empty() {
        return Err(ServerFnError::new("No messages to analyze"));
    }

    // Format the conversation for analysis
    let conversation = messages
        .iter()
        .map(|m| {
            let label = match m.role.as_str() {
                "interviewer" => "INTERVIEWER",
                "candidate" => "CANDIDATE",
                _ => "UNKNOWN",
            };
            format!("{label}: {}", m.content)
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    let mode_context = match mode.as_str() {
        "journalist" => "a press interview with a journalist",
        "debate" => "a political debate",
        "townhall" => "a town hall meeting with voters",
        _ => "a political interview",
    };

    let system_prompt = format!(
        "You are an expert political communications coach. Analyze the following \
coaching session transcript from {mode_context} and provide detailed feedback.\n\n\
You MUST respond with valid JSON in exactly this format:\n\
{{\n\
  \"overall_score\": <number from 0 to 100>,\n\
  \"strengths\": [\"strength 1\", \"strength 2\", ...],\n\
  \"areas_for_improvement\": [\"area 1\", \"area 2\", ...],\n\
  \"specific_feedback\": [\"feedback on specific answer 1\", \"feedback on specific answer 2\", ...]\n\
}}\n\n\
Evaluation criteria:\n\
- Clarity and conciseness of answers\n\
- Staying on message while being responsive\n\
- Handling of tough questions and follow-ups\n\
- Use of concrete examples and data\n\
- Emotional intelligence and empathy\n\
- Avoidance of common political pitfalls (deflection, jargon, etc.)\n\n\
Provide 3-5 items for each list. Be specific and actionable in your feedback. \
Reference actual quotes or moments from the conversation."
    );

    let llm_messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: system_prompt,
        },
        LlmMessage {
            role: "user".to_string(),
            content: format!("Here is the coaching session transcript:\n\n{conversation}"),
        },
    ];

    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let start = std::time::Instant::now();

    let response = llm
        .generate(&llm_messages, None, Some(0.3), Some(1500))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM error: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "coaching",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    // Parse JSON response — try to extract JSON from the content
    let content = response.content.trim();
    let json_str = if let Some(start_idx) = content.find('{') {
        if let Some(end_idx) = content.rfind('}') {
            &content[start_idx..=end_idx]
        } else {
            content
        }
    } else {
        content
    };

    let feedback: CoachingFeedback = serde_json::from_str(json_str).map_err(|e| {
        ServerFnError::new(format!(
            "Failed to parse feedback JSON: {e}. Raw response: {content}"
        ))
    })?;

    tracing::info!(
        mode = %mode,
        score = feedback.overall_score,
        latency_ms = latency_ms,
        "Coaching feedback generated"
    );

    Ok(feedback)
}
