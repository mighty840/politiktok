# Backups

Protect your PolitikTok data with regular backups.

## What to Back Up

| Data | Location | Priority |
|------|----------|----------|
| PostgreSQL database | `pgdata` Docker volume | Critical |
| Qdrant vectors | `qdrant_data` Docker volume | High |
| Keycloak realm config | Keycloak admin export | High |
| LLM models | `ollama_data` Docker volume | Low (re-downloadable) |
| Environment files | `.env`, docker-compose files | Critical |

## PostgreSQL Backup

### Manual Dump

```bash
docker compose exec postgres pg_dump -U politiktok politiktok > backup_$(date +%Y%m%d).sql
```

### Automated Daily Backups

Create `/opt/politiktok/backup.sh`:

```bash
#!/bin/bash
BACKUP_DIR="/opt/politiktok/backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

mkdir -p "$BACKUP_DIR"

# PostgreSQL dump
docker compose exec -T postgres pg_dump -U politiktok politiktok \
  | gzip > "$BACKUP_DIR/db_${TIMESTAMP}.sql.gz"

# Keep only last 30 days
find "$BACKUP_DIR" -name "db_*.sql.gz" -mtime +30 -delete

echo "Backup completed: db_${TIMESTAMP}.sql.gz"
```

Add a cron job:

```bash
chmod +x /opt/politiktok/backup.sh
crontab -e
# Add: 0 3 * * * /opt/politiktok/backup.sh
```

### Restore from Backup

```bash
gunzip < backup_20240101.sql.gz | docker compose exec -T postgres psql -U politiktok politiktok
```

## Qdrant Backup

Qdrant supports snapshot-based backups:

```bash
# Create a snapshot of all collections
curl -X POST http://localhost:6333/snapshots

# Download the snapshot
curl http://localhost:6333/snapshots/<snapshot-name> -o qdrant_backup.snapshot
```

## Off-Site Backup

Use `rclone` or `restic` to sync backups to cloud storage:

```bash
# Example with rclone to S3-compatible storage
rclone copy /opt/politiktok/backups remote:politiktok-backups
```

## Next Steps

- [Maintenance](/self-hosting/maintenance) — Ongoing server management
