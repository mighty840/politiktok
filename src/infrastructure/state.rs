use std::ops::Deref;
use std::sync::Arc;

use crate::infrastructure::{
    AppConfig, Database, EmbeddingConfig, KeycloakConfig, LlmConfig, VectorStoreConfig,
};

/// Server-side application state, cheaply cloneable via Arc.
#[derive(Clone, Debug)]
pub struct ServerState(Arc<ServerStateInner>);

impl Deref for ServerState {
    type Target = ServerStateInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ServerStateInner> for ServerState {
    fn from(inner: ServerStateInner) -> Self {
        Self(Arc::new(inner))
    }
}

/// Inner state holding all shared resources.
#[derive(Debug)]
pub struct ServerStateInner {
    pub db: Database,
    pub app_config: &'static AppConfig,
    pub keycloak: &'static KeycloakConfig,
    pub llm_config: &'static LlmConfig,
    pub embedding_config: &'static EmbeddingConfig,
    pub vector_store_config: &'static VectorStoreConfig,
}

/// User session state stored in signed cookies.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct UserSessionState {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub access_token: String,
    pub refresh_token: String,
}

/// Make ServerState extractable from Axum request parts.
impl<S> axum::extract::FromRequestParts<S> for ServerState
where
    S: Send + Sync,
{
    type Rejection = crate::infrastructure::Error;

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async {
            parts
                .extensions
                .get::<ServerState>()
                .cloned()
                .ok_or_else(|| {
                    crate::infrastructure::Error::InternalError(
                        "ServerState not found in extensions".into(),
                    )
                })
        }
    }
}
