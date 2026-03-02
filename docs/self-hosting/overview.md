# Self-Hosting Overview

PolitikTok is designed to be fully self-hosted. Your data never leaves your infrastructure — all AI processing happens locally through Ollama or any OpenAI-compatible API.

## Deployment Options

| Method | Best For | Complexity |
|--------|----------|------------|
| Docker Compose | Small to medium deployments | Low |
| Manual | Full control, custom setups | Medium |
| Kubernetes | Large-scale, multi-node | High |

## Architecture Overview

A production PolitikTok deployment consists of:

```
┌─────────────────────────────────────────────┐
│                 Reverse Proxy               │
│            (Caddy / Nginx / Traefik)        │
│              SSL Termination                │
├─────────────────────────────────────────────┤
│              PolitikTok App                 │
│         (Dioxus SSR + Axum API)             │
├──────────┬──────────┬──────────┬────────────┤
│ PostgreSQL│  Qdrant  │ Keycloak │   Ollama   │
│    DB     │  Vectors │   Auth   │    LLM     │
└──────────┴──────────┴──────────┴────────────┘
```

## Minimum Server Requirements

- **CPU:** 4 cores (8+ recommended for LLM inference)
- **RAM:** 8 GB (16+ GB recommended)
- **Disk:** 40 GB SSD (100+ GB if hosting LLM models)
- **OS:** Ubuntu 22.04 LTS or newer

## Security Considerations

- All services should run behind a reverse proxy with SSL
- Keycloak handles authentication — never expose it directly
- Database ports should not be publicly accessible
- Use strong secrets for JWT, database passwords, and API keys
- Enable firewall rules to restrict access

## Next Steps

1. [Server Setup](/self-hosting/server-setup) — Prepare your server
2. [Docker Compose Production](/self-hosting/docker-compose) — Deploy with Docker
3. [Domain & DNS](/self-hosting/domain-dns) — Configure your domain
4. [Reverse Proxy & SSL](/self-hosting/reverse-proxy-ssl) — Set up HTTPS
5. [Backups](/self-hosting/backups) — Protect your data
6. [Maintenance](/self-hosting/maintenance) — Keep things running
