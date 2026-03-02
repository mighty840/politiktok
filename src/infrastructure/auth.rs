use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use axum::extract::{Extension, Query};
use axum::response::Redirect;
use tower_sessions::Session;

use crate::infrastructure::{Error, ServerState, UserSessionState};

/// Session key for the logged-in user.
pub const LOGGED_IN_USER_SESS_KEY: &str = "logged-in-user";

/// In-memory store for pending OAuth PKCE flows.
/// Survives dx proxy drops unlike cookie-based state.
#[derive(Clone, Debug, Default)]
pub struct PendingOAuthStore(Arc<RwLock<HashMap<String, PendingOAuthEntry>>>);

impl PendingOAuthStore {
    /// Create a new empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a pending entry.
    pub async fn insert(&self, state: String, entry: PendingOAuthEntry) {
        self.0.write().await.insert(state, entry);
    }

    /// Remove and return a pending entry.
    pub async fn remove(&self, state: &str) -> Option<PendingOAuthEntry> {
        self.0.write().await.remove(state)
    }
}

/// Pending OAuth entry holding PKCE verifier and redirect target.
#[derive(Debug, Clone)]
pub struct PendingOAuthEntry {
    pub code_verifier: String,
    pub redirect_url: String,
}

/// Generate a PKCE code verifier (43-char base64url string from 32 random bytes).
pub fn generate_code_verifier() -> String {
    use base64::Engine;
    use rand::RngCore;

    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

/// Derive PKCE code challenge from verifier (S256).
pub fn derive_code_challenge(verifier: &str) -> String {
    use base64::Engine;
    use sha2::{Digest, Sha256};

    let hash = Sha256::digest(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash)
}

/// Generate a random state string for CSRF protection.
pub fn generate_state() -> String {
    use rand::RngCore;

    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Initiate OAuth login — redirect to Keycloak.
#[axum::debug_handler]
pub async fn auth_login(
    Extension(state): Extension<ServerState>,
    Extension(pending): Extension<PendingOAuthStore>,
    Query(params): Query<HashMap<String, String>>,
) -> Redirect {
    let redirect_url = params
        .get("redirect_url")
        .cloned()
        .unwrap_or_else(|| "/dashboard".into());

    let code_verifier = generate_code_verifier();
    let code_challenge = derive_code_challenge(&code_verifier);
    let oauth_state = generate_state();

    pending
        .insert(
            oauth_state.clone(),
            PendingOAuthEntry {
                code_verifier,
                redirect_url,
            },
        )
        .await;

    let auth_url = format!(
        "{}?client_id={}&response_type=code&scope=openid+email+profile&redirect_uri={}/auth/callback&state={}&code_challenge={}&code_challenge_method=S256",
        state.keycloak.auth_endpoint(),
        state.keycloak.client_id,
        state.app_config.app_url,
        oauth_state,
        code_challenge
    );

    Redirect::temporary(&auth_url)
}

/// Handle Keycloak callback — exchange code for tokens, create session.
#[axum::debug_handler]
pub async fn auth_callback(
    Extension(state): Extension<ServerState>,
    Extension(pending): Extension<PendingOAuthStore>,
    Query(params): Query<HashMap<String, String>>,
    session: Session,
) -> Result<Redirect, Error> {
    let code = params
        .get("code")
        .ok_or_else(|| Error::AuthError("Missing authorization code".into()))?;
    let oauth_state = params
        .get("state")
        .ok_or_else(|| Error::AuthError("Missing state parameter".into()))?;

    let entry = pending
        .remove(oauth_state)
        .await
        .ok_or_else(|| Error::AuthError("Invalid or expired state".into()))?;

    // Exchange code for tokens
    let client = reqwest::Client::new();
    let token_response = client
        .post(state.keycloak.token_endpoint())
        .form(&[
            ("grant_type", "authorization_code"),
            ("client_id", &state.keycloak.client_id),
            ("code", code),
            (
                "redirect_uri",
                &format!("{}/auth/callback", state.app_config.app_url),
            ),
            ("code_verifier", &entry.code_verifier),
        ])
        .send()
        .await
        .map_err(|e| Error::AuthError(format!("Token exchange failed: {e}")))?;

    if !token_response.status().is_success() {
        let body = token_response.text().await.unwrap_or_default();
        return Err(Error::AuthError(format!("Token exchange rejected: {body}")));
    }

    let tokens: serde_json::Value = token_response
        .json()
        .await
        .map_err(|e| Error::AuthError(format!("Invalid token response: {e}")))?;

    let access_token = tokens["access_token"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let refresh_token = tokens["refresh_token"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    // Fetch user info
    let userinfo = client
        .get(state.keycloak.userinfo_endpoint())
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|e| Error::AuthError(format!("Userinfo fetch failed: {e}")))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| Error::AuthError(format!("Invalid userinfo response: {e}")))?;

    let sub = userinfo["sub"].as_str().unwrap_or_default().to_string();
    let email = userinfo["email"].as_str().unwrap_or_default().to_string();
    let name = userinfo["preferred_username"]
        .as_str()
        .or_else(|| userinfo["name"].as_str())
        .unwrap_or_default()
        .to_string();

    // Extract role from realm roles
    let role = extract_role_from_token(&access_token).unwrap_or_else(|| "staff".into());

    let user_state = UserSessionState {
        sub,
        email,
        name,
        role,
        access_token,
        refresh_token,
    };

    session
        .insert(LOGGED_IN_USER_SESS_KEY, &user_state)
        .await
        .map_err(|e| Error::AuthError(format!("Session insert failed: {e}")))?;

    Ok(Redirect::temporary(&entry.redirect_url))
}

/// Logout — clear session and redirect to Keycloak logout.
#[axum::debug_handler]
pub async fn logout(
    Extension(state): Extension<ServerState>,
    session: Session,
) -> Result<Redirect, Error> {
    session.flush().await.map_err(|e| {
        Error::AuthError(format!("Session flush failed: {e}"))
    })?;

    let logout_url = format!(
        "{}?client_id={}&post_logout_redirect_uri={}",
        state.keycloak.logout_endpoint(),
        state.keycloak.client_id,
        state.app_config.app_url
    );

    Ok(Redirect::temporary(&logout_url))
}

/// Extract role from JWT access token (decode payload without verification).
fn extract_role_from_token(token: &str) -> Option<String> {
    use base64::Engine;

    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return None;
    }

    let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(parts[1])
        .ok()?;
    let claims: serde_json::Value = serde_json::from_slice(&payload).ok()?;

    // Check realm_access.roles for our app roles
    let roles = claims["realm_access"]["roles"].as_array()?;
    for role_name in &["admin", "staff", "volunteer", "readonly"] {
        if roles
            .iter()
            .any(|r| r.as_str() == Some(role_name))
        {
            return Some(role_name.to_string());
        }
    }

    Some("staff".into())
}

/// Helper: extract the current user session from a server function context.
pub async fn require_session() -> Result<UserSessionState, Error> {
    let session: Session = dioxus::fullstack::FullstackContext::extract()
        .await
        .map_err(|e| Error::AuthError(format!("Failed to extract session: {e:?}")))?;

    session
        .get::<UserSessionState>(LOGGED_IN_USER_SESS_KEY)
        .await
        .map_err(|e| Error::AuthError(format!("Session read failed: {e}")))?
        .ok_or_else(|| Error::AuthError("Not authenticated".into()))
}

/// Helper: require a specific role.
pub async fn require_role(required: &str) -> Result<UserSessionState, Error> {
    let user = require_session().await?;
    if user.role == required || user.role == "admin" {
        Ok(user)
    } else {
        Err(Error::AuthError(format!(
            "Insufficient permissions: requires {required}"
        )))
    }
}

/// Helper: require the current user, returning their state.
pub async fn require_user() -> Result<UserSessionState, Error> {
    require_session().await
}

// hex encoding helper (avoid adding a dep just for this)
mod hex {
    pub fn encode(bytes: [u8; 32]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkce_s256() {
        // RFC 7636 Appendix B test vector
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let challenge = derive_code_challenge(verifier);
        assert_eq!(challenge, "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM");
    }

    #[test]
    fn test_code_verifier_length() {
        let verifier = generate_code_verifier();
        assert_eq!(verifier.len(), 43);
    }

    #[test]
    fn test_state_length() {
        let state = generate_state();
        assert_eq!(state.len(), 64);
    }
}
