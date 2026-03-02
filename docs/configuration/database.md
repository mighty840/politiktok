# Database

PolitikTok uses PostgreSQL for relational data and Qdrant for vector storage.

## PostgreSQL

### Connection String

```
DATABASE__URL=postgresql://user:password@host:5432/politiktok
```

### Connection Pool

PolitikTok uses `sqlx` with an async connection pool. Pool settings are configured via environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE__URL` | — | PostgreSQL connection string (required) |
| `DATABASE__MAX_CONNECTIONS` | `10` | Maximum pool connections |
| `DATABASE__MIN_CONNECTIONS` | `1` | Minimum idle connections |

### Migrations

Migrations are managed by `sqlx` and stored in the `migrations/` directory.

```bash
# Run pending migrations
sqlx migrate run

# Check migration status
sqlx migrate info

# Create a new migration
sqlx migrate add <name>
```

### Production Recommendations

- Use a dedicated PostgreSQL instance or managed service
- Enable SSL connections (`?sslmode=require` in connection string)
- Configure regular backups (see [Backups](/self-hosting/backups))
- Monitor connection pool usage

## Qdrant (Vector Store)

### Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `VECTOR_STORE__URL` | `http://localhost:6334` | Qdrant gRPC endpoint |
| `VECTOR_STORE__API_KEY` | — | Optional API key |

### Collections

PolitikTok creates the following Qdrant collections:

| Collection | Vector Size | Purpose |
|------------|------------|---------|
| `policy_documents` | 1536 | Policy chatbot RAG |
| `knowledge_base` | 1536 | Knowledge base RAG |

Collections are auto-created on first use with cosine distance.
