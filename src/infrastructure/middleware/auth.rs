use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use tower_sessions::Session;

use crate::infrastructure::{UserSessionState, LOGGED_IN_USER_SESS_KEY};

/// Public API endpoints that don't require authentication.
const PUBLIC_API_ENDPOINTS: &[&str] = &["/api/check-auth"];

/// Auth middleware that gates `/api/` endpoints.
pub async fn require_auth(
    session: Session,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let path = request.uri().path().to_string();

    // Only gate API paths
    if !path.starts_with("/api/") {
        return next.run(request).await;
    }

    // Allow public endpoints
    for public in PUBLIC_API_ENDPOINTS {
        if path.starts_with(public) {
            return next.run(request).await;
        }
    }

    // Check session
    let authenticated = session
        .get::<UserSessionState>(LOGGED_IN_USER_SESS_KEY)
        .await
        .ok()
        .flatten()
        .is_some();

    if authenticated {
        next.run(request).await
    } else {
        (StatusCode::UNAUTHORIZED, "Authentication required").into_response()
    }
}
