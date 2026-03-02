# Server Setup

This guide walks through preparing a fresh Linux server for a PolitikTok deployment.

## Create a Server

Any VPS or dedicated server running Ubuntu 22.04+ works. Recommended providers:

- Hetzner
- DigitalOcean
- Linode
- AWS EC2
- Self-hosted bare metal

## Initial Server Configuration

### Connect via SSH

```bash
ssh root@your-server-ip
```

### Create a Non-Root User

```bash
adduser politiktok
usermod -aG sudo politiktok
```

### Update the System

```bash
apt update && apt upgrade -y
```

### Install Docker

```bash
curl -fsSL https://get.docker.com | sh
usermod -aG docker politiktok
```

Log out and back in for the group change to take effect.

### Install Docker Compose

```bash
apt install docker-compose-plugin -y
```

Verify:

```bash
docker compose version
```

## Firewall Configuration

```bash
ufw allow OpenSSH
ufw allow 80/tcp
ufw allow 443/tcp
ufw enable
```

::: warning
Do **not** expose database ports (5432, 6333, 6334) or internal service ports (8080, 11434) to the public internet.
:::

## Automatic Security Updates

```bash
apt install unattended-upgrades -y
dpkg-reconfigure -plow unattended-upgrades
```

## SSH Hardening (Optional)

Edit `/etc/ssh/sshd_config`:

```
PermitRootLogin no
PasswordAuthentication no
PubkeyAuthentication yes
```

Then restart SSH:

```bash
systemctl restart sshd
```

## Next Steps

Once your server is ready, proceed to [Docker Compose Production](/self-hosting/docker-compose).
