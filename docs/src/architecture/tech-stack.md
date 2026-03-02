# Tech Stack

## Core Technologies

| Category | Technology | Version | Role |
|----------|-----------|---------|------|
| Language | **Rust** | 1.89+ | Systems programming language for both server and client |
| Framework | **Dioxus** | 0.7.3 | Fullstack web framework with SSR, hydration, and WASM |
| Server | **Axum** | 0.8 | Async HTTP server framework built on Tower |
| Database | **PostgreSQL** | 16 | Primary relational data store |
| SQL Toolkit | **SQLx** | 0.8 | Async, compile-time checked SQL queries |
| Vector Database | **Qdrant** | latest | Vector similarity search for RAG pipelines |
| Authentication | **Keycloak** | 26.0 | OpenID Connect identity provider |
| LLM Runtime | **Ollama** | latest | Local LLM inference with OpenAI-compatible API |
| Search Engine | **SearXNG** | latest | Privacy-respecting metasearch for web data retrieval |
| CSS Framework | **Tailwind CSS** | 3 | Utility-first CSS loaded via CDN |
| UI Components | **DaisyUI** | 4.12 | Component library on top of Tailwind |

## Rust Dependencies

### Shared (Server + Client)

| Crate | Purpose |
|-------|---------|
| `dioxus` | Core framework with `fullstack` and `router` features |
| `dioxus-logger` | Tracing-based logging integration |
| `dioxus-sdk` | Time and storage utilities |
| `dioxus-free-icons` | Bootstrap and Font Awesome icon sets |
| `serde` / `serde_json` | Serialization for server function arguments and API responses |
| `chrono` | Date and time handling with serde support |
| `uuid` | UUID v4 generation with serde and JS compatibility |
| `thiserror` | Ergonomic error type derivation |
| `tracing` | Structured logging |
| `pulldown-cmark` | Markdown to HTML rendering |

### Server-Only

| Crate | Purpose |
|-------|---------|
| `axum` | HTTP router, middleware, and request handling |
| `sqlx` | Async PostgreSQL driver with migration support |
| `reqwest` | HTTP client for calling Ollama, Qdrant, and external APIs |
| `tower-sessions` | Session management with signed cookies |
| `tower-http` | CORS and request tracing middleware |
| `tower` | Service abstractions for middleware composition |
| `tokio` | Async runtime with full features |
| `tokio-stream` | Stream utilities for SSE |
| `async-stream` | Macro for creating async streams (LLM streaming) |
| `dotenvy` | `.env` file loading |
| `secrecy` | Wrapper type that prevents accidental secret logging |
| `url` | URL parsing and manipulation |
| `rand` | Cryptographic random number generation (PKCE, CSRF) |
| `argon2` | Password hashing (optional internal auth) |
| `sha2` | SHA-256 for content deduplication and PKCE challenges |
| `base64` | Base64 encoding for PKCE and JWT handling |
| `futures` | Stream trait and combinators |

## Architecture Rationale

### Why Dioxus?

Dioxus provides a React-like developer experience in Rust with first-class fullstack support. Key advantages:

- **Single language**: Server and client share types, validation logic, and business rules without code generation or API schema definitions.
- **Server functions**: The `#[server]` macro generates API endpoints automatically, eliminating boilerplate for client-server communication.
- **SSR + hydration**: Pages render on the server for fast initial load and SEO, then hydrate on the client for SPA-like interactivity.
- **Type safety**: Rust's type system catches errors at compile time across the full stack.

### Why Axum?

Axum integrates naturally with Dioxus's server-side rendering and provides:

- Tower middleware ecosystem for auth, CORS, rate limiting
- Extension-based dependency injection for database pools and configuration
- Efficient async request handling via Tokio

### Why PostgreSQL + SQLx?

- PostgreSQL handles structured data (users, tasks, donors) and semi-structured data (JSONB for flexible fields like availability, location, metadata).
- SQLx provides compile-time SQL checking and avoids the complexity of a full ORM.
- Array types (`text[]`) are used for skills, tags, and topics -- leveraging PostgreSQL's native array operations.

### Why Qdrant?

- Purpose-built for vector similarity search with cosine distance.
- Simple HTTP API that does not require a dedicated Rust SDK.
- Supports payload filtering alongside vector search.
- Collections are created lazily, keeping the setup minimal.

### Why Keycloak?

- Battle-tested OIDC provider with support for PKCE (no client secret needed).
- Role-based access control via realm roles mapped to application permissions.
- Admin console for managing users without building custom user management.
- Supports social login providers for future extension.

### Why Ollama?

- Runs LLMs locally, ensuring campaign data never leaves the infrastructure.
- OpenAI-compatible API means the codebase works with any compatible provider (vLLM, llama.cpp, or even OpenAI itself) by changing one URL.
- GPU acceleration via NVIDIA container toolkit for production-grade inference speed.

### Why Tailwind CSS + DaisyUI?

- Tailwind's utility classes integrate well with Dioxus RSX (class strings in `rsx!` macros).
- DaisyUI provides pre-built component styles (buttons, cards, modals, tables) that match the dark theme used throughout the application.
- CDN delivery avoids build toolchain complexity for CSS.

## Feature Flags

The application uses Cargo feature flags to separate server and client code:

| Feature | Activates |
|---------|-----------|
| `server` | All server-only dependencies (Axum, SQLx, Reqwest, etc.) |
| `web` | Dioxus web rendering target |
| (default) | No features -- used for shared code compilation checks |

This separation ensures that server-only code (database queries, HTTP clients) is never compiled into the WASM binary, keeping the client bundle small.
