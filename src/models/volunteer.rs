/// Volunteer profile with skills, availability, and engagement data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Volunteer {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub skills: Vec<String>,
    pub availability: serde_json::Value,
    pub location: Option<serde_json::Value>,
    pub tags: Vec<String>,
    pub bio: Option<String>,
    pub status: String,
    pub churn_score: f64,
    pub created_at: Option<String>,
    pub last_active: Option<String>,
}

/// Summary for volunteer list views.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VolunteerSummary {
    pub id: String,
    pub name: String,
    pub email: String,
    pub skills: Vec<String>,
    pub status: String,
    pub churn_score: f64,
    pub last_active: Option<String>,
}

/// Volunteer match result with relevance score.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VolunteerMatch {
    pub volunteer: VolunteerSummary,
    pub score: f64,
    pub score_breakdown: MatchScoreBreakdown,
}

/// Breakdown of matching score components.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MatchScoreBreakdown {
    pub semantic: f64,
    pub availability: f64,
    pub proximity: f64,
}
