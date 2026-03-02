//! F06: Canvassing Script Generator
//!
//! Creates dynamic canvassing scripts adapted to neighborhood demographics,
//! local issues, and voter concerns for door-to-door outreach.
//!
//! Uses LLM generation to produce structured scripts with opening, issue
//! discussion, objection handling, and closing sections.

use dioxus::prelude::*;

/// A complete canvassing script with multiple sections.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CanvassingScript {
    pub id: String,
    pub voter_segment: String,
    pub local_issues: Vec<String>,
    pub candidate_name: String,
    pub key_asks: Vec<String>,
    pub script_sections: Vec<ScriptSection>,
    pub created_at: Option<String>,
}

/// A single section within a canvassing script.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ScriptSection {
    pub section_type: String, // "opening", "issue_discussion", "objection_handling", "closing"
    pub title: String,
    pub content: String,
    pub talking_points: Vec<String>,
}

/// System prompt for the canvassing script generator LLM.
const SYSTEM_PROMPT: &str = "\
You are an expert political campaign strategist specializing in door-to-door canvassing \
and voter outreach. Your job is to generate structured, persuasive canvassing scripts \
that volunteers can use when knocking on doors.

The scripts should be conversational, empathetic, and focused on connecting with voters \
on issues they care about. Avoid overly political jargon. The tone should be friendly \
and respectful, acknowledging that the voter's time is valuable.

You MUST respond with valid JSON in the following format (no markdown fences, just raw JSON):
{
  \"sections\": [
    {
      \"section_type\": \"opening\",
      \"title\": \"Opening / Introduction\",
      \"content\": \"The full script text for this section.\",
      \"talking_points\": [\"Key point 1\", \"Key point 2\"]
    },
    {
      \"section_type\": \"issue_discussion\",
      \"title\": \"Issue: <issue name>\",
      \"content\": \"Script text discussing this issue.\",
      \"talking_points\": [\"Point 1\", \"Point 2\"]
    },
    {
      \"section_type\": \"objection_handling\",
      \"title\": \"Handling Common Objections\",
      \"content\": \"Script text for handling pushback.\",
      \"talking_points\": [\"If they say X, respond with Y\"]
    },
    {
      \"section_type\": \"closing\",
      \"title\": \"Closing / The Ask\",
      \"content\": \"Script text for wrapping up the conversation.\",
      \"talking_points\": [\"Key closing point 1\"]
    }
  ]
}

Include one \"issue_discussion\" section for EACH local issue provided. \
Always include exactly one \"opening\", one \"objection_handling\", and one \"closing\" section.";

