/// Donor record with encrypted PII and engagement tracking.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Donor {
    pub id: String,
    pub encrypted_name: Option<String>,
    pub encrypted_email: Option<String>,
    pub donation_history: serde_json::Value,
    pub engagement_score: f64,
    pub last_contact: Option<String>,
    pub tags: Vec<String>,
    pub created_at: Option<String>,
}
