# Scaling

Guidelines for scaling PolitikTok for larger deployments.

## Horizontal Scaling

### Application Layer

PolitikTok is stateless at the application layer (sessions are stored in PostgreSQL), so you can run multiple instances behind a load balancer:

```
                    ┌──────────────┐
                    │ Load Balancer│
                    └──────┬───────┘
               ┌───────────┼───────────┐
               │           │           │
         ┌─────┴──┐  ┌─────┴──┐  ┌─────┴──┐
         │ App 1  │  │ App 2  │  │ App 3  │
         └────────┘  └────────┘  └────────┘
               │           │           │
         ┌─────┴───────────┴───────────┴──┐
         │          PostgreSQL             │
         └────────────────────────────────┘
```

### Database Layer

- Use PostgreSQL connection pooling (PgBouncer) for high connection counts
- Consider read replicas for analytics-heavy workloads
- Qdrant supports distributed mode for large vector datasets

## Vertical Scaling

### LLM Performance

LLM inference is typically the bottleneck. Options:

| Approach | Benefit |
|----------|---------|
| GPU acceleration | 10-50x faster inference |
| Smaller models | Faster responses, less memory |
| Multiple Ollama instances | Parallel request handling |
| Remote LLM API | Offload compute entirely |

### Memory Optimization

| Component | Memory Usage | Notes |
|-----------|-------------|-------|
| PolitikTok app | ~100-300 MB | Scales with connections |
| PostgreSQL | ~256 MB - 2 GB | Depends on data size |
| Qdrant | ~500 MB - 4 GB | Depends on vector count |
| Ollama (7B model) | ~4-8 GB | Depends on model size |
| Ollama (13B model) | ~8-16 GB | GPU recommended |

## Monitoring

For production deployments, set up monitoring:

- **Application metrics:** Integrate with Prometheus via `axum` middleware
- **Database monitoring:** PostgreSQL `pg_stat_*` views
- **Infrastructure:** Node Exporter + Grafana dashboards
- **Logs:** Centralize with Loki, ELK, or similar

## Caching

Currently not implemented. Planned caching layers:

- LLM response caching for repeated queries
- Database query result caching
- Static asset CDN
