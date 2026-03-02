use dioxus::prelude::*;

use axum::routing::get;
use axum::{middleware, Extension};
use tower_sessions::cookie::SameSite;
use tower_sessions::{cookie::Key, MemoryStore, SessionManagerLayer};

use crate::infrastructure::{
    auth_callback, auth_login, logout,
    config::{AppConfig, EmbeddingConfig, KeycloakConfig, LlmConfig, VectorStoreConfig},
    db::Database,
    middleware::auth::require_auth,
    state::{ServerState, ServerStateInner},
    PendingOAuthStore,
};

/// Start the Axum server with Dioxus fullstack, session management,
/// PostgreSQL, and Keycloak OAuth routes.
pub fn server_start(app: fn() -> Element) -> Result<(), super::Error> {
    tokio::runtime::Runtime::new()?.block_on(async move {
        dotenvy::dotenv().ok();

        // Load and leak config structs for 'static lifetime
        let app_config: &'static AppConfig = Box::leak(Box::new(AppConfig::from_env()?));
        let keycloak: &'static KeycloakConfig = Box::leak(Box::new(KeycloakConfig::from_env()?));
        let llm_config: &'static LlmConfig = Box::leak(Box::new(LlmConfig::from_env()?));
        let embedding_config: &'static EmbeddingConfig =
            Box::leak(Box::new(EmbeddingConfig::from_env()?));
        let vector_store_config: &'static VectorStoreConfig =
            Box::leak(Box::new(VectorStoreConfig::from_env()?));

        tracing::info!("Configuration loaded");

        // Connect to PostgreSQL
        let database_url = std::env::var("DATABASE_URL").map_err(|_| {
            crate::infrastructure::Error::ConfigError("Missing DATABASE_URL".into())
        })?;
        let db = Database::connect(&database_url).await?;

        // Build ServerState
        let server_state: ServerState = ServerStateInner {
            db,
            app_config,
            keycloak,
            llm_config,
            embedding_config,
            vector_store_config,
        }
        .into();

        // Session layer
        let key = Key::generate();
        let store = MemoryStore::default();
        let session = SessionManagerLayer::new(store)
            .with_secure(false)
            .with_same_site(SameSite::Lax)
            .with_expiry(tower_sessions::Expiry::OnInactivity(
                tower_sessions::cookie::time::Duration::hours(24),
            ))
            .with_signed(key);

        // Build router
        let addr = dioxus_cli_config::fullstack_address_or_localhost();
        let listener = tokio::net::TcpListener::bind(addr).await?;

        // Layers wrap in reverse order: session (outermost) -> auth middleware
        // -> extensions -> route handlers
        let router = axum::Router::new()
            .route("/auth", get(auth_login))
            .route("/auth/callback", get(auth_callback))
            .route("/logout", get(logout))
            .serve_dioxus_application(ServeConfig::new(), app)
            .layer(Extension(PendingOAuthStore::default()))
            .layer(Extension(server_state))
            .layer(middleware::from_fn(require_auth))
            .layer(session);

        tracing::info!("Serving at {addr}");
        axum::serve(listener, router.into_make_service()).await?;

        Ok(())
    })
}
