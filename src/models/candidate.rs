/// A candidate profile for briefings and coaching.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Candidate {
    pub id: String,
    pub name: String,
    pub role: Option<String>,
    pub district: Option<String>,
    pub bio: Option<String>,
    pub policy_positions: serde_json::Value,
    pub created_at: Option<String>,
}

/// An opponent profile for opposition research.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Opponent {
    pub id: String,
    pub name: String,
    pub party: Option<String>,
    pub district: Option<String>,
    pub bio: Option<String>,
    pub policy_positions: serde_json::Value,
    pub contradictions: Vec<Contradiction>,
    pub created_at: Option<String>,
}

/// A detected contradiction in an opponent's record.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Contradiction {
    pub statement_a: String,
    pub statement_b: String,
    pub topic: String,
    pub confidence: f64,
    pub source_a: Option<String>,
    pub source_b: Option<String>,
}
