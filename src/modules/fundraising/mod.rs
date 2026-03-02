//! F07: Intelligent Fundraising Assistant
//!
//! Optimizes fundraising strategy by analyzing donor patterns, suggesting
//! outreach timing, and generating personalized solicitation content.
//!
//! Provides CRUD operations for donors, donation recording with engagement
//! scoring, aggregate fundraising summaries, and LLM-powered solicitation
//! email drafting.

use dioxus::prelude::*;

use crate::models::donor::Donor;

/// Aggregate fundraising statistics returned by `get_fundraising_summary`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FundraisingSummary {
    pub total_donors: i64,
    pub total_raised: f64,
    pub average_donation: f64,
    pub top_donors: Vec<TopDonor>,
}

/// A top-donor entry used within the fundraising summary.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TopDonor {
    pub id: String,
    pub name: String,
    pub total_donated: f64,
}

/// System prompt for the solicitation email drafter.
const SOLICITATION_SYSTEM_PROMPT: &str = "\
You are an expert political fundraising copywriter. Your job is to draft a \
personalized, compelling fundraising solicitation email for a political campaign.

Guidelines:
- Be warm, personal, and authentic — avoid sounding like a form letter.
- Reference the donor's past giving history and engagement when provided.
- Clearly state the ask amount and tie it to a specific campaign goal or need.
- Keep the tone respectful and appreciative of past support.
- Include a clear call-to-action.
- Keep the email concise (under 300 words).
- Do NOT include subject line unless asked.";

// ---------------------------------------------------------------------------
// Server functions
// ---------------------------------------------------------------------------

/// Create a new donor record.
///
/// Stores name and email as-is (encryption is deferred to a future iteration).
/// Returns the newly created `Donor`.
#[server(endpoint = "fundraising/create-donor")]
pub async fn create_donor(
    name: String,
    email: String,
    tags: Vec<String>,
) -> Result<Donor, ServerFnError> {
    use crate::infrastructure::{require_user, ServerState};
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let _user = require_user()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();
    let id = uuid::Uuid::new_v4().to_string();

    let row = sqlx::query(
        r#"INSERT INTO donors (id, encrypted_name, encrypted_email, donation_history, engagement_score, tags, created_at)
           VALUES ($1::uuid, $2, $3, '[]'::jsonb, 0.0, $4, NOW())
           RETURNING
               id::text,
               encrypted_name,
               encrypted_email,
               donation_history,
               engagement_score,
               to_char(last_contact, 'YYYY-MM-DD HH24:MI:SS') AS last_contact,
               tags,
               to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at"#,
    )
    .bind(&id)
    .bind(&name)
    .bind(&email)
    .bind(&tags)
    .fetch_one(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    Ok(Donor {
        id: row.get::<String, _>("id"),
        encrypted_name: row.get::<Option<String>, _>("encrypted_name"),
        encrypted_email: row.get::<Option<String>, _>("encrypted_email"),
        donation_history: row.get::<serde_json::Value, _>("donation_history"),
        engagement_score: row.get::<f64, _>("engagement_score"),
        last_contact: row.get::<Option<String>, _>("last_contact"),
        tags: row.get::<Vec<String>, _>("tags"),
        created_at: row.get::<Option<String>, _>("created_at"),
    })
}

/// List donors with optional search on name and minimum engagement score filter.
#[server(endpoint = "fundraising/list-donors")]
pub async fn list_donors(
    search: Option<String>,
    min_score: Option<f64>,
) -> Result<Vec<Donor>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    let search_pattern = search
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| format!("%{s}%"));
    let min_score = min_score.unwrap_or(0.0);

    let rows = sqlx::query(
        r#"SELECT
            id::text,
            encrypted_name,
            encrypted_email,
            donation_history,
            engagement_score,
            to_char(last_contact, 'YYYY-MM-DD HH24:MI:SS') AS last_contact,
            COALESCE(tags, '{}') AS tags,
            to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
        FROM donors
        WHERE ($1::text IS NULL OR encrypted_name ILIKE $1)
          AND engagement_score >= $2
        ORDER BY engagement_score DESC, created_at DESC"#,
    )
    .bind(&search_pattern)
    .bind(min_score)
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let donors: Vec<Donor> = rows
        .iter()
        .map(|row| Donor {
            id: row.get::<String, _>("id"),
            encrypted_name: row.get::<Option<String>, _>("encrypted_name"),
            encrypted_email: row.get::<Option<String>, _>("encrypted_email"),
            donation_history: row.get::<serde_json::Value, _>("donation_history"),
            engagement_score: row.get::<f64, _>("engagement_score"),
            last_contact: row.get::<Option<String>, _>("last_contact"),
            tags: row.get::<Vec<String>, _>("tags"),
            created_at: row.get::<Option<String>, _>("created_at"),
        })
        .collect();

    Ok(donors)
}

