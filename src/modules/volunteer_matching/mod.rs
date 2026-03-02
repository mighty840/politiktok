//! F01: Volunteer Matching & Retention Engine
//!
//! Matches volunteers to campaign tasks based on skills, availability, and
//! location. Tracks engagement to improve retention over time.

use dioxus::prelude::*;

use crate::models::{Assignment, Task, TaskSummary, Volunteer, VolunteerMatch, VolunteerSummary};

/// Create a new volunteer profile.
#[server(endpoint = "volunteer-matching/create-volunteer")]
pub async fn create_volunteer(
    name: String,
    email: String,
    phone: Option<String>,
    skills: Vec<String>,
    availability: serde_json::Value,
    location: Option<serde_json::Value>,
    tags: Vec<String>,
    bio: Option<String>,
) -> Result<Volunteer, ServerFnError> {
    use crate::infrastructure::{require_user, ServerState};
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let user = require_user().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();
    let id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"INSERT INTO volunteers (id, name, email, phone, skills, availability, location, tags, bio, status, churn_score, created_by)
         VALUES ($1::uuid, $2, $3, $4, $5::text[], $6, $7, $8::text[], $9, 'active', 0.0, $10::uuid)"#,
    )
    .bind(&id)
    .bind(&name)
    .bind(&email)
    .bind(&phone)
    .bind(&skills)
    .bind(&availability)
    .bind(&location)
    .bind(&tags)
    .bind(&bio)
    .bind(&user.sub)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let row = sqlx::query(
        r#"SELECT
            id::text,
            name, email, phone,
            skills,
            availability,
            location,
            tags,
            bio,
            status,
            COALESCE(churn_score, 0.0) AS churn_score,
            to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at,
            to_char(last_active, 'YYYY-MM-DD HH24:MI:SS') AS last_active
        FROM volunteers WHERE id::text = $1"#,
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    tracing::info!(volunteer_id = %id, name = %name, "Volunteer created");

    Ok(Volunteer {
        id: row.get::<String, _>("id"),
        name: row.get::<String, _>("name"),
        email: row.get::<String, _>("email"),
        phone: row.get::<Option<String>, _>("phone"),
        skills: row.get::<Vec<String>, _>("skills"),
        availability: row.get::<serde_json::Value, _>("availability"),
        location: row.get::<Option<serde_json::Value>, _>("location"),
        tags: row.get::<Vec<String>, _>("tags"),
        bio: row.get::<Option<String>, _>("bio"),
        status: row.get::<String, _>("status"),
        churn_score: row.get::<f64, _>("churn_score"),
        created_at: row.get::<Option<String>, _>("created_at"),
        last_active: row.get::<Option<String>, _>("last_active"),
    })
}

