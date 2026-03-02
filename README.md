<p align="center">
  <img src="assets/logo.svg" width="128" height="128" alt="PolitikTok Logo" />
</p>

<h1 align="center">PolitikTok</h1>

<p align="center">
  <strong>AI-powered political campaign operations platform</strong>
</p>

<p align="center">
  <a href="https://github.com/mighty840/politiktok/actions/workflows/ci.yml"><img src="https://github.com/mighty840/politiktok/actions/workflows/ci.yml/badge.svg?branch=main" alt="CI" /></a>
  <a href="https://github.com/mighty840/politiktok/actions/workflows/docs.yml"><img src="https://github.com/mighty840/politiktok/actions/workflows/docs.yml/badge.svg?branch=main" alt="Docs" /></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.89-orange?logo=rust&logoColor=white" alt="Rust" /></a>
  <a href="https://dioxuslabs.com/"><img src="https://img.shields.io/badge/Dioxus-0.7.3-blue?logo=webassembly&logoColor=white" alt="Dioxus" /></a>
  <a href="https://www.postgresql.org/"><img src="https://img.shields.io/badge/PostgreSQL-16-336791?logo=postgresql&logoColor=white" alt="PostgreSQL" /></a>
  <a href="https://qdrant.tech/"><img src="https://img.shields.io/badge/Qdrant-1.12-DC382D?logo=qdrant&logoColor=white" alt="Qdrant" /></a>
</p>

<p align="center">
  <a href="https://www.keycloak.org/"><img src="https://img.shields.io/badge/Keycloak-26-4D4D4D?logo=keycloak&logoColor=white" alt="Keycloak" /></a>
  <a href="https://tailwindcss.com/"><img src="https://img.shields.io/badge/Tailwind_CSS-3-06B6D4?logo=tailwindcss&logoColor=white" alt="Tailwind CSS" /></a>
  <a href="https://daisyui.com/"><img src="https://img.shields.io/badge/DaisyUI-4-5A0EF8?logo=daisyui&logoColor=white" alt="DaisyUI" /></a>
  <a href="https://ollama.com/"><img src="https://img.shields.io/badge/Ollama-LLM-000000?logo=ollama&logoColor=white" alt="Ollama" /></a>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-AGPL--3.0-blue" alt="License" /></a>
  <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20Docker-lightgrey?logo=linux&logoColor=white" alt="Platform" />
  <img src="https://img.shields.io/badge/Modules-26-blueviolet" alt="Modules" />
  <img src="https://img.shields.io/badge/PRs-Welcome-brightgreen" alt="PRs Welcome" />
</p>

---

## About

PolitikTok is a comprehensive GenAI platform for political party operations. It provides 26 integrated modules covering everything from volunteer matching and policy chatbots to opposition research, sentiment monitoring, and campaign compliance -- all powered by local LLMs for data sovereignty.

> **Why?** Political campaigns generate and consume massive amounts of data. PolitikTok brings AI capabilities in-house so parties can operate efficiently without relying on third-party cloud services that may compromise voter data privacy.

## Modules

