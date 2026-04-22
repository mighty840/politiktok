#![cfg_attr(not(feature = "server"), allow(dead_code))]
//! F04: Campaign Copy Generator
//!
//! Generates campaign messaging, ad copy, and communications materials
//! tailored to specific audiences and platforms. Supports multiple output
//! formats including emails, social media posts, press releases, and speeches.

use dioxus::prelude::*;

/// Request payload for generating campaign copy across multiple formats.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CopyGenerationRequest {
    pub topic: String,
    pub key_messages: Vec<String>,
    pub audience: String,
    pub tone: String,
    pub formats: Vec<String>,
    pub word_limits: Option<serde_json::Value>,
}

/// A single piece of generated copy in a specific format.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GeneratedCopy {
    pub format: String,
    pub content: String,
}

/// A copy generation job containing the request and all generated results.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CopyJob {
    pub id: String,
    pub request: CopyGenerationRequest,
    pub results: Vec<GeneratedCopy>,
    pub created_at: Option<String>,
}

/// Build the system prompt for copy generation.
fn build_system_prompt(format: &str, tone: &str, audience: &str) -> String {
    let format_guidance = match format {
        "email" => {
            "\
You are an expert political campaign copywriter specializing in email communications. \
Write a professional campaign email that includes:\n\
- A compelling subject line (on its own line, prefixed with \"Subject: \")\n\
- A warm, personal greeting\n\
- A clear and persuasive body that communicates the key messages\n\
- A strong call-to-action (donate, volunteer, share, attend, etc.)\n\
- A professional sign-off\n\
\n\
The email should feel personal and direct, as if written to a single supporter."
        }

        "social_post" => {
            "\
You are an expert political campaign copywriter specializing in social media. \
Write social media posts for the campaign. Provide TWO versions:\n\
\n\
1. **Twitter/X version** (max 280 characters): Short, punchy, attention-grabbing. \
Include a relevant hashtag.\n\
\n\
2. **Facebook/Instagram version** (max 500 words): Longer form, more detailed, \
emotionally engaging. Include a call-to-action and relevant hashtags.\n\
\n\
Label each version clearly."
        }

        "press_release" => {
            "\
You are an expert political communications professional specializing in press releases. \
Write a formal press release that includes:\n\
- A strong headline in ALL CAPS\n\
- A subheadline that expands on the headline\n\
- A dateline (use [CITY, STATE] as placeholder)\n\
- An impactful opening paragraph (the lede) answering who, what, when, where, why\n\
- Supporting paragraphs with key messages and context\n\
- At least one direct quote attributed to the candidate or campaign spokesperson\n\
- A boilerplate \"About\" paragraph at the end\n\
- End with \"###\" centered\n\
\n\
Follow AP style guidelines."
        }

        "speech" => {
            "\
You are an expert political speechwriter. Write a speech excerpt that includes:\n\
- A powerful opening hook that grabs the audience's attention\n\
- Clear articulation of the key messages using rhetorical devices \
(anaphora, tricolon, antithesis, etc.)\n\
- Concrete examples and stories that illustrate the points\n\
- Applause lines and moments of emotional connection\n\
- A memorable closing that inspires action\n\
\n\
Use short sentences for emphasis. Write for the ear, not the eye. \
Include stage directions in [brackets] where appropriate."
        }

        other => &format!(
            "You are an expert political campaign copywriter. \
Write compelling campaign content in the \"{other}\" format. \
Make it persuasive, clear, and appropriate for the target audience."
        ),
    };

    format!(
        "{format_guidance}\n\n\
Tone: {tone}\n\
Target audience: {audience}\n\n\
Important guidelines:\n\
- Stay on message with the provided key points\n\
- Be authentic and avoid jargon unless appropriate for the audience\n\
- Do not fabricate statistics, quotes, or endorsements\n\
- Maintain a consistent voice throughout"
    )
}

/// Build the user prompt for a specific format generation.
fn build_user_prompt(
    topic: &str,
    key_messages: &[String],
    format: &str,
    word_limit: Option<u64>,
) -> String {
    let key_points = key_messages
        .iter()
        .enumerate()
        .map(|(i, msg)| format!("{}. {}", i + 1, msg))
        .collect::<Vec<_>>()
        .join("\n");

    let limit_instruction = if let Some(limit) = word_limit {
        format!("\n\nWord limit: approximately {limit} words.")
    } else {
        String::new()
    };

    format!(
        "Topic: {topic}\n\n\
Key messages to incorporate:\n\
{key_points}\n\n\
Please write the {format} now.{limit_instruction}"
    )
}

/// Extract the word limit for a specific format from the word_limits JSON.
fn get_word_limit(word_limits: &Option<serde_json::Value>, format: &str) -> Option<u64> {
    word_limits
        .as_ref()
        .and_then(|wl| wl.get(format))
        .and_then(|v| v.as_u64())
}

