# Docker Deployment

Deploy PolitikTok using Docker for production use.

## Building the Docker Image

```bash
docker build -t politiktok:latest .
```

## Running with Docker

```bash
docker run -d \
  --name politiktok \
  -p 9000:9000 \
  -e DATABASE__URL=postgresql://user:pass@host:5432/politiktok \
  -e LLM__BASE_URL=http://host:11434/v1 \
  -e LLM__MODEL=llama3.1:8b \
  politiktok:latest
```

## Docker Compose

For a complete production setup with all services, see [Docker Compose Production](/self-hosting/docker-compose).

## Container Registry

Pre-built images are available from GitHub Container Registry:

```bash
docker pull ghcr.io/mighty840/politiktok:latest
```

## Health Check

The container exposes a health check endpoint:

```bash
curl http://localhost:9000/api/check-auth
```

## Resource Limits

Recommended Docker resource limits:

```yaml
services:
  app:
    deploy:
      resources:
        limits:
          cpus: "2.0"
          memory: 1G
        reservations:
          cpus: "0.5"
          memory: 256M
```
