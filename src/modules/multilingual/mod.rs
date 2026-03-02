//! F15: Multilingual Outreach Automation
//!
//! Automates translation and cultural adaptation of campaign materials
//! for multilingual communities while preserving tone and intent.

use dioxus::prelude::*;

/// A translated piece of content with cultural adaptation notes.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Translation {
    pub id: String,
    pub source_text: String,
    pub source_language: String,
    pub target_language: String,
    pub translated_text: String,
    pub cultural_notes: Vec<String>,
    pub created_at: Option<String>,
}

/// A supported language entry.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SupportedLanguage {
    pub code: String,
    pub name: String,
}

/// Culturally adapted messaging result.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdaptedMessaging {
    pub original_text: String,
    pub adapted_text: String,
    pub target_culture: String,
    pub tone: String,
    pub adaptation_notes: Vec<String>,
}

/// Translate content from one language to another with cultural adaptation.
///
/// Uses the LLM to produce a culturally aware translation along with notes
/// about idioms, tone shifts, or cultural considerations.
#[server(endpoint = "multilingual/translate")]
pub async fn translate_content(
    source_text: String,
    source_language: String,
    target_language: String,
    context: Option<String>,
) -> Result<Translation, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if source_text.trim().is_empty() {
        return Err(ServerFnError::new("Source text cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let context_line = context
        .as_deref()
        .filter(|c| !c.trim().is_empty())
        .map(|c| format!("\nContext for the translation: {c}"))
        .unwrap_or_default();

    let system_prompt = format!(
        "You are an expert political campaign translator specializing in culturally \
         aware translations. You translate from {source_language} to {target_language} \
         while preserving political messaging tone and intent.\n\n\
         Your response MUST be valid JSON with exactly two keys:\n\
         - \"translated_text\": the full translated text\n\
         - \"cultural_notes\": an array of strings, each a brief note about cultural \
           adaptations, idiom changes, tone adjustments, or other considerations \
           made during translation.\n\n\
         Do NOT include any text outside the JSON object."
    );

    let user_prompt = format!(
        "Translate the following {source_language} text to {target_language}.\n\
         Preserve the political messaging tone and intent.\n\
         Adapt idioms and cultural references for the target audience.\n\
         {context_line}\n\n\
         Text to translate:\n{source_text}"
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
        .generate(&messages, None, Some(0.5), Some(2048))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM translation error: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "multilingual_translate",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    // Parse the JSON response
    let parsed: serde_json::Value = serde_json::from_str(&response.content)
        .map_err(|_| ServerFnError::new("Failed to parse translation response as JSON"))?;

    let translated_text = parsed
        .get("translated_text")
        .and_then(|v| v.as_str())
        .unwrap_or(&response.content)
        .to_string();

    let cultural_notes = parsed
        .get("cultural_notes")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let translation = Translation {
        id: uuid::Uuid::new_v4().to_string(),
        source_text,
        source_language,
        target_language,
        translated_text,
        cultural_notes,
        created_at: Some(now),
    };

    tracing::info!(
        id = %translation.id,
        latency_ms = latency_ms,
        "Translation completed"
    );

    Ok(translation)
}

/// Return the list of supported languages.
#[server(endpoint = "multilingual/languages")]
pub async fn get_supported_languages() -> Result<Vec<SupportedLanguage>, ServerFnError> {
    let languages = vec![
        SupportedLanguage {
            code: "en".to_string(),
            name: "English".to_string(),
        },
        SupportedLanguage {
            code: "es".to_string(),
            name: "Spanish".to_string(),
        },
        SupportedLanguage {
            code: "fr".to_string(),
            name: "French".to_string(),
        },
        SupportedLanguage {
            code: "zh".to_string(),
            name: "Mandarin".to_string(),
        },
        SupportedLanguage {
            code: "ar".to_string(),
            name: "Arabic".to_string(),
        },
        SupportedLanguage {
            code: "hi".to_string(),
            name: "Hindi".to_string(),
        },
        SupportedLanguage {
            code: "pt".to_string(),
            name: "Portuguese".to_string(),
        },
        SupportedLanguage {
            code: "de".to_string(),
            name: "German".to_string(),
        },
        SupportedLanguage {
            code: "ja".to_string(),
            name: "Japanese".to_string(),
        },
        SupportedLanguage {
            code: "ko".to_string(),
            name: "Korean".to_string(),
        },
    ];
    Ok(languages)
}

/// Culturally adapt messaging beyond simple translation.
///
/// Takes text and adapts it for a specific cultural context and tone,
/// accounting for cultural norms, values, and communication styles.
#[server(endpoint = "multilingual/adapt")]
pub async fn adapt_messaging(
    text: String,
    target_culture: String,
    tone: String,
) -> Result<AdaptedMessaging, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    if text.trim().is_empty() {
        return Err(ServerFnError::new("Text cannot be empty"));
    }
    if target_culture.trim().is_empty() {
        return Err(ServerFnError::new("Target culture cannot be empty"));
    }

    let pool = state.db.pool();
    let llm = LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let system_prompt = "You are an expert in cross-cultural political communication. You adapt \
         political messaging to resonate with specific cultural communities while \
         maintaining the core message.\n\n\
         Your response MUST be valid JSON with exactly two keys:\n\
         - \"adapted_text\": the culturally adapted version of the text\n\
         - \"adaptation_notes\": an array of strings explaining each cultural \
           adaptation made (e.g., value framing, communication style, references).\n\n\
         Do NOT include any text outside the JSON object."
        .to_string();

    let user_prompt = format!(
        "Adapt the following political messaging for the {target_culture} cultural context.\n\
         Desired tone: {tone}\n\n\
         Consider:\n\
         - Cultural values and priorities of the target community\n\
         - Communication norms and preferred rhetorical styles\n\
         - Relevant cultural references and metaphors\n\
         - Sensitivity to cultural taboos or hot-button issues\n\n\
         Original text:\n{text}"
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
        .generate(&messages, None, Some(0.7), Some(2048))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM adaptation error: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "multilingual_adapt",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    let parsed: serde_json::Value = serde_json::from_str(&response.content)
        .map_err(|_| ServerFnError::new("Failed to parse adaptation response as JSON"))?;

    let adapted_text = parsed
        .get("adapted_text")
        .and_then(|v| v.as_str())
        .unwrap_or(&response.content)
        .to_string();

    let adaptation_notes = parsed
        .get("adaptation_notes")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    tracing::info!(
        target_culture = %target_culture,
        tone = %tone,
        latency_ms = latency_ms,
        "Messaging adapted"
    );

    Ok(AdaptedMessaging {
        original_text: text,
        adapted_text,
        target_culture,
        tone,
        adaptation_notes,
    })
}
