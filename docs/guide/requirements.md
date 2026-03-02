# Requirements

Before installing PolitikTok, ensure your system meets the following requirements.

## System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | 2 cores | 4+ cores |
| RAM | 4 GB | 8+ GB |
| Disk | 10 GB | 20+ GB (models need space) |
| OS | Linux (x86_64) | Ubuntu 22.04+ / Fedora 39+ |

## Software Prerequisites

### Required

- **Rust** 1.82+ with `rustup`
- **Dioxus CLI** 0.7.3 (`dx`)
- **Docker** and **Docker Compose** (for services)
- **PostgreSQL** 16+ (or via Docker)

### Optional

- **Ollama** or any OpenAI-compatible LLM API
- **Qdrant** (for RAG/vector search features)
- **Keycloak** 24+ (for production authentication)
- **SearXNG** (for web search integration)
- **Node.js** 20+ (for building documentation)

## Supported Browsers

PolitikTok's web UI supports all modern browsers:

- Chrome / Chromium 90+
- Firefox 90+
- Safari 15+
- Edge 90+

## Network Requirements

| Service | Default Port | Purpose |
|---------|-------------|---------|
| PolitikTok | 9000 | Application |
| PostgreSQL | 5432 | Database |
| Qdrant | 6333/6334 | Vector store |
| Keycloak | 8080 | Auth provider |
| Ollama | 11434 | LLM API |
| SearXNG | 8888 | Web search |
