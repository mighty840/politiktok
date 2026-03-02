use axum::extract::Extension;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_sessions::Session;

use crate::infrastructure::{ServerState, UserSessionState, LOGGED_IN_USER_SESS_KEY};

/// Audit logging middleware — logs all admin API actions.
pub async fn audit_log(
    Extension(state): Extension<ServerState>,
    session: Session,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let path = request.uri().path().to_string();
    let method = request.method().clone();

    // Only log API mutations (POST, PUT, DELETE, PATCH)
    if !path.starts_with("/api/") || method == axum::http::Method::GET {
        return next.run(request).await;
    }

    let user = session
        .get::<UserSessionState>(LOGGED_IN_USER_SESS_KEY)
        .await
        .ok()
        .flatten();

    let response = next.run(request).await;

    // Log asynchronously to avoid slowing down the response
    if let Some(user) = user {
        let pool = state.db.pool().clone();
        let status = response.status();
        tokio::spawn(async move {
            let _ = sqlx::query(
                "INSERT INTO audit_log (id, actor_id, actor_email, action_type, resource_type, change_summary, created_at)
                 VALUES ($1, NULL, $2, $3, $4, $5, NOW())"
            )
            .bind(uuid::Uuid::new_v4())
            .bind(&user.email)
            .bind(format!("{method}"))
            .bind(&path)
            .bind(serde_json::json!({ "status": status.as_u16() }))
            .execute(&pool)
            .await;
        });
    }

    response
}
