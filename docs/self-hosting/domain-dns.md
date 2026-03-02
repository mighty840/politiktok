# Domain & DNS

Configure a custom domain for your PolitikTok instance.

## DNS Records

Add the following DNS records at your domain registrar or DNS provider:

| Type | Name | Value | TTL |
|------|------|-------|-----|
| A | `politiktok.yourdomain.com` | `your-server-ip` | 300 |
| A | `auth.yourdomain.com` | `your-server-ip` | 300 |

The second record is for Keycloak if you want it on a subdomain.

## Verify DNS Propagation

```bash
dig politiktok.yourdomain.com +short
```

This should return your server's IP address. DNS propagation can take up to 48 hours but usually completes within minutes.

## Update Application Configuration

Once DNS is configured, update your environment variables:

```bash
# In your .env or docker-compose environment
APP__PUBLIC_URL=https://politiktok.yourdomain.com
KEYCLOAK__URL=https://auth.yourdomain.com
```

## Next Steps

- [Reverse Proxy & SSL](/self-hosting/reverse-proxy-ssl) — Set up HTTPS with your domain