/// Get a single donor by ID.
#[server(endpoint = "fundraising/get-donor")]
pub async fn get_donor(id: String) -> Result<Donor, ServerFnError> {
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
            encrypted_name,
            encrypted_email,
            donation_history,
            engagement_score,
            to_char(last_contact, 'YYYY-MM-DD HH24:MI:SS') AS last_contact,
            COALESCE(tags, '{}') AS tags,
            to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
        FROM donors
        WHERE id::text = $1"#,
    )
    .bind(&id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
    .ok_or_else(|| ServerFnError::new("Donor not found"))?;

    Ok(Donor {
        id: row.get::<String, _>("id"),
        encrypted_name: row.get::<Option<String>, _>("encrypted_name"),
        encrypted_email: row.get::<Option<String>, _>("encrypted_email"),
        donation_history: row.get::<serde_json::Value, _>("donation_history"),
        engagement_score: row.get::<f64, _>("engagement_score"),
        last_contact: row.get::<Option<String>, _>("last_contact"),
        tags: row.get::<Vec<String>, _>("tags"),
        created_at: row.get::<Option<String>, _>("created_at"),
    })
}

/// Record a donation for a donor.
///
/// Appends the donation entry to the `donation_history` JSONB array and
/// recalculates the `engagement_score` using:
///   score = (total_donations * 0.5 + recency_score * 0.3 + frequency_score * 0.2)
/// normalized to 0.0-1.0.
#[server(endpoint = "fundraising/record-donation")]
pub async fn record_donation(
    donor_id: String,
    amount: f64,
    date: String,
    note: String,
) -> Result<Donor, ServerFnError> {
    use crate::infrastructure::{require_user, ServerState};
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let _user = require_user()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    // Build the new donation entry
    let donation_entry = serde_json::json!({
        "amount": amount,
        "date": date,
        "note": note,
    });
    let donation_json = serde_json::to_string(&[&donation_entry])
        .map_err(|e| ServerFnError::new(format!("JSON error: {e}")))?;

    // Append donation to history and update last_contact in one query
    sqlx::query(
        r#"UPDATE donors
           SET donation_history = donation_history || $2::jsonb,
               last_contact = NOW()
           WHERE id::text = $1"#,
    )
    .bind(&donor_id)
    .bind(&donation_json)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    // Recalculate engagement_score from the updated donation_history.
    // We pull the full history, compute the score, and write it back.
    let row =
        sqlx::query(r#"SELECT donation_history, last_contact FROM donors WHERE id::text = $1"#)
            .bind(&donor_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
            .ok_or_else(|| ServerFnError::new("Donor not found"))?;

    let history: serde_json::Value = row.get("donation_history");
    let entries = history.as_array().cloned().unwrap_or_default();
    let count = entries.len() as f64;

    // Total donations component (normalized: sigmoid-like capping at ~$50k)
    let total: f64 = entries
        .iter()
        .filter_map(|e| e.get("amount").and_then(|a| a.as_f64()))
        .sum();
    let total_score = (total / 50_000.0).min(1.0);

    // Recency score: days since last donation, more recent = higher
    let recency_score = {
        let now = chrono::Utc::now();
        let mut most_recent_days = 365.0_f64;
        for entry in &entries {
            if let Some(d) = entry.get("date").and_then(|v| v.as_str()) {
                if let Ok(parsed) = chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                    let dt = parsed.and_hms_opt(0, 0, 0).unwrap();
                    let days = (now
                        - chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                            dt,
                            chrono::Utc,
                        ))
                    .num_days() as f64;
                    if days < most_recent_days {
                        most_recent_days = days;
                    }
                }
            }
        }
        // 0 days ago => 1.0, 365+ days => ~0.0
        (1.0 - (most_recent_days / 365.0)).max(0.0)
    };

    // Frequency score: number of donations normalized (cap at 20)
    let frequency_score = (count / 20.0).min(1.0);

    let engagement = total_score * 0.5 + recency_score * 0.3 + frequency_score * 0.2;
    let engagement = engagement.clamp(0.0, 1.0);

    sqlx::query("UPDATE donors SET engagement_score = $2 WHERE id::text = $1")
        .bind(&donor_id)
        .bind(engagement)
        .execute(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    // Return the updated donor
    get_donor(donor_id).await
}

/// Get an aggregate fundraising summary.
///
/// Returns total donors, total amount raised, average donation size,
/// and the top 5 donors by total donated.
#[server(endpoint = "fundraising/get-summary")]
pub async fn get_fundraising_summary() -> Result<FundraisingSummary, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    // Total donor count
    let total_donors: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM donors")
        .fetch_one(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    // Compute per-donor totals from the JSONB donation_history
    let rows = sqlx::query(
        r#"SELECT
            id::text,
            encrypted_name,
            donation_history
        FROM donors
        ORDER BY created_at DESC"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let mut grand_total = 0.0_f64;
    let mut donation_count = 0_u64;
    let mut donor_totals: Vec<(String, String, f64)> = Vec::new();

    for row in &rows {
        let id: String = row.get("id");
        let name: Option<String> = row.get("encrypted_name");
        let history: serde_json::Value = row.get("donation_history");

        let entries = history.as_array().cloned().unwrap_or_default();
        let donor_sum: f64 = entries
            .iter()
            .filter_map(|e| e.get("amount").and_then(|a| a.as_f64()))
            .sum();
        let count = entries.len() as u64;

        grand_total += donor_sum;
        donation_count += count;
        donor_totals.push((id, name.unwrap_or_else(|| "Unknown".to_string()), donor_sum));
    }

    let average_donation = if donation_count > 0 {
        grand_total / donation_count as f64
    } else {
        0.0
    };

    // Top 5 donors by total donated
    donor_totals.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    let top_donors: Vec<TopDonor> = donor_totals
        .into_iter()
        .take(5)
        .map(|(id, name, total_donated)| TopDonor {
            id,
            name,
            total_donated,
        })
        .collect();

    Ok(FundraisingSummary {
        total_donors,
        total_raised: grand_total,
        average_donation,
        top_donors,
    })
}

/// Draft a personalized fundraising solicitation email using the LLM.
///
/// Takes the donor's profile, a campaign context description, and a suggested
/// ask amount, then generates a ready-to-send email body.
#[server(endpoint = "fundraising/draft-solicitation")]
pub async fn draft_solicitation(
    donor_id: String,
    campaign_context: String,
    ask_amount: f64,
) -> Result<String, ServerFnError> {
    use crate::infrastructure::llm::LlmMessage;
    use crate::infrastructure::{require_user, LlmClient, ServerState};
    use dioxus::fullstack::FullstackContext;

    let _user = require_user()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    // Fetch donor details
    let donor = get_donor(donor_id).await?;

    let donor_name = donor
        .encrypted_name
        .as_deref()
        .unwrap_or("Valued Supporter");
    let history = donor.donation_history;
    let entries = history.as_array().cloned().unwrap_or_default();
    let total_given: f64 = entries
        .iter()
        .filter_map(|e| e.get("amount").and_then(|a| a.as_f64()))
        .sum();
    let num_donations = entries.len();
    let last_donation = entries.last().and_then(|e| {
        let amt = e.get("amount").and_then(|a| a.as_f64()).unwrap_or(0.0);
        let date = e.get("date").and_then(|d| d.as_str()).unwrap_or("unknown");
        Some(format!("${amt:.2} on {date}"))
    });

    let tags_str = donor.tags.join(", ");

    let user_prompt = format!(
        "Draft a fundraising solicitation email with the following details:\n\n\
         Donor Name: {donor_name}\n\
         Past Donations: {num_donations} donations totaling ${total_given:.2}\n\
         Most Recent Donation: {}\n\
         Engagement Score: {:.0}%\n\
         Donor Tags: {tags_str}\n\
         Last Contact: {}\n\n\
         Campaign Context: {campaign_context}\n\n\
         Suggested Ask Amount: ${ask_amount:.2}\n\n\
         Please draft the email body now.",
        last_donation.as_deref().unwrap_or("None"),
        donor.engagement_score * 100.0,
        donor.last_contact.as_deref().unwrap_or("Never"),
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: SOLICITATION_SYSTEM_PROMPT.to_string(),
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
        .generate(&messages, None, Some(0.7), Some(1024))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    // Log LLM usage
    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "fundraising",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    tracing::info!(
        donor_id = %donor.id,
        ask_amount = ask_amount,
        latency_ms = latency_ms,
        "Solicitation email drafted"
    );

    Ok(response.content)
}
