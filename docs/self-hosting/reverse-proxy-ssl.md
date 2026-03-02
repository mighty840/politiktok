# Reverse Proxy & SSL

Set up a reverse proxy with automatic SSL certificate management.

## Option 1: Caddy (Recommended)

Caddy automatically obtains and renews SSL certificates from Let's Encrypt.

### Install Caddy

```bash
apt install -y caddy
```

### Configure Caddy

Edit `/etc/caddy/Caddyfile`:

```
politiktok.yourdomain.com {
    reverse_proxy localhost:9000
}

auth.yourdomain.com {
    reverse_proxy localhost:8080
}
```

### Start Caddy

```bash
systemctl enable caddy
systemctl restart caddy
```

Caddy will automatically obtain SSL certificates and redirect HTTP to HTTPS.

## Option 2: Nginx + Certbot

### Install Nginx and Certbot

```bash
apt install -y nginx certbot python3-certbot-nginx
```

### Configure Nginx

Create `/etc/nginx/sites-available/politiktok`:

```nginx
server {
    listen 80;
    server_name politiktok.yourdomain.com;

    location / {
        proxy_pass http://127.0.0.1:9000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # WebSocket support (for streaming responses)
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}

server {
    listen 80;
    server_name auth.yourdomain.com;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

Enable the site:

```bash
ln -s /etc/nginx/sites-available/politiktok /etc/nginx/sites-enabled/
nginx -t
systemctl restart nginx
```

### Obtain SSL Certificates

```bash
certbot --nginx -d politiktok.yourdomain.com -d auth.yourdomain.com
```

Certbot will automatically configure Nginx for HTTPS and set up auto-renewal.

### Auto-Renewal

Certbot sets up a systemd timer automatically. Verify:

```bash
systemctl status certbot.timer
```

## Option 3: Traefik (Docker-native)

Add Traefik to your Docker Compose stack:

```yaml
services:
  traefik:
    image: traefik:v3.0
    command:
      - "--providers.docker=true"
      - "--entrypoints.web.address=:80"
      - "--entrypoints.websecure.address=:443"
      - "--certificatesresolvers.letsencrypt.acme.tlschallenge=true"
      - "--certificatesresolvers.letsencrypt.acme.email=admin@yourdomain.com"
      - "--certificatesresolvers.letsencrypt.acme.storage=/acme/acme.json"
      - "--entrypoints.web.http.redirections.entrypoint.to=websecure"
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - acme_data:/acme
    restart: unless-stopped

  app:
    # ... your existing app config ...
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.app.rule=Host(`politiktok.yourdomain.com`)"
      - "traefik.http.routers.app.tls.certresolver=letsencrypt"
      - "traefik.http.services.app.loadbalancer.server.port=9000"
```

## Verifying SSL

```bash
curl -I https://politiktok.yourdomain.com
```

You should see `HTTP/2 200` with valid certificate headers.

## Next Steps

- [Backups](/self-hosting/backups) — Set up data backups
- [Maintenance](/self-hosting/maintenance) — Ongoing server management
