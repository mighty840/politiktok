/// An ingested document in the knowledge base.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub source_path: Option<String>,
    pub content_hash: Option<String>,
    pub collection_name: String,
    pub chunk_count: i32,
    pub tags: Vec<String>,
    pub ingested_at: Option<String>,
    pub status: String,
}

/// Chat session for conversational interfaces.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub user_id: Option<String>,
    pub session_type: String,
    pub created_at: Option<String>,
    pub last_active: Option<String>,
    pub metadata: serde_json::Value,
}

/// A single message in a chat session.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub sources: Option<serde_json::Value>,
    pub created_at: Option<String>,
}
