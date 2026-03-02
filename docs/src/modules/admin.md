# F26 -- Admin Panel

Provides administrative controls for managing users, permissions, system configuration, and monitoring platform health. Access is restricted to users with the `admin` role.

## Key Features

- **User management**: View and manage user accounts, roles, and permissions.
- **Module configuration**: Enable/disable individual modules and configure module-specific settings.
- **LLM configuration**: Manage language model settings including model selection, temperature defaults, and rate limits.
- **Source management**: Configure external data sources for monitoring modules.
- **Knowledge base admin**: Manage document ingestion across all collections.
- **System health**: Real-time health checks for all infrastructure services (PostgreSQL, Qdrant, Ollama, Keycloak).
- **Audit log**: Browse the audit trail of administrative actions and module usage.
- **Alert management**: Configure and review system alerts and notifications.
- **Data governance**: Manage data retention policies and privacy controls.
- **Integration management**: Configure external API integrations (Mastodon, Reddit, etc.).

## Admin Routes

The admin panel has its own layout (`AdminShell`) with a dedicated navigation bar:

| Route | Page | Description |
|-------|------|-------------|
| `/admin` | Dashboard | Admin overview with system metrics |
| `/admin/users` | Users | User management interface |
| `/admin/modules` | Modules | Module configuration |
| `/admin/llm` | LLM Config | Language model settings |
| `/admin/sources` | Sources | External data source configuration |
| `/admin/kb` | Knowledge Base | Cross-collection document management |
| `/admin/health` | Health | Infrastructure health monitoring |
| `/admin/audit` | Audit Log | Activity and usage audit trail |
| `/admin/alerts` | Alerts | Alert configuration and review |
| `/admin/data` | Data Governance | Data retention and privacy settings |
| `/admin/integrations` | Integrations | External API configuration |

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `list_users` | `admin/list-users` | List all users with roles |
| `update_user_role` | `admin/update-role` | Change a user's role |
| `get_system_health` | `admin/health` | Check health of all services |
| `get_llm_usage` | `admin/llm-usage` | Aggregate LLM usage statistics |
| `get_audit_log` | `admin/audit-log` | Paginated audit log entries |
| `update_module_config` | `admin/module-config` | Update module settings |

## Access Control

All admin routes and server functions require the `admin` role. The `AdminShell` layout is nested inside the `AppShell` layout, so authentication is enforced at two levels:

1. `AppShell` checks that the user is authenticated
2. Admin server functions call `require_role("admin")`

## LLM Usage Dashboard

The admin panel provides visibility into LLM consumption across all 26 modules:

- Token usage by module (prompt + completion tokens)
- Latency distribution
- Request volume over time
- Model usage breakdown

Data comes from the `llm_usage_log` table, which is populated by every module that calls the LLM.

## UI Components

- **Admin dashboard** (`/admin`): High-level system overview with quick metrics.
- **User table**: Sortable list of users with role dropdowns for quick role changes.
- **Health dashboard**: Service health cards with status indicators and response times.
- **LLM usage charts**: Token consumption and latency visualizations.
- **Audit log viewer**: Searchable, filterable audit trail with pagination.
- **Module toggle grid**: Enable/disable modules with configuration panels.

## Database Tables

- `users` -- user profiles with roles and status
- `llm_usage_log` -- per-request LLM usage records (module, model, tokens, latency)
- `audit_log` -- administrative action records with timestamps and actor