| # | Module | Description |
|---|--------|-------------|
| F01 | **Volunteer Matching** | AI-powered volunteer-task matching with churn prediction |
| F02 | **Policy Chatbot** | RAG-based Q&A over party documents for citizens |
| F03 | **Sentiment Monitor** | Real-time social media sentiment analysis with spike detection |
| F04 | **Campaign Copy** | Multi-format campaign content generation (email, social, press) |
| F05 | **Opposition Research** | Automated briefing generation and contradiction detection |
| F06 | **Canvassing Scripts** | Dynamic door-to-door conversation scripts |
| F07 | **Fundraising Assistant** | Donor engagement scoring and solicitation drafting |
| F08 | **Accountability Engine** | Manifesto promise tracking with evidence classification |
| F09 | **Empathy Simulator** | Audience persona-based policy impact analysis |
| F10 | **Narrative Contagion** | Message spread modeling and virality prediction |
| F11 | **Coalition Detector** | Coalition tension analysis and stress scoring |
| F12 | **Candidate Briefings** | Auto-generated event and meeting preparation briefs |
| F13 | **Call Intelligence** | Constituent call transcription and theme extraction |
| F14 | **Coaching & Debate** | AI-powered debate rehearsal with pressure simulation |
| F15 | **Multilingual Outreach** | Translation and cultural adaptation of campaign materials |
| F16 | **Question Anticipation** | Predicted voter questions with preparation checklists |
| F17 | **Local Issues** | Hyper-local issue mapping and prioritization |
| F18 | **Policy Diff** | Semantic policy comparison and mutation testing |
| F19 | **Faction Mapper** | Internal faction consensus and tension mapping |
| F20 | **Regulatory Monitor** | Plain-language regulatory change alerts |
| F21 | **Media Monitor** | Media bias and coverage tracking |
| F22 | **Disinfo Warning** | Disinformation early warning system |
| F23 | **Compliance** | Electoral compliance reporting and audit trails |
| F24 | **Meeting Summarizer** | Meeting transcription, summary, and action tracking |
| F25 | **Knowledge Base** | Internal knowledge base Q&A |
| F26 | **Admin Panel** | System administration, user management, module config |

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | [Dioxus 0.7.3](https://dioxuslabs.com/) (fullstack SSR + hydration), Tailwind CSS 3, DaisyUI 4 |
| Backend | Axum 0.8, tower-sessions, Dioxus server functions |
| Database | PostgreSQL 16 (via sqlx) |
| Vector Store | Qdrant (semantic search, RAG) |
| Auth | Keycloak 26+ (OAuth2 + PKCE) |
| LLM | Ollama (OpenAI-compatible API) |
| Search | SearXNG (meta-search for news/social feeds) |

## Getting Started

### Prerequisites

- Rust 1.89+
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started) (`dx`)
- Docker & Docker Compose (for services)

### Setup

```bash
# Clone the repository
git clone https://github.com/mighty840/politiktok.git
cd politiktok

# Start external services
docker compose up -d

# Configure environment
cp .env.example .env
# Edit .env with your service URLs

# Run the dev server
dx serve --port 9000
```

### External Services

| Service | Purpose | Default URL |
|---------|---------|-------------|
| PostgreSQL | Relational data store | `localhost:5433` |
| Qdrant | Vector database for RAG | `localhost:6335` |
| Keycloak | Identity provider / SSO | `localhost:8081` |
| Ollama | Local LLM inference | `localhost:11434` |
| SearXNG | Meta-search engine | `localhost:8889` |

## Project Structure

```
src/
  app.rs              Root component, Router, auth shell
  lib.rs              Module declarations
  components/         Reusable UI components (sidebar, table, modal, etc.)
  pages/              Full page views for each module
  models/             Shared data models (web + server)
  infrastructure/     Server-side: auth, config, DB, LLM, vector store
  modules/            Per-module server logic and server functions
assets/               Static assets (CSS, icons, logo)
bin/                  Binary entrypoint
docs/                 Documentation (mdBook)
```

## Documentation

Full documentation is available at [https://mighty840.github.io/politiktok](https://mighty840.github.io/politiktok).

## Development

```bash
# Check server compilation
cargo check --features server --no-default-features

# Check web/WASM compilation
cargo check --features web --no-default-features

# Run tests
cargo test --features server --no-default-features

# Format code
cargo fmt

# Lint
cargo clippy --features server --no-default-features -- -D warnings
```

## Contributing

Contributions are welcome! Please read the contributing guidelines and submit pull requests.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes using [Conventional Commits](https://www.conventionalcommits.org/)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the GNU Affero General Public License v3.0 -- see the [LICENSE](LICENSE) file for details.

---

<p align="center">
  <sub>Built with Rust, Dioxus, and a commitment to democratic technology.</sub>
</p>