/// Update an existing volunteer profile.
#[server(endpoint = "volunteer-matching/update-volunteer")]
pub async fn update_volunteer(
    id: String,
    name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    skills: Option<Vec<String>>,
    availability: Option<serde_json::Value>,
    location: Option<serde_json::Value>,
    tags: Option<Vec<String>>,
    bio: Option<String>,
    status: Option<String>,
) -> Result<Volunteer, ServerFnError> {
    use crate::infrastructure::{require_user, ServerState};
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let _user = require_user().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    // Verify volunteer exists
    let exists: Option<String> = sqlx::query_scalar(
        "SELECT id::text FROM volunteers WHERE id::text = $1",
    )
    .bind(&id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    if exists.is_none() {
        return Err(ServerFnError::new("Volunteer not found"));
    }

    sqlx::query(
        r#"UPDATE volunteers SET
            name = COALESCE($2, name),
            email = COALESCE($3, email),
            phone = COALESCE($4, phone),
            skills = COALESCE($5::text[], skills),
            availability = COALESCE($6, availability),
            location = COALESCE($7, location),
            tags = COALESCE($8::text[], tags),
            bio = COALESCE($9, bio),
            status = COALESCE($10, status),
            last_active = NOW()
        WHERE id::text = $1"#,
    )
    .bind(&id)
    .bind(&name)
    .bind(&email)
    .bind(&phone)
    .bind(&skills)
    .bind(&availability)
    .bind(&location)
    .bind(&tags)
    .bind(&bio)
    .bind(&status)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let row = sqlx::query(
        r#"SELECT
            id::text,
            name, email, phone,
            skills,
            availability,
            location,
            tags,
            bio,
            status,
            COALESCE(churn_score, 0.0) AS churn_score,
            to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at,
            to_char(last_active, 'YYYY-MM-DD HH24:MI:SS') AS last_active
        FROM volunteers WHERE id::text = $1"#,
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    tracing::info!(volunteer_id = %id, "Volunteer updated");

    Ok(Volunteer {
        id: row.get::<String, _>("id"),
        name: row.get::<String, _>("name"),
        email: row.get::<String, _>("email"),
        phone: row.get::<Option<String>, _>("phone"),
        skills: row.get::<Vec<String>, _>("skills"),
        availability: row.get::<serde_json::Value, _>("availability"),
        location: row.get::<Option<serde_json::Value>, _>("location"),
        tags: row.get::<Vec<String>, _>("tags"),
        bio: row.get::<Option<String>, _>("bio"),
        status: row.get::<String, _>("status"),
        churn_score: row.get::<f64, _>("churn_score"),
        created_at: row.get::<Option<String>, _>("created_at"),
        last_active: row.get::<Option<String>, _>("last_active"),
    })
}

/// List volunteers with optional search, status filter, and skill filter.
#[server(endpoint = "volunteer-matching/list-volunteers")]
pub async fn list_volunteers(
    search: Option<String>,
    status_filter: Option<String>,
    skill_filter: Option<String>,
) -> Result<Vec<VolunteerSummary>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    let rows = sqlx::query(
        r#"SELECT
            id::text,
            name, email,
            skills,
            status,
            COALESCE(churn_score, 0.0) AS churn_score,
            to_char(last_active, 'YYYY-MM-DD HH24:MI:SS') AS last_active
        FROM volunteers
        WHERE ($1::text IS NULL OR name ILIKE '%' || $1 || '%' OR email ILIKE '%' || $1 || '%')
          AND ($2::text IS NULL OR status = $2)
          AND ($3::text IS NULL OR $3 = ANY(skills))
        ORDER BY last_active DESC NULLS LAST"#,
    )
    .bind(&search)
    .bind(&status_filter)
    .bind(&skill_filter)
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let volunteers: Vec<VolunteerSummary> = rows
        .iter()
        .map(|row| VolunteerSummary {
            id: row.get::<String, _>("id"),
            name: row.get::<String, _>("name"),
            email: row.get::<String, _>("email"),
            skills: row.get::<Vec<String>, _>("skills"),
            status: row.get::<String, _>("status"),
            churn_score: row.get::<f64, _>("churn_score"),
            last_active: row.get::<Option<String>, _>("last_active"),
        })
        .collect();

    Ok(volunteers)
}

/// Get a single volunteer by ID.
#[server(endpoint = "volunteer-matching/get-volunteer")]
pub async fn get_volunteer(id: String) -> Result<Volunteer, ServerFnError> {
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
            name, email, phone,
            skills,
            availability,
            location,
            tags,
            bio,
            status,
            COALESCE(churn_score, 0.0) AS churn_score,
            to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at,
            to_char(last_active, 'YYYY-MM-DD HH24:MI:SS') AS last_active
        FROM volunteers WHERE id::text = $1"#,
    )
    .bind(&id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
    .ok_or_else(|| ServerFnError::new("Volunteer not found"))?;

    Ok(Volunteer {
        id: row.get::<String, _>("id"),
        name: row.get::<String, _>("name"),
        email: row.get::<String, _>("email"),
        phone: row.get::<Option<String>, _>("phone"),
        skills: row.get::<Vec<String>, _>("skills"),
        availability: row.get::<serde_json::Value, _>("availability"),
        location: row.get::<Option<serde_json::Value>, _>("location"),
        tags: row.get::<Vec<String>, _>("tags"),
        bio: row.get::<Option<String>, _>("bio"),
        status: row.get::<String, _>("status"),
        churn_score: row.get::<f64, _>("churn_score"),
        created_at: row.get::<Option<String>, _>("created_at"),
        last_active: row.get::<Option<String>, _>("last_active"),
    })
}

/// Get volunteers at risk of churning (churn_score > 0.7).
#[server(endpoint = "volunteer-matching/at-risk-volunteers")]
pub async fn get_at_risk_volunteers() -> Result<Vec<VolunteerSummary>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    let rows = sqlx::query(
        r#"SELECT
            id::text,
            name, email,
            skills,
            status,
            COALESCE(churn_score, 0.0) AS churn_score,
            to_char(last_active, 'YYYY-MM-DD HH24:MI:SS') AS last_active
        FROM volunteers
        WHERE COALESCE(churn_score, 0.0) > 0.7
        ORDER BY churn_score DESC"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let volunteers: Vec<VolunteerSummary> = rows
        .iter()
        .map(|row| VolunteerSummary {
            id: row.get::<String, _>("id"),
            name: row.get::<String, _>("name"),
            email: row.get::<String, _>("email"),
            skills: row.get::<Vec<String>, _>("skills"),
            status: row.get::<String, _>("status"),
            churn_score: row.get::<f64, _>("churn_score"),
            last_active: row.get::<Option<String>, _>("last_active"),
        })
        .collect();

    Ok(volunteers)
}

/// Create a new task that volunteers can be assigned to.
#[server(endpoint = "volunteer-matching/create-task")]
pub async fn create_task(
    title: String,
    description: String,
    required_skills: Vec<String>,
    location: Option<serde_json::Value>,
    date_start: Option<String>,
    date_end: Option<String>,
    max_volunteers: i32,
) -> Result<Task, ServerFnError> {
    use crate::infrastructure::{require_user, ServerState};
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let user = require_user().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();
    let id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"INSERT INTO tasks (id, title, description, required_skills, location, date_start, date_end, max_volunteers, status, created_by)
         VALUES ($1::uuid, $2, $3, $4::text[], $5, $6::timestamptz, $7::timestamptz, $8, 'open', $9::uuid)"#,
    )
    .bind(&id)
    .bind(&title)
    .bind(&description)
    .bind(&required_skills)
    .bind(&location)
    .bind(&date_start)
    .bind(&date_end)
    .bind(max_volunteers)
    .bind(&user.sub)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let row = sqlx::query(
        r#"SELECT
            id::text,
            title, description,
            required_skills,
            location,
            to_char(date_start, 'YYYY-MM-DD HH24:MI:SS') AS date_start,
            to_char(date_end, 'YYYY-MM-DD HH24:MI:SS') AS date_end,
            max_volunteers,
            status,
            created_by::text,
            to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
        FROM tasks WHERE id::text = $1"#,
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    tracing::info!(task_id = %id, title = %title, "Task created");

    Ok(Task {
        id: row.get::<String, _>("id"),
        title: row.get::<String, _>("title"),
        description: row.get::<String, _>("description"),
        required_skills: row.get::<Vec<String>, _>("required_skills"),
        location: row.get::<Option<serde_json::Value>, _>("location"),
        date_start: row.get::<Option<String>, _>("date_start"),
        date_end: row.get::<Option<String>, _>("date_end"),
        max_volunteers: row.get::<i32, _>("max_volunteers"),
        status: row.get::<String, _>("status"),
        created_by: row.get::<Option<String>, _>("created_by"),
        created_at: row.get::<Option<String>, _>("created_at"),
    })
}

/// List tasks with optional status filter and search.
#[server(endpoint = "volunteer-matching/list-tasks")]
pub async fn list_tasks(
    status_filter: Option<String>,
    search: Option<String>,
) -> Result<Vec<TaskSummary>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    let rows = sqlx::query(
        r#"SELECT
            t.id::text,
            t.title,
            t.required_skills,
            t.status,
            to_char(t.date_start, 'YYYY-MM-DD HH24:MI:SS') AS date_start,
            COALESCE(
                (SELECT count(*)::int FROM assignments a WHERE a.task_id = t.id AND a.status = 'active'),
                0
            ) AS assigned_count,
            t.max_volunteers
        FROM tasks t
        WHERE ($1::text IS NULL OR t.status = $1)
          AND ($2::text IS NULL OR t.title ILIKE '%' || $2 || '%' OR t.description ILIKE '%' || $2 || '%')
        ORDER BY t.date_start ASC NULLS LAST"#,
    )
    .bind(&status_filter)
    .bind(&search)
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let tasks: Vec<TaskSummary> = rows
        .iter()
        .map(|row| TaskSummary {
            id: row.get::<String, _>("id"),
            title: row.get::<String, _>("title"),
            required_skills: row.get::<Vec<String>, _>("required_skills"),
            status: row.get::<String, _>("status"),
            date_start: row.get::<Option<String>, _>("date_start"),
            assigned_count: row.get::<i32, _>("assigned_count"),
            max_volunteers: row.get::<i32, _>("max_volunteers"),
        })
        .collect();

    Ok(tasks)
}

