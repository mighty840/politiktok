# Architecture Overview

PolitikTok follows a **Dioxus fullstack** architecture where a single Rust codebase compiles to both a server binary and a WebAssembly client. The server renders pages on the first request (SSR) and the browser hydrates them into an interactive SPA.

## High-Level Architecture

```
                    +------------------+
                    |    Browser       |
                    |  (WASM + Hydrate)|
                    +--------+---------+
                             |
                     HTTP / WebSocket
                             |
                    +--------+---------+
                    |   Axum Server    |
                    |  (Dioxus SSR)    |
                    +--------+---------+
                             |
           +---------+-------+-------+---------+
           |         |               |         |
     +-----+----+ +--+---+   +------+---+ +---+------+
     |PostgreSQL | |Qdrant|   | Keycloak | | Ollama   |
     | (5433)    | |(6335)|   |  (8081)  | | (11434)  |
     +----------+ +------+   +----------+ +----------+
```

## Request Lifecycle

1. **Initial page load**: The browser requests a URL. The Axum server renders the matching Dioxus component tree to HTML and sends it along with the WASM bundle.

2. **Hydration**: The browser loads the WASM binary, which attaches event handlers to the server-rendered DOM without re-rendering. The page becomes interactive.

3. **Client-side navigation**: Subsequent page transitions happen entirely in WASM via the Dioxus router. No full-page reloads occur.

4. **Server function calls**: When a component needs backend data, it calls a `#[server]` function. Dioxus serializes the arguments, sends them as a POST request to a generated endpoint, and deserializes the response.

5. **Authentication check**: The Axum middleware layer verifies the session cookie on every request. Protected routes redirect unauthenticated users to the Keycloak login flow.

## Layered Architecture

The codebase is organized into four layers:

### Infrastructure Layer (`src/infrastructure/`)

Handles cross-cutting concerns that all modules depend on:

- **Database** (`db.rs`) -- PostgreSQL connection pool via `sqlx`, automatic migrations
- **Authentication** (`auth.rs`) -- Keycloak OIDC + PKCE flow, session management
- **LLM Client** (`llm.rs`) -- OpenAI-compatible API client with retry, streaming support
- **Embedding Client** (`embedding.rs`) -- Text embedding and chunking utilities
- **Vector Store** (`vector_store.rs`) -- Qdrant client for upsert, search, and delete
- **Config** (`config.rs`) -- Environment variable loading for all subsystems
- **State** (`state.rs`) -- Arc-wrapped shared state holding database pool and config references
- **Middleware** (`middleware/`) -- Axum middleware for auth enforcement and audit logging

### Models Layer (`src/models/`)

Shared data structures used across modules:

- `User`, `Volunteer`, `Task`, `Assignment` -- people and operations
- `Donor`, `Voter` -- fundraising and outreach
- `Document`, `ChatMessage`, `ChatSession` -- knowledge management
- `SocialPost`, `Candidate` -- intelligence and analysis
- `Role` -- role-based access control (Admin, Staff, Volunteer, ReadOnly)

### Modules Layer (`src/modules/`)

The 26 feature modules, each in its own subdirectory. Every module exposes its functionality exclusively through **Dioxus server functions** annotated with `#[server]`. Server functions:

- Run only on the server (behind `#[cfg(feature = "server")]`)
- Are callable from client components as regular async functions
- Automatically serialize/deserialize arguments and return values
- Have access to the Axum `Extension` layer for database, config, and auth

### Pages & Components Layer (`src/pages/`, `src/components/`)

UI code built with Dioxus RSX:

- **Pages** map 1:1 to routes defined in the `Route` enum in `app.rs`
- **Components** are reusable UI building blocks: `Sidebar`, `DataTable`, `Card`, `Modal`, `LoadingSpinner`, `StreamingText`, `Chart`, `Pagination`, `Badge`, `Toast`, `AlertBanner`, `FormFields`, `Feedback`
- **Layouts** provide nested structure: `AppShell` (main authenticated layout with sidebar) and `AdminShell` (admin sub-navigation)

## Server Startup Sequence

The server startup (`src/infrastructure/server_start.rs`) follows this order:

1. Load environment variables from `.env`
2. Initialize config structs and leak them to `'static` lifetime
3. Connect to PostgreSQL and run migrations
4. Build the `ServerState` with all config references
5. Configure the session layer (signed cookies, 24-hour expiry)
6. Mount OAuth routes (`/auth`, `/auth/callback`, `/logout`)
7. Attach Dioxus fullstack serving with SSR
8. Layer middleware: session -> auth enforcement -> extensions
9. Bind to the address from `dioxus-cli-config` and start serving

## Data Flow: Server Function Example

A typical module interaction (e.g., listing volunteers):

```
Browser Component
    |
    | calls list_volunteers(search, status_filter, skill_filter)
    |
    v
Dioxus Server Function
    |
    | extracts ServerState from Axum extensions
    | extracts session for auth check
    |
    v
SQLx Query
    |
    | executes parameterized SQL against PostgreSQL
    |
    v
Response serialized back to browser
```

For LLM-powered features, the flow extends through the LLM client:

```
Server Function
    |
    +-- Query PostgreSQL for context data
    |
    +-- Build system/user prompt
    |
    +-- Call LlmClient::generate() -> Ollama /chat/completions
    |
    +-- Log usage to llm_usage_log table
    |
    +-- Return generated content to browser
```
