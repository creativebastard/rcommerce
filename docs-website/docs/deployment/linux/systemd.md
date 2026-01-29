# Linux Deployment with systemd

Deploy R Commerce as a systemd service on Linux distributions.

## Supported Distributions

- Ubuntu 20.04 LTS+
- Debian 11+
- CentOS/RHEL 8+
- Fedora 35+
- Arch Linux

## Prerequisites

```bash
# Update system
sudo apt update && sudo apt upgrade -y  # Debian/Ubuntu
sudo dnf update -y                       # RHEL/Fedora

# Install dependencies
sudo apt install -y postgresql redis-server nginx
```

## Installation

### 1. Create User

```bash
sudo useradd -r -s /bin/false rcommerce
sudo mkdir -p /opt/rcommerce
sudo chown rcommerce:rcommerce /opt/rcommerce
```

### 2. Download Binary

```bash
# Download latest release
cd /opt/rcommerce
sudo -u rcommerce curl -L -o rcommerce \
  https://github.com/captainjez/gocart/releases/latest/download/rcommerce-linux-amd64
sudo -u rcommerce chmod +x rcommerce
```

Or build from source:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://gitee.com/captainjez/gocart.git
cd gocart
cargo build --release
sudo cp target/release/rcommerce /opt/rcommerce/
sudo chown rcommerce:rcommerce /opt/rcommerce/rcommerce
```

### 3. Configuration

```bash
sudo mkdir -p /etc/rcommerce
sudo tee /etc/rcommerce/config.toml << 'EOF'
[server]
host = "127.0.0.1"
port = 8080
worker_threads = 4

[database]
db_type = "Postgres"
host = "localhost"
port = 5432
database = "rcommerce"
username = "rcommerce"
password = "${DB_PASSWORD}"
pool_size = 20

[cache]
cache_type = "Redis"
redis_url = "redis://localhost:6379"

[security]
api_key_prefix_length = 8
api_key_secret_length = 32

[security.jwt]
secret = "${JWT_SECRET}"
expiry_hours = 24

[logging]
level = "info"
format = "json"
EOF

sudo chown -R rcommerce:rcommerce /etc/rcommerce
sudo chmod 600 /etc/rcommerce/config.toml
```

### 4. Environment File

```bash
sudo tee /etc/rcommerce/environment << 'EOF'
DB_PASSWORD=your_secure_password
JWT_SECRET=your_jwt_secret_min_32_chars
RCOMMERCE_CONFIG=/etc/rcommerce/config.toml
RUST_LOG=info
EOF

sudo chmod 600 /etc/rcommerce/environment
```

### 5. Create systemd Service

```bash
sudo tee /etc/systemd/system/rcommerce.service << 'EOF'
[Unit]
Description=R Commerce Headless E-Commerce Platform
After=network.target postgresql.service redis.service
Wants=postgresql.service redis.service

[Service]
Type=simple
User=rcommerce
Group=rcommerce
WorkingDirectory=/opt/rcommerce
EnvironmentFile=/etc/rcommerce/environment
ExecStart=/opt/rcommerce/rcommerce server
ExecReload=/bin/kill -HUP $MAINPID
Restart=on-failure
RestartSec=5
StandardOutput=journal
StandardError=journal
SyslogIdentifier=rcommerce

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/rcommerce/uploads
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true

[Install]
WantedBy=multi-user.target
EOF
```

### 6. Database Setup

```bash
# Create database and user
sudo -u postgres psql << 'EOF'
CREATE USER rcommerce WITH PASSWORD 'your_secure_password';
CREATE DATABASE rcommerce OWNER rcommerce;
GRANT ALL PRIVILEGES ON DATABASE rcommerce TO rcommerce;
\c rcommerce
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
EOF

# Run migrations
sudo -u rcommerce /opt/rcommerce/rcommerce migrate
```

### 7. Start Service

```bash
sudo systemctl daemon-reload
sudo systemctl enable rcommerce
sudo systemctl start rcommerce

# Check status
sudo systemctl status rcommerce
sudo journalctl -u rcommerce -f
```

## Reverse Proxy (nginx)

```bash
sudo tee /etc/nginx/sites-available/rcommerce << 'EOF'
upstream rcommerce {
    server 127.0.0.1:8080;
    keepalive 32;
}

server {
    listen 80;
    server_name api.yourstore.com;
    
    location / {
        return 301 https://$server_name$request_uri;
    }
}

server {
    listen 443 ssl http2;
    server_name api.yourstore.com;
    
    ssl_certificate /etc/letsencrypt/live/api.yourstore.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.yourstore.com/privkey.pem;
    
    client_max_body_size 50M;
    
    location / {
        proxy_pass http://rcommerce;
        proxy_http_version 1.1;
        proxy_set_header Connection "";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_buffering off;
        proxy_read_timeout 300s;
    }
}
EOF

sudo ln -s /etc/nginx/sites-available/rcommerce /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

## Management Commands

```bash
# View logs
sudo journalctl -u rcommerce -f

# Restart service
sudo systemctl restart rcommerce

# Check configuration
sudo -u rcommerce /opt/rcommerce/rcommerce config

# Database status
sudo -u rcommerce /opt/rcommerce/rcommerce db status
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Service fails to start | Check `journalctl -u rcommerce` for errors |
| Database connection failed | Verify credentials and PostgreSQL status |
| Permission denied | Check file ownership and permissions |
| Port already in use | Change port in config or stop conflicting service |

## Security Hardening

1. **Firewall**: Allow only necessary ports
2. **Fail2ban**: Protect against brute force
3. **Updates**: Keep system packages updated
4. **Backups**: Regular database backups
5. **SSL**: Use Let's Encrypt for certificates