/// Get a single task by ID.
#[server(endpoint = "volunteer-matching/get-task")]
pub async fn get_task(id: String) -> Result<Task, ServerFnError> {
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
            title, description,
            required_skills,
            location,
            to_char(date_start, 'YYYY-MM-DD HH24:MI:SS') AS date_start,
            to_char(date_end, 'YYYY-MM-DD HH24:MI:SS') AS date_end,
            max_volunteers,
            status,
            created_by::text,
            to_char(created_at, 'YYYY-MM-DD HH24:MI:SS') AS created_at
        FROM tasks WHERE id::text = $1"#,
    )
    .bind(&id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
    .ok_or_else(|| ServerFnError::new("Task not found"))?;

    Ok(Task {
        id: row.get::<String, _>("id"),
        title: row.get::<String, _>("title"),
        description: row.get::<String, _>("description"),
        required_skills: row.get::<Vec<String>, _>("required_skills"),
        location: row.get::<Option<serde_json::Value>, _>("location"),
        date_start: row.get::<Option<String>, _>("date_start"),
        date_end: row.get::<Option<String>, _>("date_end"),
        max_volunteers: row.get::<i32, _>("max_volunteers"),
        status: row.get::<String, _>("status"),
        created_by: row.get::<Option<String>, _>("created_by"),
        created_at: row.get::<Option<String>, _>("created_at"),
    })
}

