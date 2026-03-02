# Manual Deployment

Deploy PolitikTok by building from source.

## Prerequisites

- Rust 1.82+
- Dioxus CLI 0.7.3
- PostgreSQL 16+
- Qdrant (for RAG features)
- Keycloak (for authentication)

## Build for Production

```bash
git clone https://github.com/mighty840/politiktok.git
cd politiktok

# Build the release binary
dx build --release
```

The compiled binary will be in `target/release/`.

## Configure Environment

Create a `.env` file or export environment variables:

```bash
export APP__HOST=0.0.0.0
export APP__PORT=9000
export DATABASE__URL=postgresql://user:pass@localhost:5432/politiktok
export LLM__BASE_URL=http://localhost:11434/v1
export LLM__MODEL=llama3.1:8b
export EMBEDDING__BASE_URL=http://localhost:11434/v1
export EMBEDDING__MODEL=nomic-embed-text
export VECTOR_STORE__URL=http://localhost:6334
export KEYCLOAK__URL=http://localhost:8080
export KEYCLOAK__REALM=politiktok
export KEYCLOAK__CLIENT_ID=politiktok-app
```

## Run Migrations

```bash
sqlx migrate run
```

## Create a Systemd Service

Create `/etc/systemd/system/politiktok.service`:

```ini
[Unit]
Description=PolitikTok Campaign Intelligence Platform
After=network.target postgresql.service

[Service]
Type=simple
User=politiktok
Group=politiktok
WorkingDirectory=/opt/politiktok
ExecStart=/opt/politiktok/target/release/politiktok
EnvironmentFile=/opt/politiktok/.env
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
systemctl daemon-reload
systemctl enable politiktok
systemctl start politiktok
```

## Verify

```bash
systemctl status politiktok
curl http://localhost:9000
```

## Next Steps

- Set up a [reverse proxy with SSL](/self-hosting/reverse-proxy-ssl)
- Configure [backups](/self-hosting/backups)
