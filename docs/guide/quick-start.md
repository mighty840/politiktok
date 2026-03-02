# Quick Start

Get PolitikTok running locally in under 5 minutes.

## 1. Clone the Repository

```bash
git clone https://github.com/mighty840/politiktok.git
cd politiktok
```

## 2. Start Infrastructure Services

```bash
docker compose up -d
```

This starts PostgreSQL, Qdrant, Keycloak, Ollama, and SearXNG.

## 3. Run Database Migrations

```bash
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
```

## 4. Pull an LLM Model

```bash
ollama pull llama3.1:8b
```

## 5. Start the Development Server

```bash
dx serve --port 9000
```

Open `http://localhost:9000` in your browser.

## 6. Default Credentials

In development mode, PolitikTok uses mock authentication. You'll be automatically logged in as:

- **Name:** Dev User
- **Email:** dev@politiktok.local
- **Role:** admin

## What's Next?

- [Configuration](/configuration/environment-variables) — Customize environment variables
- [Docker Setup](/guide/docker) — Detailed Docker Compose reference
- [Architecture](/architecture/overview) — Understand the system design
- [Modules](/modules/volunteer-matching) — Explore the 26 AI modules
