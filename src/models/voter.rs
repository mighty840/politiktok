/// Voter segment for canvassing and outreach targeting.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VoterSegment {
    pub id: String,
    pub name: String,
    pub description: String,
    pub demographics: serde_json::Value,
    pub issues: Vec<String>,
    pub size_estimate: Option<i64>,
}
