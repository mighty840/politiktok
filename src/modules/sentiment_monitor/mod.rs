//! F03: Social Sentiment Monitor
//!
//! Monitors social media and public discourse to track sentiment trends
//! around candidates, policies, and political topics in real time.

use dioxus::prelude::*;

use crate::models::social_post::{SentimentSpike, SentimentSummary, SocialPost};

/// A new social post submitted for ingestion and classification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewSocialPost {
    pub source_platform: String,
    pub external_id: Option<String>,
    pub text: String,
    pub author_hash: Option<String>,
    pub posted_at: Option<String>,
}

/// Convert a time-window shorthand into a PostgreSQL interval string.
fn window_to_interval(window: &str) -> &'static str {
    match window {
        "1h" => "1 hour",
        "4h" => "4 hours",
        "7d" => "7 days",
        // Default to 24 hours for unknown windows.
        _ => "24 hours",
    }
}

/// Aggregate sentiment counts and trends for a given topic and time window.
#[server(endpoint = "sentiment-summary")]
pub async fn get_sentiment_summary(
    topic: Option<String>,
    window: String,
) -> Result<Vec<SentimentSummary>, ServerFnError> {
    let state: crate::infrastructure::ServerState =
        dioxus::fullstack::FullstackContext::extract().await?;
    let pool = state.db.pool();
    let interval = window_to_interval(&window);

    let rows = sqlx::query_as::<
        _,
        (
            String,         // topic
            Option<String>, // sentiment
            i64,            // cnt
            Option<f64>,    // avg_score
        ),
    >(
        r#"
        SELECT
            unnest(topics) AS topic,
            sentiment,
            count(*)::bigint AS cnt,
            avg(sentiment_score) AS avg_score
        FROM social_posts
        WHERE fetched_at >= now() - $1::interval
        GROUP BY topic, sentiment
        ORDER BY topic, sentiment
        "#,
    )
    .bind(interval)
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    // Filter by topic if provided.
    let filtered: Vec<_> = match &topic {
        Some(t) => rows.into_iter().filter(|(tp, _, _, _)| tp == t).collect(),
        None => rows,
    };

    // Group rows by topic and build summary structs.
    let mut summaries: std::collections::HashMap<String, SentimentSummary> =
        std::collections::HashMap::new();

    for (tp, sentiment, cnt, avg_score) in filtered {
        let entry = summaries
            .entry(tp.clone())
            .or_insert_with(|| SentimentSummary {
                topic: tp,
                window: window.clone(),
                positive_count: 0,
                negative_count: 0,
                neutral_count: 0,
                total_count: 0,
                avg_score: 0.0,
                trend: 0.0,
            });

        match sentiment.as_deref() {
            Some("positive") => entry.positive_count += cnt,
            Some("negative") => entry.negative_count += cnt,
            _ => entry.neutral_count += cnt,
        }

        entry.total_count += cnt;
        if let Some(score) = avg_score {
            // Running weighted average approximation.
            let prev_total = entry.total_count - cnt;
            if entry.total_count > 0 {
                entry.avg_score = (entry.avg_score * prev_total as f64 + score * cnt as f64)
                    / entry.total_count as f64;
            }
        }
    }

    // Compute a simple trend: (positive - negative) / total, clamped to [-1, 1].
    let mut result: Vec<SentimentSummary> = summaries
        .into_values()
        .map(|mut s| {
            if s.total_count > 0 {
                s.trend =
                    (s.positive_count as f64 - s.negative_count as f64) / s.total_count as f64;
            }
            s
        })
        .collect();

    result.sort_by(|a, b| b.total_count.cmp(&a.total_count));
    Ok(result)
}