/// Generate a canvassing script using LLM.
///
/// Produces a structured script with opening, per-issue discussion,
/// objection handling, and closing sections tailored to the voter segment.
#[server(endpoint = "canvassing/generate-script")]
pub async fn generate_script(
    voter_segment: String,
    local_issues: Vec<String>,
    candidate_name: String,
    key_asks: Vec<String>,
) -> Result<CanvassingScript, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    // Validate inputs
    if voter_segment.trim().is_empty() {
        return Err(ServerFnError::new("Voter segment is required"));
    }
    if candidate_name.trim().is_empty() {
        return Err(ServerFnError::new("Candidate name is required"));
    }
    if local_issues.is_empty() {
        return Err(ServerFnError::new("At least one local issue is required"));
    }

    let issues_list = local_issues
        .iter()
        .enumerate()
        .map(|(i, issue)| format!("{}. {}", i + 1, issue))
        .collect::<Vec<_>>()
        .join("\n");

    let asks_list = if key_asks.is_empty() {
        "- Vote for the candidate on election day".to_string()
    } else {
        key_asks
            .iter()
            .map(|ask| format!("- {}", ask))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let user_prompt = format!(
        "Generate a canvassing script with the following parameters:\n\n\
         Candidate Name: {candidate_name}\n\
         Target Voter Segment: {voter_segment}\n\n\
         Local Issues to Address:\n{issues_list}\n\n\
         Key Asks (what we want the voter to do):\n{asks_list}\n\n\
         Please generate a complete, natural-sounding door-to-door canvassing script \
         with all required sections. Make the script specific to the voter segment \
         and tailor the language accordingly."
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: SYSTEM_PROMPT.to_string(),
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
        .generate(&messages, None, Some(0.7), Some(2000))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    // Log LLM usage
    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "canvassing",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    // Parse the LLM response into structured sections
    let script_sections = parse_script_sections(&response.content, &local_issues);

    let script_id = uuid::Uuid::new_v4().to_string();

    let script = CanvassingScript {
        id: script_id,
        voter_segment,
        local_issues,
        candidate_name,
        key_asks,
        script_sections,
        created_at: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
    };

    tracing::info!(
        script_id = %script.id,
        segment = %script.voter_segment,
        sections = script.script_sections.len(),
        latency_ms = latency_ms,
        "Canvassing script generated"
    );

    Ok(script)
}

/// List saved canvassing scripts.
///
/// Returns an empty vec for now as database persistence is not yet implemented.
#[server(endpoint = "canvassing/list-scripts")]
pub async fn list_scripts() -> Result<Vec<CanvassingScript>, ServerFnError> {
    // No DB persistence yet — return empty list
    Ok(Vec::new())
}

/// Export a canvassing script as formatted plain text.
///
/// Takes the script data and formats it into a readable plain-text document
/// suitable for printing or sharing with canvassing volunteers.
#[server(endpoint = "canvassing/export-script-text")]
pub async fn export_script_text(script: CanvassingScript) -> Result<String, ServerFnError> {
    let mut output = String::new();

    // Header
    output.push_str(&format!(
        "========================================\n\
         CANVASSING SCRIPT\n\
         ========================================\n\n\
         Candidate: {}\n\
         Target Segment: {}\n\
         Local Issues: {}\n\
         Key Asks: {}\n",
        script.candidate_name,
        script.voter_segment,
        script.local_issues.join(", "),
        script.key_asks.join(", "),
    ));

    if let Some(ref created) = script.created_at {
        output.push_str(&format!("Generated: {}\n", created));
    }

    output.push_str("\n========================================\n\n");

    // Sections
    for (i, section) in script.script_sections.iter().enumerate() {
        if i > 0 {
            output.push_str("----------------------------------------\n\n");
        }

        let section_label = match section.section_type.as_str() {
            "opening" => "OPENING",
            "issue_discussion" => "ISSUE DISCUSSION",
            "objection_handling" => "OBJECTION HANDLING",
            "closing" => "CLOSING",
            other => other,
        };

        output.push_str(&format!(
            "[{}] {}\n\n{}\n\n",
            section_label, section.title, section.content,
        ));

        if !section.talking_points.is_empty() {
            output.push_str("Talking Points:\n");
            for point in &section.talking_points {
                output.push_str(&format!("  * {}\n", point));
            }
            output.push('\n');
        }
    }

    output.push_str("========================================\n");
    output.push_str("END OF SCRIPT\n");
    output.push_str("========================================\n");

    Ok(output)
}

// ---------------------------------------------------------------------------
// Internal helpers (server-only)
// ---------------------------------------------------------------------------

/// Parse the LLM JSON response into a Vec<ScriptSection>.
///
/// Attempts to parse structured JSON first. If that fails, falls back to
/// wrapping the raw text as a single "full_script" section.
#[cfg(feature = "server")]
fn parse_script_sections(raw: &str, local_issues: &[String]) -> Vec<ScriptSection> {
    // Try to extract JSON from the response (handle possible markdown fences)
    let json_str = extract_json(raw);

    if let Some(json_str) = json_str {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json_str) {
            if let Some(sections) = parsed.get("sections").and_then(|s| s.as_array()) {
                let result: Vec<ScriptSection> = sections
                    .iter()
                    .filter_map(|s| {
                        let section_type = s.get("section_type")?.as_str()?.to_string();
                        let title = s.get("title")?.as_str()?.to_string();
                        let content = s.get("content")?.as_str()?.to_string();
                        let talking_points = s
                            .get("talking_points")
                            .and_then(|tp| tp.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|v| v.as_str().map(String::from))
                                    .collect()
                            })
                            .unwrap_or_default();

                        Some(ScriptSection {
                            section_type,
                            title,
                            content,
                            talking_points,
                        })
                    })
                    .collect();

                if !result.is_empty() {
                    return result;
                }
            }
        }
    }

    // Fallback: wrap the entire raw response as a single section
    tracing::warn!("Failed to parse structured JSON from LLM response, using raw text fallback");
    build_fallback_sections(raw, local_issues)
}

/// Try to extract a JSON object from potentially markdown-fenced LLM output.
#[cfg(feature = "server")]
fn extract_json(raw: &str) -> Option<String> {
    let trimmed = raw.trim();

    // If it starts with '{', treat the whole thing as JSON
    if trimmed.starts_with('{') {
        return Some(trimmed.to_string());
    }

    // Try to extract from markdown code fences
    if let Some(start) = trimmed.find("```json") {
        let after_fence = &trimmed[start + 7..];
        if let Some(end) = after_fence.find("```") {
            return Some(after_fence[..end].trim().to_string());
        }
    }

    if let Some(start) = trimmed.find("```") {
        let after_fence = &trimmed[start + 3..];
        // Skip optional language identifier on the same line
        let after_lang = if let Some(nl) = after_fence.find('\n') {
            &after_fence[nl + 1..]
        } else {
            after_fence
        };
        if let Some(end) = after_lang.find("```") {
            let candidate = after_lang[..end].trim();
            if candidate.starts_with('{') {
                return Some(candidate.to_string());
            }
        }
    }

    // Last resort: find the first '{' and last '}'
    if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}')) {
        if end > start {
            return Some(trimmed[start..=end].to_string());
        }
    }

    None
}

/// Build fallback sections from raw text when JSON parsing fails.
#[cfg(feature = "server")]
fn build_fallback_sections(raw: &str, local_issues: &[String]) -> Vec<ScriptSection> {
    let mut sections = Vec::new();

    // Create a single comprehensive section with the full raw text
    sections.push(ScriptSection {
        section_type: "opening".to_string(),
        title: "Complete Canvassing Script".to_string(),
        content: raw.to_string(),
        talking_points: local_issues
            .iter()
            .map(|issue| format!("Discuss: {}", issue))
            .collect(),
    });

    sections
}
