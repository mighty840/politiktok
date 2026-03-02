# Configuration

PolitikTok loads its configuration from environment variables at server startup using `dotenvy`. A `.env` file in the project root is automatically loaded.

## Environment Variables Reference

### Application

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `APP_URL` | Yes | -- | Public URL of the application (e.g., `http://localhost:8080`) |
| `ENCRYPTION_KEY` | Yes | -- | Key used for encrypting sensitive data at rest |
| `AUTH_SECRET` | Yes | -- | Secret for signing session cookies |

### Database

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | Yes | -- | PostgreSQL connection string (e.g., `postgresql://politiktok:politiktok@localhost:5433/politiktok`) |

The database connection pool is configured with a maximum of 20 connections. Migrations in the `./migrations` directory are applied automatically on startup.

### LLM (Text Generation)

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `LLM_BASE_URL` | Yes | -- | Base URL for the OpenAI-compatible API (e.g., `http://localhost:11434/v1`) |
| `LLM_MODEL` | Yes | -- | Model name to use for generation (e.g., `llama3.1:8b`) |
| `LLM_TIMEOUT_SECS` | No | `120` | Request timeout in seconds |
| `LLM_MAX_RETRIES` | No | `3` | Number of retry attempts with exponential backoff |

The LLM client uses the `/chat/completions` endpoint and supports both streaming (SSE) and non-streaming responses. Any OpenAI-compatible API can be used -- Ollama, vLLM, llama.cpp, or a hosted provider.

### Embedding

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `EMBEDDING_BASE_URL` | Yes | -- | Base URL for the embedding API (e.g., `http://localhost:11434/v1`) |
| `EMBEDDING_MODEL` | Yes | -- | Embedding model name (e.g., `nomic-embed-text`) |

The embedding client calls the `/embeddings` endpoint. Embeddings are used by the RAG pipeline in the Policy Chatbot (F02) and Knowledge Base (F25) modules.

### Vector Store (Qdrant)

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `VECTOR_STORE_URL` | Yes | -- | Qdrant HTTP API URL (e.g., `http://localhost:6335`) |

Collections are created automatically when documents are first ingested. The default vector dimension is 1536 (matching OpenAI-compatible embedding models), but it adapts to the actual embedding size returned by the model.

### Keycloak (Authentication)

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `KEYCLOAK_URL` | Yes | -- | Keycloak server URL (e.g., `http://localhost:8081`) |
| `KEYCLOAK_REALM` | Yes | -- | Realm name (e.g., `politiktok`) |
| `KEYCLOAK_CLIENT_ID` | Yes | -- | OIDC client ID (e.g., `politiktok-app`) |
| `KEYCLOAK_CLIENT_SECRET` | No | `""` | Client secret (empty for public clients using PKCE) |

### SearXNG (Web Search)

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `SEARXNG_URL` | No | -- | SearXNG instance URL for web search features |

### External APIs (Optional)

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `MASTODON_INSTANCE_URL` | No | -- | Mastodon instance for social media monitoring |
| `MASTODON_ACCESS_TOKEN` | No | -- | Mastodon API token |
| `REDDIT_CLIENT_ID` | No | -- | Reddit API client ID |
| `REDDIT_CLIENT_SECRET` | No | -- | Reddit API client secret |

## Example `.env` File

```bash
# Database
DATABASE_URL=postgresql://politiktok:politiktok@localhost:5433/politiktok

# LLM Configuration
LLM_BASE_URL=http://localhost:11434/v1
LLM_MODEL=llama3.1:8b
LLM_TIMEOUT_SECS=120
LLM_MAX_RETRIES=3

# Embedding
EMBEDDING_BASE_URL=http://localhost:11434/v1
EMBEDDING_MODEL=nomic-embed-text

# Vector Store (Qdrant)
VECTOR_STORE_URL=http://localhost:6335

# Keycloak
KEYCLOAK_URL=http://localhost:8081
KEYCLOAK_REALM=politiktok
KEYCLOAK_CLIENT_ID=politiktok-app
KEYCLOAK_CLIENT_SECRET=

# Application
APP_URL=http://localhost:8080
ENCRYPTION_KEY=changeme-generate-a-real-key
AUTH_SECRET=changeme-generate-a-real-secret

# SearXNG
SEARXNG_URL=http://localhost:8889
```

## Configuration Loading

Configuration is loaded in `src/infrastructure/config.rs` through five config structs:

- `AppConfig` -- application URL and secrets
- `KeycloakConfig` -- OIDC endpoints and client credentials
- `LlmConfig` -- text generation settings
- `EmbeddingConfig` -- embedding API settings
- `VectorStoreConfig` -- Qdrant connection

Each struct is loaded from environment variables at startup and leaked into `'static` references for zero-cost access throughout the application lifetime. Missing required variables cause the server to exit immediately with a descriptive error message.

## Security Notes

- Never commit `.env` to version control. The `.gitignore` should exclude it.
- Use strong, randomly generated values for `ENCRYPTION_KEY` and `AUTH_SECRET` in production.
- When using Keycloak with a public client (PKCE flow), `KEYCLOAK_CLIENT_SECRET` can remain empty.
- The `DATABASE_URL` contains credentials -- treat it as a secret.
