# Docker Compose Production

Deploy PolitikTok and all its services with Docker Compose.

## Production Docker Compose

Create a `docker-compose.prod.yml` on your server:

```yaml
services:
  app:
    image: ghcr.io/mighty840/politiktok:latest
    # Or build from source:
    # build: .
    ports:
      - "127.0.0.1:9000:9000"
    environment:
      - APP__HOST=0.0.0.0
      - APP__PORT=9000
      - DATABASE__URL=postgresql://politiktok:${DB_PASSWORD}@postgres:5432/politiktok
      - LLM__BASE_URL=http://ollama:11434/v1
      - LLM__MODEL=llama3.1:8b
      - EMBEDDING__BASE_URL=http://ollama:11434/v1
      - EMBEDDING__MODEL=nomic-embed-text
      - VECTOR_STORE__URL=http://qdrant:6334
      - KEYCLOAK__URL=http://keycloak:8080
      - KEYCLOAK__REALM=politiktok
      - KEYCLOAK__CLIENT_ID=politiktok-app
    depends_on:
      postgres:
        condition: service_healthy
      qdrant:
        condition: service_started
      keycloak:
        condition: service_started
    restart: unless-stopped

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: politiktok
      POSTGRES_USER: politiktok
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - pgdata:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U politiktok"]
      interval: 5s
      timeout: 5s
      retries: 5
    restart: unless-stopped

  qdrant:
    image: qdrant/qdrant:v1.12.4
    volumes:
      - qdrant_data:/qdrant/storage
    restart: unless-stopped

  keycloak:
    image: quay.io/keycloak/keycloak:24.0
    command: start --hostname-strict=false --http-enabled=true
    environment:
      KC_DB: postgres
      KC_DB_URL: jdbc:postgresql://postgres:5432/politiktok
      KC_DB_USERNAME: politiktok
      KC_DB_PASSWORD: ${DB_PASSWORD}
      KEYCLOAK_ADMIN: admin
      KEYCLOAK_ADMIN_PASSWORD: ${KEYCLOAK_ADMIN_PASSWORD}
    depends_on:
      postgres:
        condition: service_healthy
    restart: unless-stopped

  ollama:
    image: ollama/ollama:latest
    volumes:
      - ollama_data:/root/.ollama
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: all
              capabilities: [gpu]
    restart: unless-stopped

volumes:
  pgdata:
  qdrant_data:
  ollama_data:
```

## Environment File

Create a `.env` file alongside the compose file:

```bash
DB_PASSWORD=your-strong-database-password
KEYCLOAK_ADMIN_PASSWORD=your-strong-keycloak-password
```

## Start the Stack

```bash
docker compose -f docker-compose.prod.yml up -d
```

## Pull LLM Models

After Ollama is running:

```bash
docker compose -f docker-compose.prod.yml exec ollama ollama pull llama3.1:8b
docker compose -f docker-compose.prod.yml exec ollama ollama pull nomic-embed-text
```

## Verify

```bash
docker compose -f docker-compose.prod.yml ps
```

All services should show `running` status.

## Next Steps

- [Domain & DNS](/self-hosting/domain-dns) — Point your domain to this server
- [Reverse Proxy & SSL](/self-hosting/reverse-proxy-ssl) — Add HTTPS
