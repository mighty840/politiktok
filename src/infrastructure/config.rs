use secrecy::SecretString;

use crate::infrastructure::Error;

/// Application-wide configuration loaded from environment variables.
#[derive(Debug)]
pub struct AppConfig {
    pub app_url: String,
    pub encryption_key: SecretString,
    pub auth_secret: SecretString,
}

impl AppConfig {
    /// Load application config from environment.
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            app_url: required_env("APP_URL")?,
            encryption_key: SecretString::from(required_env("ENCRYPTION_KEY")?),
            auth_secret: SecretString::from(required_env("AUTH_SECRET")?),
        })
    }
}

/// Keycloak OIDC configuration.
#[derive(Debug)]
pub struct KeycloakConfig {
    pub url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: SecretString,
}

impl KeycloakConfig {
    /// Load Keycloak config from environment.
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            url: required_env("KEYCLOAK_URL")?,
            realm: required_env("KEYCLOAK_REALM")?,
            client_id: required_env("KEYCLOAK_CLIENT_ID")?,
            client_secret: SecretString::from(optional_env("KEYCLOAK_CLIENT_SECRET")),
        })
    }

    /// Authorization endpoint URL.
    pub fn auth_endpoint(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/auth",
            self.url, self.realm
        )
    }

    /// Token endpoint URL.
    pub fn token_endpoint(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.url, self.realm
        )
    }

    /// Userinfo endpoint URL.
    pub fn userinfo_endpoint(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/userinfo",
            self.url, self.realm
        )
    }

    /// Logout endpoint URL.
    pub fn logout_endpoint(&self) -> String {
        format!(
            "{}/realms/{}/protocol/openid-connect/logout",
            self.url, self.realm
        )
    }
}

/// LLM service configuration.
#[derive(Debug)]
pub struct LlmConfig {
    pub base_url: String,
    pub model: String,
    pub timeout_secs: u64,
    pub max_retries: u32,
}

impl LlmConfig {
    /// Load LLM config from environment.
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            base_url: required_env("LLM_BASE_URL")?,
            model: required_env("LLM_MODEL")?,
            timeout_secs: optional_env("LLM_TIMEOUT_SECS")
                .parse()
                .unwrap_or(120),
            max_retries: optional_env("LLM_MAX_RETRIES").parse().unwrap_or(3),
        })
    }
}

/// Embedding service configuration.
#[derive(Debug)]
pub struct EmbeddingConfig {
    pub base_url: String,
    pub model: String,
}

impl EmbeddingConfig {
    /// Load embedding config from environment.
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            base_url: required_env("EMBEDDING_BASE_URL")?,
            model: required_env("EMBEDDING_MODEL")?,
        })
    }
}

/// Vector store (Qdrant) configuration.
#[derive(Debug)]
pub struct VectorStoreConfig {
    pub url: String,
}

impl VectorStoreConfig {
    /// Load vector store config from environment.
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            url: required_env("VECTOR_STORE_URL")?,
        })
    }
}

/// Load a required environment variable, returning an error if missing.
fn required_env(name: &str) -> Result<String, Error> {
    std::env::var(name).map_err(|_| Error::ConfigError(format!("Missing env var: {name}")))
}

/// Load an optional environment variable, returning empty string if missing.
fn optional_env(name: &str) -> String {
    std::env::var(name).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_env_missing() {
        let result = required_env("DEFINITELY_NOT_SET_12345");
        assert!(result.is_err());
    }
}