/// Find the best matching volunteers for a task based on skill overlap scoring.
///
/// Composite score = 0.6 * skills_overlap + 0.4 * availability_score
/// - Skills overlap: count of matching skills / max(required_skills.len(), 1)
/// - Availability: 1.0 if volunteer status is 'active', 0.5 otherwise
#[server(endpoint = "volunteer-matching/match-task")]
pub async fn match_task(
    task_id: String,
    top_n: Option<i32>,
) -> Result<Vec<VolunteerMatch>, ServerFnError> {
    use crate::infrastructure::ServerState;
    use crate::models::MatchScoreBreakdown;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();
    let limit = top_n.unwrap_or(10);

    // Fetch the task to get required_skills
    let task_row = sqlx::query(
        "SELECT required_skills FROM tasks WHERE id::text = $1",
    )
    .bind(&task_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
    .ok_or_else(|| ServerFnError::new("Task not found"))?;

    let required_skills: Vec<String> = task_row.get::<Vec<String>, _>("required_skills");
    let skill_count = required_skills.len().max(1) as f64;

    // Find volunteers with skill overlap, excluding those already assigned to this task
    let rows = sqlx::query(
        r#"SELECT
            v.id::text,
            v.name,
            v.email,
            v.skills,
            v.status,
            COALESCE(v.churn_score, 0.0) AS churn_score,
            to_char(v.last_active, 'YYYY-MM-DD HH24:MI:SS') AS last_active,
            COALESCE(
                array_length(
                    ARRAY(SELECT unnest(v.skills) INTERSECT SELECT unnest($1::text[])),
                    1
                ),
                0
            ) AS matching_skill_count
        FROM volunteers v
        WHERE v.id NOT IN (
            SELECT a.volunteer_id FROM assignments a
            WHERE a.task_id::text = $2 AND a.status = 'active'
        )
        ORDER BY matching_skill_count DESC, v.churn_score ASC
        LIMIT $3"#,
    )
    .bind(&required_skills)
    .bind(&task_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let matches: Vec<VolunteerMatch> = rows
        .iter()
        .map(|row| {
            let matching_count: i32 = row.get::<i32, _>("matching_skill_count");
            let status: String = row.get::<String, _>("status");

            let skills_score = matching_count as f64 / skill_count;
            let availability_score = if status == "active" { 1.0 } else { 0.5 };
            let composite_score = 0.6 * skills_score + 0.4 * availability_score;

            VolunteerMatch {
                volunteer: VolunteerSummary {
                    id: row.get::<String, _>("id"),
                    name: row.get::<String, _>("name"),
                    email: row.get::<String, _>("email"),
                    skills: row.get::<Vec<String>, _>("skills"),
                    status,
                    churn_score: row.get::<f64, _>("churn_score"),
                    last_active: row.get::<Option<String>, _>("last_active"),
                },
                score: composite_score,
                score_breakdown: MatchScoreBreakdown {
                    semantic: skills_score,
                    availability: availability_score,
                    proximity: 0.0,
                },
            }
        })
        .collect();

    // Re-sort by composite score descending (SQL sorts by matching_skill_count but
    // the composite score also includes availability)
    let mut matches = matches;
    matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    Ok(matches)
}

/// Assign a volunteer to a task.
#[server(endpoint = "volunteer-matching/assign-volunteer")]
pub async fn assign_volunteer(
    task_id: String,
    volunteer_id: String,
) -> Result<Assignment, ServerFnError> {
    use crate::infrastructure::{require_user, ServerState};
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let user = require_user().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    // Verify task exists
    let task_exists: Option<String> = sqlx::query_scalar(
        "SELECT id::text FROM tasks WHERE id::text = $1",
    )
    .bind(&task_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    if task_exists.is_none() {
        return Err(ServerFnError::new("Task not found"));
    }

    // Verify volunteer exists
    let vol_exists: Option<String> = sqlx::query_scalar(
        "SELECT id::text FROM volunteers WHERE id::text = $1",
    )
    .bind(&volunteer_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    if vol_exists.is_none() {
        return Err(ServerFnError::new("Volunteer not found"));
    }

    // Check for duplicate active assignment
    let dup: Option<String> = sqlx::query_scalar(
        "SELECT id::text FROM assignments WHERE task_id::text = $1 AND volunteer_id::text = $2 AND status = 'active'",
    )
    .bind(&task_id)
    .bind(&volunteer_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    if dup.is_some() {
        return Err(ServerFnError::new("Volunteer is already assigned to this task"));
    }

    // Check if task has room
    let assigned_count: i64 = sqlx::query_scalar(
        "SELECT count(*)::bigint FROM assignments WHERE task_id::text = $1 AND status = 'active'",
    )
    .bind(&task_id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let max_vol: i32 = sqlx::query_scalar(
        "SELECT max_volunteers FROM tasks WHERE id::text = $1",
    )
    .bind(&task_id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    if assigned_count >= max_vol as i64 {
        return Err(ServerFnError::new("Task has reached maximum volunteer capacity"));
    }

    let id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"INSERT INTO assignments (id, task_id, volunteer_id, assigned_by, status)
         VALUES ($1::uuid, $2::uuid, $3::uuid, $4::uuid, 'active')"#,
    )
    .bind(&id)
    .bind(&task_id)
    .bind(&volunteer_id)
    .bind(&user.sub)
    .execute(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    // Update volunteer last_active
    sqlx::query("UPDATE volunteers SET last_active = NOW() WHERE id::text = $1")
        .bind(&volunteer_id)
        .execute(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    let row = sqlx::query(
        r#"SELECT
            id::text,
            task_id::text,
            volunteer_id::text,
            assigned_by::text,
            to_char(assigned_at, 'YYYY-MM-DD HH24:MI:SS') AS assigned_at,
            status
        FROM assignments WHERE id::text = $1"#,
    )
    .bind(&id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

    tracing::info!(
        assignment_id = %id,
        task_id = %task_id,
        volunteer_id = %volunteer_id,
        "Volunteer assigned to task"
    );

    Ok(Assignment {
        id: row.get::<String, _>("id"),
        task_id: row.get::<String, _>("task_id"),
        volunteer_id: row.get::<String, _>("volunteer_id"),
        assigned_by: row.get::<Option<String>, _>("assigned_by"),
        assigned_at: row.get::<Option<String>, _>("assigned_at"),
        status: row.get::<String, _>("status"),
    })
}

/// Draft a personalized message for a volunteer using the LLM.
///
/// Message types:
/// - "outreach": Invite a volunteer to a specific task
/// - "retention": Re-engage an at-risk volunteer
/// - "thanks": Thank a volunteer for completing a task
#[server(endpoint = "volunteer-matching/draft-message")]
pub async fn draft_message(
    volunteer_id: String,
    message_type: String,
    task_id: Option<String>,
) -> Result<String, ServerFnError> {
    use crate::infrastructure::{LlmClient, ServerState};
    use crate::infrastructure::llm::LlmMessage;
    use dioxus::fullstack::FullstackContext;
    use sqlx::Row;

    let state: ServerState = FullstackContext::extract()
        .await
        .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

    let pool = state.db.pool();

    // Fetch volunteer details
    let vol_row = sqlx::query(
        r#"SELECT
            name, email, skills, bio, status,
            COALESCE(churn_score, 0.0) AS churn_score,
            to_char(last_active, 'YYYY-MM-DD HH24:MI:SS') AS last_active
        FROM volunteers WHERE id::text = $1"#,
    )
    .bind(&volunteer_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?
    .ok_or_else(|| ServerFnError::new("Volunteer not found"))?;

    let vol_name: String = vol_row.get("name");
    let vol_skills: Vec<String> = vol_row.get("skills");
    let vol_bio: Option<String> = vol_row.get("bio");
    let vol_status: String = vol_row.get("status");
    let vol_churn: f64 = vol_row.get("churn_score");
    let vol_last_active: Option<String> = vol_row.get("last_active");

    // Optionally fetch task details
    let task_context = if let Some(ref tid) = task_id {
        let task_row = sqlx::query(
            r#"SELECT
                title, description, required_skills,
                to_char(date_start, 'YYYY-MM-DD HH24:MI:SS') AS date_start,
                to_char(date_end, 'YYYY-MM-DD HH24:MI:SS') AS date_end
            FROM tasks WHERE id::text = $1"#,
        )
        .bind(tid)
        .fetch_optional(pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {e}")))?;

        if let Some(tr) = task_row {
            let title: String = tr.get("title");
            let description: String = tr.get("description");
            let req_skills: Vec<String> = tr.get("required_skills");
            let start: Option<String> = tr.get("date_start");
            let end: Option<String> = tr.get("date_end");
            format!(
                "\nTask: {title}\nDescription: {description}\nRequired Skills: {}\nDate: {} to {}",
                req_skills.join(", "),
                start.unwrap_or_else(|| "TBD".to_string()),
                end.unwrap_or_else(|| "TBD".to_string()),
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // Build the system prompt based on message type
    let system_prompt = match message_type.as_str() {
        "outreach" => concat!(
            "You are a campaign volunteer coordinator. Draft a warm, personalized message ",
            "inviting a volunteer to a specific task. Reference their skills and how they ",
            "match the task. Keep the tone friendly and professional. ",
            "The message should be 2-3 short paragraphs."
        ),
        "retention" => concat!(
            "You are a campaign volunteer coordinator. Draft a warm, personalized message ",
            "to re-engage a volunteer who may be losing interest. Acknowledge their past ",
            "contributions, express genuine appreciation, and gently encourage continued ",
            "involvement. Do NOT be pushy or guilt-trip. ",
            "The message should be 2-3 short paragraphs."
        ),
        "thanks" => concat!(
            "You are a campaign volunteer coordinator. Draft a heartfelt thank-you message ",
            "to a volunteer who completed a task. Be specific about their contribution and ",
            "its impact. The tone should be warm and genuinely grateful. ",
            "The message should be 2-3 short paragraphs."
        ),
        _ => return Err(ServerFnError::new(
            "Invalid message_type. Must be one of: outreach, retention, thanks"
        )),
    };

    let user_prompt = format!(
        "Volunteer Name: {vol_name}\n\
         Skills: {skills}\n\
         Bio: {bio}\n\
         Status: {vol_status}\n\
         Churn Score: {vol_churn:.2}\n\
         Last Active: {last_active}\n\
         {task_context}\n\n\
         Draft the {message_type} message now.",
        skills = vol_skills.join(", "),
        bio = vol_bio.unwrap_or_else(|| "Not provided".to_string()),
        last_active = vol_last_active.unwrap_or_else(|| "Unknown".to_string()),
    );

    let messages = vec![
        LlmMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
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
        .generate(&messages, None, Some(0.7), Some(512))
        .await
        .map_err(|e| ServerFnError::new(format!("LLM generation failed: {e}")))?;

    let latency_ms = start.elapsed().as_millis() as i32;

    // Log LLM usage
    let _ = crate::infrastructure::log_llm_usage(
        pool,
        "volunteer_matching",
        &state.llm_config.model,
        response.prompt_tokens,
        response.completion_tokens,
        latency_ms,
    )
    .await;

    tracing::info!(
        volunteer_id = %volunteer_id,
        message_type = %message_type,
        "Drafted volunteer message"
    );

    Ok(response.content)
}
