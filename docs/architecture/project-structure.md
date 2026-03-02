# Project Structure

This page describes the directory layout of the PolitikTok repository and explains the purpose of each major directory and file.

## Top-Level Layout

```
politiktok/
├── assets/                   # Static assets (CSS, images)
│   └── main.css              # Custom CSS beyond Tailwind/DaisyUI
├── bin/
│   └── main.rs               # Application entry point
├── docs/                     # mdBook documentation (this site)
│   ├── book.toml
│   └── src/
├── migrations/               # SQLx database migrations
│   └── *.sql
├── src/
│   ├── app.rs                # Root component, Route enum, layouts
│   ├── lib.rs                # Crate root, module declarations
│   ├── components/           # Reusable UI components
│   ├── infrastructure/       # Server-side infrastructure
│   ├── models/               # Shared data structures
│   ├── modules/              # 26 feature modules (server functions)
│   └── pages/                # Route-mapped page components
├── .env.example              # Example environment variables
├── .github/workflows/        # CI/CD pipelines
├── Cargo.toml                # Rust dependencies and features
├── Dioxus.toml               # Dioxus CLI configuration
└── docker-compose.yml        # Infrastructure services
```

## `bin/main.rs`

The entry point that dispatches to either the web (WASM) or server runtime based on the active feature flag:

- `#[cfg(feature = "web")]` -- launches Dioxus web with hydration enabled
- `#[cfg(feature = "server")]` -- calls `server_start()` to boot the Axum server

## `src/lib.rs`

Declares the five top-level modules and re-exports public items:

```rust
mod app;
mod components;
pub mod infrastructure;
mod models;
mod modules;
mod pages;
```

## `src/app.rs`

Contains three critical pieces:

1. **`App` component** -- root element that loads Tailwind CSS, DaisyUI, and the custom stylesheet, then renders the `Router`.
2. **`Route` enum** -- the full routing table with 40+ routes mapped to page components. Uses Dioxus `#[derive(Routable)]`.
3. **Layout components** -- `AppShell` (authenticated sidebar layout) and `AdminShell` (admin sub-navigation).

## `src/components/`

Reusable UI building blocks shared across pages:

| Component | File | Purpose |
|-----------|------|---------|
| `Sidebar` | `sidebar.rs` | Navigation sidebar with module links |
| `Header` | `header.rs` | Page header with breadcrumbs |
| `Card` | `card.rs` | Content card container |
| `Modal` | `modal.rs` | Dialog overlay |
| `DataTable` | `data_table.rs` | Sortable, filterable data table |
| `LoadingSpinner` | `loading.rs` | Loading state indicator |
| `StreamingText` | `streaming_text.rs` | Progressive text display for LLM output |
| `Chart` | `chart.rs` | Data visualization |
| `Pagination` | `pagination.rs` | Page navigation for lists |
| `Badge` | `badge.rs` | Status and tag badges |
| `Toast` | `toast.rs` | Notification toasts |
| `AlertBanner` | `alert_banner.rs` | Persistent alert messages |
| `FormFields` | `form_fields.rs` | Form input components |
| `Feedback` | `feedback.rs` | User feedback elements |

## `src/infrastructure/`

Server-side foundation code:

| File | Purpose |
|------|---------|
| `mod.rs` | Module declarations and re-exports |
| `config.rs` | Environment variable loading (`AppConfig`, `KeycloakConfig`, `LlmConfig`, `EmbeddingConfig`, `VectorStoreConfig`) |
| `db.rs` | PostgreSQL connection pool and migration runner |
| `state.rs` | `ServerState` (Arc-wrapped) and `UserSessionState` |
| `auth.rs` | Keycloak OIDC flow (login, callback, logout), PKCE helpers, session extraction |
| `auth_check.rs` | Authentication verification utilities |
| `llm.rs` | `LlmClient` with retry, streaming, and usage logging |
| `embedding.rs` | `EmbeddingClient`, text chunking, content hashing |
| `vector_store.rs` | `VectorStoreClient` for Qdrant operations |
| `error.rs` | Application error types |
| `server_start.rs` | Axum server bootstrap and middleware configuration |
| `middleware/mod.rs` | Middleware module |
| `middleware/auth.rs` | Auth enforcement middleware |
| `middleware/audit_log.rs` | Request audit logging middleware |

