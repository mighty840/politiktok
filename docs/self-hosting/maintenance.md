# Maintenance

Ongoing maintenance tasks for your PolitikTok deployment.

## Updating PolitikTok

### Docker Deployment

```bash
# Pull latest image
docker compose -f docker-compose.prod.yml pull app

# Restart with new image
docker compose -f docker-compose.prod.yml up -d app

# Run any pending migrations
docker compose -f docker-compose.prod.yml exec app sqlx migrate run
```

### From Source

```bash
git pull origin main
cargo build --release --features server
# Restart the service
```

## Updating LLM Models

```bash
docker compose exec ollama ollama pull llama3.1:8b
```

## Log Management

### View Logs

```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f app

# Last 100 lines
docker compose logs --tail=100 app
```

### Log Rotation

Docker automatically rotates logs. Configure limits in your compose file:

```yaml
services:
  app:
    logging:
      driver: json-file
      options:
        max-size: "10m"
        max-file: "3"
```

## Health Checks

### Service Status

```bash
docker compose ps
```

### Database Connection

```bash
docker compose exec postgres pg_isready -U politiktok
```

### Disk Usage

```bash
# Docker volumes
docker system df -v

# Overall disk
df -h
```

## Troubleshooting

### Services Won't Start

```bash
# Check logs for errors
docker compose logs app --tail=50

# Restart all services
docker compose down && docker compose up -d
```

### Database Migrations Failed

```bash
# Check migration status
docker compose exec app sqlx migrate info

# Force run pending migrations
docker compose exec app sqlx migrate run
```

### Out of Disk Space

```bash
# Clean unused Docker resources
docker system prune -a --volumes

# Remove old backups
find /opt/politiktok/backups -mtime +30 -delete
```

### High Memory Usage

If Ollama is consuming too much memory, use a smaller model:

```bash
docker compose exec ollama ollama pull llama3.2:3b
```

Update `LLM__MODEL=llama3.2:3b` in your environment.
