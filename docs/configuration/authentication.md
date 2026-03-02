# Authentication

PolitikTok uses **Keycloak** as its identity provider, implementing the **OpenID Connect Authorization Code flow with PKCE** (Proof Key for Code Exchange). This eliminates the need for a client secret in the browser, making the setup suitable for public-facing single-page applications.

## Authentication Flow

```
1. User clicks "Login"
       |
       v
2. Browser -> GET /auth?redirect_url=/dashboard
       |
       v
3. Server generates PKCE verifier + challenge + state
   Stores (state -> verifier, redirect_url) in PendingOAuthStore
       |
       v
4. Server redirects to Keycloak authorize endpoint
   with client_id, code_challenge (S256), state
       |
       v
5. User authenticates in Keycloak
       |
       v
6. Keycloak redirects to /auth/callback?code=...&state=...
       |
       v
7. Server looks up PendingOAuthStore by state
   Exchanges authorization code + code_verifier for tokens
       |
       v
8. Server fetches userinfo from Keycloak
   Extracts role from JWT realm_access.roles
       |
       v
9. Server creates session with UserSessionState
   Redirects user to original redirect_url
```

## PKCE Implementation

PKCE prevents authorization code interception attacks. The implementation is in `src/infrastructure/auth.rs`:

- **Code verifier**: 43-character base64url string generated from 32 random bytes
- **Code challenge**: SHA-256 hash of the verifier, base64url-encoded (S256 method)
- **State parameter**: 64-character hex string for CSRF protection

The pending OAuth entries are stored in an in-memory `PendingOAuthStore` (behind `Arc<RwLock<HashMap>>`), which survives Dioxus CLI proxy restarts better than cookie-based state.

## Session Management

After successful authentication, the server stores a `UserSessionState` in a signed session cookie:

```rust
pub struct UserSessionState {
    pub sub: String,           // Keycloak subject ID
    pub email: String,
    pub name: String,
    pub role: String,          // "admin", "staff", "volunteer", "readonly"
    pub access_token: String,
    pub refresh_token: String,
}
```

Sessions are managed by `tower-sessions` with:

- **Memory store** for session data
- **Signed cookies** using a randomly generated key
- **24-hour inactivity expiry**
- **SameSite=Lax** cookie policy

## Role-Based Access Control

PolitikTok defines four roles, mapped from Keycloak realm roles:

| Role | Access Level | Typical User |
|------|-------------|--------------|
| **Admin** | Full access to all modules + admin panel | Campaign manager, system administrator |
| **Staff** | All operational modules, no admin panel | Campaign staff, communications director |
| **Volunteer** | Volunteer-facing modules only | Field volunteers |
| **ReadOnly** | View-only access across modules | Observers, auditors |

Roles are extracted from the JWT access token's `realm_access.roles` claim. The extraction checks for roles in priority order: admin, staff, volunteer, readonly. If none match, the default role is `staff`.

### Enforcing Roles in Server Functions

Server functions use helper methods from `src/infrastructure/auth.rs`:

```rust
// Require any authenticated user
let user = require_user().await?;

// Require a specific role (admin always passes)
let user = require_role("staff").await?;

// Extract session without role check
let session = require_session().await?;
```

The `require_role` function grants access if the user's role matches the required role OR if the user is an admin.

## Keycloak Configuration

Keycloak endpoints are derived from the base URL and realm name:

| Endpoint | URL Pattern |
|----------|-------------|
| Authorization | `{KEYCLOAK_URL}/realms/{REALM}/protocol/openid-connect/auth` |
| Token | `{KEYCLOAK_URL}/realms/{REALM}/protocol/openid-connect/token` |
| Userinfo | `{KEYCLOAK_URL}/realms/{REALM}/protocol/openid-connect/userinfo` |
| Logout | `{KEYCLOAK_URL}/realms/{REALM}/protocol/openid-connect/logout` |

## Logout

The logout flow (`GET /logout`):

1. Flushes the server-side session
2. Redirects to Keycloak's logout endpoint with `post_logout_redirect_uri` set to `APP_URL`
3. Keycloak terminates its own session and redirects the user back to the landing page

## Auth Middleware

The Axum middleware in `src/infrastructure/middleware/auth.rs` runs on every request. For routes that require authentication, it checks for a valid session cookie. Unauthenticated requests to protected routes are redirected to `/auth`.

The OAuth routes (`/auth`, `/auth/callback`, `/logout`) and static assets are excluded from auth enforcement.

## Development Mode

During development without Keycloak running, the `check_auth` server function falls back to a mock authenticated user:

```rust
Ok(AuthInfo {
    authenticated: true,
    email: "dev@politiktok.local".to_string(),
    name: "Dev User".to_string(),
    role: "admin".to_string(),
})
```

This allows UI development without requiring the full Docker Compose stack. Remove this fallback before deploying to production.