/// Generate campaign copy across multiple formats.
///
/// For each requested format, calls the LLM with a tailored prompt and
/// returns all generated copies bundled in a `CopyJob`.
#[server(endpoint = "campaign-copy/generate")]
pub async fn generate_copy(
    topic: String,
    key_messages: Vec<String>,
    audience: String,
    tone: String,
    formats: Vec<String>,
    word_limits: Option<serde_json::Value>,
) -> Result<CopyJob, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if topic.trim().is_empty() {
        return Err(ServerFnError::new("Topic cannot be empty"));
    }
    if key_messages.is_empty() {
        return Err(ServerFnError::new("At least one key message is required"));
    }
    if formats.is_empty() {
        return Err(ServerFnError::new("At least one format is required"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let job_id = uuid::Uuid::new_v4().to_string();
    let mut results = Vec::new();

    // Generate copy for each requested format sequentially
    for format in &formats {
        let system_prompt = build_system_prompt(format, &tone, &audience);
        let word_limit = get_word_limit(&word_limits, format);
        let user_prompt = build_user_prompt(&topic, &key_messages, format, word_limit);

        let max_tokens = match format.as_str() {
            "social_post" => Some(512),
            "email" => Some(1024),
            "press_release" => Some(1500),
            "speech" => Some(2000),
            _ => Some(1024),
        };

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
            .generate(&messages, None, Some(0.7), max_tokens)
            .await
            .map_err(|e| ServerFnError::new(format!("LLM error for format '{format}': {e}")))?;

        let latency_ms = start.elapsed().as_millis() as i32;

        // Log LLM usage
        let _ = crate::infrastructure::log_llm_usage(
            pool,
            "campaign_copy",
            &state.llm_config.model,
            response.prompt_tokens,
            response.completion_tokens,
            latency_ms,
        )
        .await;

        results.push(GeneratedCopy {
            format: format.clone(),
            content: response.content,
        });

        tracing::info!(
            job_id = %job_id,
            format = %format,
            latency_ms = latency_ms,
            "Campaign copy generated"
        );
    }

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let job = CopyJob {
        id: job_id,
        request: CopyGenerationRequest {
            topic,
            key_messages,
            audience,
            tone,
            formats,
            word_limits,
        },
        results,
        created_at: Some(now),
    };

    Ok(job)
}

/// Regenerate a single copy variant with optional feedback.
///
/// Useful when the user wants to tweak one specific format without
/// regenerating all formats from scratch.
#[server(endpoint = "campaign-copy/regenerate-variant")]
pub async fn regenerate_variant(
    format: String,
    topic: String,
    key_messages: Vec<String>,
    audience: String,
    tone: String,
    feedback: Option<String>,
) -> Result<GeneratedCopy, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if topic.trim().is_empty() {
        return Err(ServerFnError::new("Topic cannot be empty"));
    }
    if format.trim().is_empty() {
        return Err(ServerFnError::new("Format cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let system_prompt = build_system_prompt(&format, &tone, &audience);
    let base_user_prompt = build_user_prompt(&topic, &key_messages, &format, None);

    let user_prompt = if let Some(ref fb) = feedback {
        format!(
            "{base_user_prompt}\n\n\
Additional feedback to incorporate in this revision:\n\
{fb}\n\n\
Please generate an improved version that addresses the feedback above."
        )
    } else {
        format!(
            "{base_user_prompt}\n\n\
Please generate a fresh alternative version with a different angle or approach."
        )
    };

    let max_tokens = match format.as_str() {
        "social_post" => Some(512),
        "email" => Some(1024),
        "press_release" => Some(1500),
        "speech" => Some(2000),
        _ => Some(1024),
    };

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
        .generate(&messages, None, Some(0.8), max_tokens)
        .await
        .map_err(|e| ServerFnError::new(format!("LLM error for variant regeneration: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    // Log LLM usage
    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "campaign_copy",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    tracing::info!(
        format = %format,
        has_feedback = feedback.is_some(),
        latency_ms = latency_ms,
        "Campaign copy variant regenerated"
    );

    Ok(GeneratedCopy {
        format,
        content: response.content,
    })
}

/// Get the history of copy generation jobs.
///
/// Currently returns an empty list as database persistence for copy jobs
/// has not been implemented yet. This endpoint exists to establish the API
/// contract for future persistence.
#[server(endpoint = "campaign-copy/history")]
pub async fn get_generation_history() -> Result<Vec<CopyJob>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;

    // Extract state to validate that the server context is available,
    // even though we don't use the DB yet.
    let _state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    // No persistence yet — return empty history.
    // Future implementation will query a `copy_jobs` table.
    Ok(Vec::new())
}
