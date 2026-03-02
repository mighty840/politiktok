/// A task that can be assigned to volunteers.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub required_skills: Vec<String>,
    pub location: Option<serde_json::Value>,
    pub date_start: Option<String>,
    pub date_end: Option<String>,
    pub max_volunteers: i32,
    pub status: String,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
}

/// Task summary for list views.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaskSummary {
    pub id: String,
    pub title: String,
    pub required_skills: Vec<String>,
    pub status: String,
    pub date_start: Option<String>,
    pub assigned_count: i32,
    pub max_volunteers: i32,
}

/// Assignment linking a volunteer to a task.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Assignment {
    pub id: String,
    pub task_id: String,
    pub volunteer_id: String,
    pub assigned_by: Option<String>,
    pub assigned_at: Option<String>,
    pub status: String,
}
