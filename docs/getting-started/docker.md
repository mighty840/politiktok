# Docker Setup

PolitikTok uses Docker Compose to run its infrastructure dependencies. All services are defined in `docker-compose.yml` at the project root.

## Starting All Services

```bash
docker compose up -d
```

To stop all services:

```bash
docker compose down
```

To stop and remove all data volumes:

```bash
docker compose down -v
```

## Services Overview

### PostgreSQL

| Property | Value |
|----------|-------|
| Image | `postgres:16` |
| Container | `politiktok-postgres` |
| Host Port | **5433** |
| Container Port | 5432 |
| Database | `politiktok` |
| Username | `politiktok` |
| Password | `politiktok` |

PostgreSQL is the primary data store for all relational data -- users, volunteers, tasks, donors, social posts, chat messages, documents, compliance records, and more. The connection pool is configured for up to 20 concurrent connections.

Migrations are stored in the `./migrations` directory and run automatically when the server starts. The health check uses `pg_isready` with a 5-second interval.

**Connection string for local development:**

```
postgresql://politiktok:politiktok@localhost:5433/politiktok
```

Note the host port is **5433** (not the default 5432) to avoid conflicts with any local PostgreSQL installation.

### Qdrant

| Property | Value |
|----------|-------|
| Image | `qdrant/qdrant:latest` |
| Container | `politiktok-qdrant` |
| HTTP Port | **6335** (host) -> 6333 (container) |
| gRPC Port | **6336** (host) -> 6334 (container) |

Qdrant is the vector database used for semantic search in the RAG pipeline. It stores document chunk embeddings for:

- **Policy Chatbot (F02)** -- `policy_documents` collection
- **Knowledge Base (F25)** -- `knowledge_base` collection

Collections are created on-demand when documents are first ingested. The default distance metric is cosine similarity.

**Health check URL:** `http://localhost:6335/healthz`

### Keycloak

| Property | Value |
|----------|-------|
| Image | `quay.io/keycloak/keycloak:26.0` |
| Container | `politiktok-keycloak` |
| Host Port | **8081** |
| Container Port | 8080 |
| Admin User | `admin` |
| Admin Password | `admin` |

Keycloak handles authentication and authorization via OpenID Connect with PKCE. It runs in `start-dev` mode with realm auto-import.

The application uses four roles mapped from Keycloak realm roles:

| Role | Permissions |
|------|------------|
| `admin` | Full access to all modules and admin panel |
| `staff` | Access to all operational modules |
| `volunteer` | Limited access to volunteer-facing modules |
| `readonly` | View-only access |

Keycloak depends on PostgreSQL and will wait for it to be healthy before starting. The Keycloak database is stored in the same PostgreSQL instance.

**Admin console:** `http://localhost:8081`

### Ollama

| Property | Value |
|----------|-------|
| Image | `ollama/ollama` |
| Container | `politiktok-ollama` |
| Host Port | **11434** |
| Container Port | 11434 |

Ollama provides the local LLM inference endpoint. It exposes an OpenAI-compatible API at `http://localhost:11434/v1`. The container is configured with GPU passthrough for NVIDIA GPUs.

**Required models:**

```bash
# Text generation
docker exec politiktok-ollama ollama pull llama3.1:8b

# Embedding
docker exec politiktok-ollama ollama pull nomic-embed-text
```

You can substitute any Ollama-supported model by changing `LLM_MODEL` and `EMBEDDING_MODEL` in your `.env` file.

### SearXNG

| Property | Value |
|----------|-------|
| Image | `searxng/searxng:latest` |
| Container | `politiktok-searxng` |
| Host Port | **8889** |
| Container Port | 8080 |

SearXNG is a privacy-respecting metasearch engine used by modules that need to retrieve current information from the web -- such as the Regulatory Monitor (F20) and Media Monitor (F21).

**Search UI:** `http://localhost:8889`

## Network

All services are connected through the `politiktok-net` bridge network, allowing inter-container communication by service name (e.g., `postgres:5432` from within other containers).

## Persistent Volumes

| Volume | Service | Purpose |
|--------|---------|---------|
| `postgres-data` | PostgreSQL | Database files |
| `qdrant-data` | Qdrant | Vector storage |
| `ollama-data` | Ollama | Downloaded model weights |
| `searxng-data` | SearXNG | Configuration |
| `keycloak-data` | Keycloak | Realm data and themes |

Data persists across `docker compose down` and `docker compose up` cycles. Use `docker compose down -v` to remove all volumes and start fresh.

## Resource Requirements

| Service | RAM (minimum) | Notes |
|---------|--------------|-------|
| PostgreSQL | 256 MB | Increases with database size |
| Qdrant | 512 MB | Scales with number of stored vectors |
| Keycloak | 512 MB | JVM-based, may spike on startup |
| Ollama | 4 -- 16 GB | Depends on model size; 8B models need ~8 GB |
| SearXNG | 128 MB | Lightweight |

For development with an 8B parameter model, plan for at least 16 GB of available RAM total.