/// Paginated feed of social posts, optionally filtered by topic and sentiment.
#[server(endpoint = "sentiment-feed")]
pub async fn get_sentiment_feed(
    topic: Option<String>,
    sentiment: Option<String>,
    limit: i64,
    offset: i64,
) -> Result<Vec<SocialPost>, ServerFnError> {
    let state: crate::infrastructure::ServerState =
        dioxus::fullstack::FullstackContext::extract().await?;
    let pool = state.db.pool();

    let rows = sqlx::query_as::<
        _,
        (
            String,                    // id
            String,                    // source_platform
            Option<String>,            // external_id
            String,                    // text
            Option<String>,            // author_hash
            Option<String>,            // posted_at
            Option<String>,            // fetched_at
            Option<String>,            // sentiment
            Option<f64>,               // sentiment_score
            Vec<String>,               // topics
            Option<serde_json::Value>, // location
            Option<serde_json::Value>, // coordination_flags
        ),
    >(
        r#"
        SELECT
            id::text,
            source_platform,
            external_id,
            text,
            author_hash,
            posted_at::text,
            fetched_at::text,
            sentiment,
            sentiment_score,
            topics,
            location,
            coordination_flags
        FROM social_posts
        WHERE ($1::text IS NULL OR $1 = ANY(topics))
          AND ($2::text IS NULL OR sentiment = $2)
        ORDER BY fetched_at DESC NULLS LAST
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(&topic)
    .bind(&sentiment)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let posts = rows
        .into_iter()
        .map(
            |(
                id,
                source_platform,
                external_id,
                text,
                author_hash,
                posted_at,
                fetched_at,
                sentiment,
                sentiment_score,
                topics,
                location,
                coordination_flags,
            )| {
                SocialPost {
                    id,
                    source_platform,
                    external_id,
                    text,
                    author_hash,
                    posted_at,
                    fetched_at,
                    sentiment,
                    sentiment_score,
                    topics,
                    location,
                    coordination_flags,
                }
            },
        )
        .collect();

    Ok(posts)
}

/// Return distinct topics found across all social posts.
#[server(endpoint = "sentiment-topics")]
pub async fn get_topics() -> Result<Vec<String>, ServerFnError> {
    let state: crate::infrastructure::ServerState =
        dioxus::fullstack::FullstackContext::extract().await?;
    let pool = state.db.pool();

    let rows = sqlx::query_as::<_, (String,)>(
        "SELECT DISTINCT unnest(topics) AS topic FROM social_posts ORDER BY topic",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    Ok(rows.into_iter().map(|(t,)| t).collect())
}

/// Return recently detected sentiment spikes.
#[server(endpoint = "sentiment-spikes")]
pub async fn get_spike_log() -> Result<Vec<SentimentSpike>, ServerFnError> {
    let state: crate::infrastructure::ServerState =
        dioxus::fullstack::FullstackContext::extract().await?;
    let pool = state.db.pool();

    let rows = sqlx::query_as::<
        _,
        (
            String,      // id
            String,      // topic
            String,      // sentiment
            f64,         // spike_magnitude
            Vec<String>, // sample_posts
            String,      // detected_at
        ),
    >(
        r#"
        SELECT
            id::text,
            topic,
            sentiment,
            spike_magnitude,
            sample_posts,
            detected_at::text
        FROM sentiment_spikes
        ORDER BY detected_at DESC
        LIMIT 20
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let spikes = rows
        .into_iter()
        .map(
            |(id, topic, sentiment, spike_magnitude, sample_posts, detected_at)| SentimentSpike {
                id,
                topic,
                sentiment,
                spike_magnitude,
                sample_posts,
                detected_at,
            },
        )
        .collect();

    Ok(spikes)
}

/// Classify a single post using the LLM, updating its sentiment and topics.
#[server(endpoint = "sentiment-classify")]
pub async fn classify_post(post_id: String) -> Result<SocialPost, ServerFnError> {
    let state: crate::infrastructure::ServerState =
        dioxus::fullstack::FullstackContext::extract().await?;
    let pool = state.db.pool();

    // Fetch the post text.
    let (text,): (String,) = sqlx::query_as("SELECT text FROM social_posts WHERE id::text = $1")
        .bind(&post_id)
        .fetch_one(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Post not found: {e}")))?;

    // Build LLM prompt for classification.
    let llm = crate::infrastructure::LlmClient::new(
        &state.llm_config.base_url,
        &state.llm_config.model,
        state.llm_config.timeout_secs,
        state.llm_config.max_retries,
    );

    let messages = vec![
        crate::infrastructure::LlmMessage {
            role: "system".into(),
            content: concat!(
                "You are a political sentiment classifier. ",
                "Given a social media post, respond with ONLY valid JSON: ",
                r#"{"sentiment": "positive"|"negative"|"neutral", "#,
                r#""score": <float -1.0 to 1.0>, "#,
                r#""topics": [<list of short topic strings>]}"#,
            )
            .into(),
        },
        crate::infrastructure::LlmMessage {
            role: "user".into(),
            content: text,
        },
    ];

    let start = std::time::Instant::now();
    let response = llm
        .generate(&messages, None, Some(0.1), Some(256))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM classification failed: {e}")))?;
    let latency_ms = start.elapsed().as_millis() as i32;

    // Log LLM usage.
    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "sentiment_monitor",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    // Parse the LLM response.
    let parsed: serde_json::Value = serde_json::from_str(&response.content)
        .map_err(|e| ServerFnError::new(format!("Failed to parse LLM response: {e}")))?;

    let sentiment = parsed["sentiment"]
        .as_str()
        .unwrap_or("neutral")
        .to_string();
    let score = parsed["score"].as_f64().unwrap_or(0.0);
    let topics: Vec<String> = parsed["topics"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    // Update the record.
    sqlx::query(
        r#"
        UPDATE social_posts
        SET sentiment = $1, sentiment_score = $2, topics = $3
        WHERE id::text = $4
        "#,
    )
    .bind(&sentiment)
    .bind(score)
    .bind(&topics)
    .bind(&post_id)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to update post: {e}")))?;

    // Return the updated post.
    let row = sqlx::query_as::<
        _,
        (
            String,
            String,
            Option<String>,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<f64>,
            Vec<String>,
            Option<serde_json::Value>,
            Option<serde_json::Value>,
        ),
    >(
        r#"
        SELECT
            id::text, source_platform, external_id, text, author_hash,
            posted_at::text, fetched_at::text, sentiment, sentiment_score,
            topics, location, coordination_flags
        FROM social_posts
        WHERE id::text = $1
        "#,
    )
    .bind(&post_id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to fetch updated post: {e}")))?;

    Ok(SocialPost {
        id: row.0,
        source_platform: row.1,
        external_id: row.2,
        text: row.3,
        author_hash: row.4,
        posted_at: row.5,
        fetched_at: row.6,
        sentiment: row.7,
        sentiment_score: row.8,
        topics: row.9,
        location: row.10,
        coordination_flags: row.11,
    })
}

/// Bulk insert new social posts and queue them for classification.
#[server(endpoint = "sentiment-ingest")]
pub async fn ingest_posts(posts: Vec<NewSocialPost>) -> Result<Vec<String>, ServerFnError> {
    let state: crate::infrastructure::ServerState =
        dioxus::fullstack::FullstackContext::extract().await?;
    let pool = state.db.pool();

    let mut inserted_ids = Vec::with_capacity(posts.len());

    for post in &posts {
        let id = uuid::Uuid::new_v4().to_string();

        sqlx::query(
            r#"
            INSERT INTO social_posts (id, source_platform, external_id, text, author_hash, posted_at, fetched_at)
            VALUES ($1::uuid, $2, $3, $4, $5, $6::timestamptz, now())
            "#,
        )
        .bind(&id)
        .bind(&post.source_platform)
        .bind(&post.external_id)
        .bind(&post.text)
        .bind(&post.author_hash)
        .bind(&post.posted_at)
        .execute(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Insert failed: {e}")))?;

        inserted_ids.push(id);
    }

    // Queue classification for each inserted post in the background.
    for id in &inserted_ids {
        let id = id.clone();
        tokio::spawn(async move {
            if let Err(e) = classify_post(id.clone()).await {
                tracing::warn!(post_id = %id, "Background classification failed: {e}");
            }
        });
    }

    tracing::info!(count = inserted_ids.len(), "Ingested social posts");
    Ok(inserted_ids)
}

/// Check whether there is a sentiment volume spike for a given topic.
///
/// A spike is detected when the current volume in the window exceeds
/// the historical average by at least `threshold` standard deviations.
#[server(endpoint = "sentiment-detect-spikes")]
pub async fn detect_spikes(
    topic: String,
    window_minutes: i32,
    threshold: f64,
) -> Result<Option<SentimentSpike>, ServerFnError> {
    let state: crate::infrastructure::ServerState =
        dioxus::fullstack::FullstackContext::extract().await?;
    let pool = state.db.pool();

    let interval = format!("{window_minutes} minutes");

    // Current window count.
    let (current_count,): (i64,) = sqlx::query_as(
        r#"
        SELECT count(*)::bigint
        FROM social_posts
        WHERE $1 = ANY(topics)
          AND fetched_at >= now() - $2::interval
        "#,
    )
    .bind(&topic)
    .bind(&interval)
    .fetch_one(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    // Historical average and stddev over the last 7 days, using same-sized windows.
    let (hist_avg, hist_stddev): (Option<f64>, Option<f64>) = sqlx::query_as(
        r#"
        WITH windows AS (
            SELECT
                floor(extract(epoch FROM now() - fetched_at) / ($2 * 60))::int AS bucket,
                count(*)::float8 AS cnt
            FROM social_posts
            WHERE $1 = ANY(topics)
              AND fetched_at >= now() - interval '7 days'
            GROUP BY bucket
        )
        SELECT avg(cnt), stddev(cnt) FROM windows
        "#,
    )
    .bind(&topic)
    .bind(window_minutes)
    .fetch_one(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let avg = hist_avg.unwrap_or(0.0);
    let stddev = hist_stddev.unwrap_or(1.0).max(1.0);
    let magnitude = (current_count as f64 - avg) / stddev;

    if magnitude >= threshold {
        // Fetch sample post IDs from the spike window.
        let samples = sqlx::query_as::<_, (String,)>(
            r#"
            SELECT id::text
            FROM social_posts
            WHERE $1 = ANY(topics)
              AND fetched_at >= now() - $2::interval
            ORDER BY fetched_at DESC
            LIMIT 5
            "#,
        )
        .bind(&topic)
        .bind(&interval)
        .fetch_all(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

        let sample_posts: Vec<String> = samples.into_iter().map(|(id,)| id).collect();

        // Persist the spike.
        let spike_id = uuid::Uuid::new_v4().to_string();
        let dominant_sentiment = determine_dominant_sentiment(pool, &topic, &interval).await;

        sqlx::query(
            r#"
            INSERT INTO sentiment_spikes (id, topic, sentiment, spike_magnitude, sample_posts, detected_at)
            VALUES ($1::uuid, $2, $3, $4, $5, now())
            "#,
        )
        .bind(&spike_id)
        .bind(&topic)
        .bind(&dominant_sentiment)
        .bind(magnitude)
        .bind(&sample_posts)
        .execute(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to record spike: {e}")))?;

        Ok(Some(SentimentSpike {
            id: spike_id,
            topic,
            sentiment: dominant_sentiment,
            spike_magnitude: magnitude,
            sample_posts,
            detected_at: chrono::Utc::now().to_rfc3339(),
        }))
    } else {
        Ok(None)
    }
}

/// Determine the dominant sentiment for a topic within a time interval.
#[cfg(feature = "server")]
async fn determine_dominant_sentiment(pool: &sqlx::PgPool, topic: &str, interval: &str) -> String {
    let result = sqlx::query_as::<_, (Option<String>,)>(
        r#"
        SELECT sentiment
        FROM social_posts
        WHERE $1 = ANY(topics)
          AND fetched_at >= now() - $2::interval
          AND sentiment IS NOT NULL
        GROUP BY sentiment
        ORDER BY count(*) DESC
        LIMIT 1
        "#,
    )
    .bind(topic)
    .bind(interval)
    .fetch_optional(pool)
    .await;

    match result {
        Ok(Some((Some(s),))) => s,
        _ => "neutral".to_string(),
    }
}
