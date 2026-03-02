# Installation

This guide walks you through setting up PolitikTok for local development.

## Prerequisites

Before starting, make sure the following tools are installed on your system:

| Tool | Minimum Version | Purpose |
|------|----------------|---------|
| **Rust** | 1.89+ | Compiler toolchain |
| **dx** (Dioxus CLI) | 0.7.3 | Build, serve, and hot-reload the application |
| **Docker** & **Docker Compose** | 20.10+ / 2.20+ | Run PostgreSQL, Qdrant, Keycloak, Ollama, SearXNG |
| **wasm32 target** | -- | WebAssembly compilation target for the frontend |

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

### Add the WebAssembly Target

```bash
rustup target add wasm32-unknown-unknown
```

### Install the Dioxus CLI

```bash
cargo install dioxus-cli@0.7.3
```

Verify the installation:

```bash
dx --version
# dioxus 0.7.3
```

## Clone the Repository

```bash
git clone https://github.com/your-org/politiktok.git
cd politiktok
```

## Start Infrastructure Services

PolitikTok depends on several services that run in Docker containers. Start them all with:

```bash
docker compose up -d
```

This launches PostgreSQL, Qdrant, Keycloak, Ollama, and SearXNG. See the [Docker Setup](./docker.md) page for details on each service and its ports.

Wait for all services to report healthy status:

```bash
docker compose ps
```

## Pull an Ollama Model

The application needs at least one LLM and one embedding model loaded in Ollama:

```bash
# Text generation model
docker exec politiktok-ollama ollama pull llama3.1:8b

# Embedding model
docker exec politiktok-ollama ollama pull nomic-embed-text
```

## Configure Environment Variables

Copy the example environment file and adjust as needed:

```bash
cp .env.example .env
```

The defaults in `.env.example` are configured for the Docker Compose setup and should work out of the box for local development. See [Configuration](/configuration/environment-variables) for details on every variable.

## Run Database Migrations

Migrations run automatically on server startup via `sqlx::migrate!("./migrations")`. No manual step is required -- the server applies pending migrations each time it starts.

## Start the Development Server

```bash
dx serve
```

This compiles both the server (`--features server`) and client (`--features web`) targets, starts the Axum server, and enables hot-reloading for RSX changes. The application will be available at:

```
http://localhost:8080
```

The `dx` CLI proxies requests and handles WASM asset serving automatically.

## Build for Production

```bash
dx build --release
```

This produces optimized server and client binaries in the `dist/` directory. The server binary embeds the WASM client assets and can be deployed as a single executable.

## Verify the Setup

1. Open `http://localhost:8080` in your browser.
2. You should see the PolitikTok landing page.
3. Click "Get Started" to be redirected to Keycloak for login.
4. Log in with the default Keycloak admin credentials (`admin` / `admin`), or create a new user in the `politiktok` realm.
5. After authentication, you will land on the dashboard.

## Troubleshooting

| Symptom | Likely Cause | Fix |
|---------|-------------|-----|
| `wasm32-unknown-unknown` compilation errors | Missing target | `rustup target add wasm32-unknown-unknown` |
| Database connection refused | PostgreSQL not running | `docker compose up -d postgres` |
| LLM requests timeout | No model loaded in Ollama | `docker exec politiktok-ollama ollama pull llama3.1:8b` |
| Keycloak redirect fails | Wrong `APP_URL` in `.env` | Ensure `APP_URL` matches the address `dx serve` is using |
| CORS errors in browser | Middleware ordering | Check that `tower-http` CORS layer is in the Axum router |
