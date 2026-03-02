/// A social media post captured for sentiment analysis.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SocialPost {
    pub id: String,
    pub source_platform: String,
    pub external_id: Option<String>,
    pub text: String,
    pub author_hash: Option<String>,
    pub posted_at: Option<String>,
    pub fetched_at: Option<String>,
    pub sentiment: Option<String>,
    pub sentiment_score: Option<f64>,
    pub topics: Vec<String>,
    pub location: Option<serde_json::Value>,
    pub coordination_flags: Option<serde_json::Value>,
}

/// Aggregated sentiment summary for a topic/window.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SentimentSummary {
    pub topic: String,
    pub window: String,
    pub positive_count: i64,
    pub negative_count: i64,
    pub neutral_count: i64,
    pub total_count: i64,
    pub avg_score: f64,
    pub trend: f64,
}

/// Detected sentiment spike event.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SentimentSpike {
    pub id: String,
    pub topic: String,
    pub sentiment: String,
    pub spike_magnitude: f64,
    pub sample_posts: Vec<String>,
    pub detected_at: String,
}