## `src/models/`

Shared data structures serializable with `serde`:

| File | Types |
|------|-------|
| `user.rs` | `User` |
| `roles.rs` | `Role` enum (Admin, Staff, Volunteer, ReadOnly) |
| `volunteer.rs` | `Volunteer`, `VolunteerSummary`, `VolunteerMatch`, `Assignment`, `Task`, `TaskSummary` |
| `task.rs` | Additional task-related types |
| `donor.rs` | `Donor` and donation records |
| `voter.rs` | `Voter` profiles |
| `document.rs` | `Document`, `ChatMessage`, `ChatSession` |
| `social_post.rs` | `SocialPost`, `SentimentSummary`, `SentimentSpike` |
| `candidate.rs` | `Candidate`, `Opponent`, `Contradiction` |

## `src/modules/`

Each of the 26 feature modules lives in its own subdirectory with a `mod.rs` file containing all server functions for that feature:

```
modules/
├── volunteer_matching/mod.rs     # F01
├── policy_chatbot/mod.rs         # F02
├── sentiment_monitor/mod.rs      # F03
├── campaign_copy/mod.rs          # F04
├── opposition_research/mod.rs    # F05
├── canvassing/mod.rs             # F06
├── fundraising/mod.rs            # F07
├── accountability/mod.rs         # F08
├── empathy_simulator/mod.rs      # F09
├── narrative_contagion/mod.rs    # F10
├── coalition_detector/mod.rs     # F11
├── candidate_briefing/mod.rs     # F12
├── call_intelligence/mod.rs      # F13
├── coaching/mod.rs               # F14
├── multilingual/mod.rs           # F15
├── question_anticipation/mod.rs  # F16
├── local_issues/mod.rs           # F17
├── policy_diff/mod.rs            # F18
├── faction_mapper/mod.rs         # F19
├── regulatory_monitor/mod.rs     # F20
├── media_monitor/mod.rs          # F21
├── disinfo_warning/mod.rs        # F22
├── compliance_reporting/mod.rs   # F23
├── meeting_summarizer/mod.rs     # F24
├── knowledge_base/mod.rs         # F25
└── admin/mod.rs                  # F26
```

## `src/pages/`

Page components that correspond to routes in `app.rs`:

```
pages/
├── landing.rs            # Public landing page (/)
├── login.rs              # Login page (/login)
├── not_found.rs          # 404 page
├── dashboard/mod.rs      # Main dashboard (/dashboard)
├── admin/mod.rs          # Admin panel pages (/admin/*)
├── volunteers/mod.rs     # F01 pages
├── policy_chatbot/mod.rs # F02 pages
├── sentiment/mod.rs      # F03 pages
├── ...                   # One directory per module
└── knowledge_base/mod.rs # F25 pages
```

## `migrations/`

SQL migration files run in order by `sqlx::migrate!()`. Each file creates or alters tables needed by the modules. Tables include: `volunteers`, `tasks`, `assignments`, `donors`, `donations`, `social_posts`, `sentiment_spikes`, `documents`, `chat_sessions`, `chat_messages`, `commitments`, `evidence`, `llm_usage_log`, and more.

## Configuration Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | Rust dependencies, feature flags (`server`, `web`), workspace lint configuration |
| `Dioxus.toml` | Dioxus CLI settings: app name, asset directory, watcher paths |
| `docker-compose.yml` | Infrastructure service definitions |
| `.env.example` | Template for environment variables |
| `.github/workflows/ci.yml` | CI pipeline (fmt, clippy, audit, test) |
| `.github/workflows/docs.yml` | Documentation deployment to GitHub Pages |
