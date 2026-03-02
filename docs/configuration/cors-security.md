# CORS & Security

Security configuration for your PolitikTok deployment.

## CORS Configuration

Cross-Origin Resource Sharing settings control which domains can access the PolitikTok API.

| Variable | Default | Description |
|----------|---------|-------------|
| `APP__CORS_ORIGINS` | `http://localhost:9000` | Comma-separated allowed origins |

In production, set this to your actual domain:

```
APP__CORS_ORIGINS=https://politiktok.yourdomain.com
```

## Authentication Security

PolitikTok uses Keycloak for OpenID Connect (OIDC) authentication with PKCE flow.

| Variable | Description |
|----------|-------------|
| `KEYCLOAK__URL` | Keycloak server URL |
| `KEYCLOAK__REALM` | Realm name |
| `KEYCLOAK__CLIENT_ID` | OIDC client ID |
| `KEYCLOAK__CLIENT_SECRET` | OIDC client secret (optional with PKCE) |

### Session Management

Sessions are stored server-side. Session cookies use:

- `HttpOnly` flag (prevents JavaScript access)
- `Secure` flag (HTTPS only in production)
- `SameSite=Lax` (CSRF protection)

### Role-Based Access Control

PolitikTok supports role-based access through Keycloak realm roles:

| Role | Access Level |
|------|-------------|
| `admin` | Full access including admin panel |
| `manager` | All modules, no admin panel |
| `analyst` | Read-only access to analytics modules |
| `volunteer` | Limited access to assigned modules |

## Security Headers

When using a reverse proxy, add these security headers:

```nginx
# Nginx example
add_header X-Frame-Options "SAMEORIGIN" always;
add_header X-Content-Type-Options "nosniff" always;
add_header X-XSS-Protection "1; mode=block" always;
add_header Referrer-Policy "strict-origin-when-cross-origin" always;
add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'unsafe-inline' cdn.tailwindcss.com cdn.jsdelivr.net; style-src 'self' 'unsafe-inline' cdn.jsdelivr.net fonts.googleapis.com; font-src fonts.gstatic.com;" always;
```

## Secret Management

::: danger
Never commit secrets to version control. Use environment variables or a secrets manager.
:::

Required secrets:

| Secret | Where Used |
|--------|-----------|
| `DATABASE__URL` | PostgreSQL connection (contains password) |
| `KEYCLOAK__CLIENT_SECRET` | OIDC authentication |
| LLM API keys | If using a remote LLM provider |

For production, consider using:

- Docker secrets
- HashiCorp Vault
- Cloud provider secret managers (AWS SSM, GCP Secret Manager)
