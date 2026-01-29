# FreeBSD Standalone Deployment

Deploy R Commerce directly on FreeBSD without using jails. This is the simplest deployment method for single-server setups.

## Supported FreeBSD Versions

- **FreeBSD 14.2** - Latest production release (recommended)
- **FreeBSD 15.0** - Current stable branch
- **FreeBSD 13.4** - Legacy support (until 2026)

## When to Use Standalone Deployment

- **Single application server** - No need for isolation
- **Development environment** - Quick setup for testing
- **Resource constraints** - Avoid jail overhead
- **Simple infrastructure** - Easier management

## Prerequisites

```bash
# FreeBSD 14.0 or later
freebsd-version

# Root or sudo access
# Internet connection for package installation
```

## Installation

### 1. System Preparation

```bash
# Update system packages
pkg update
pkg upgrade -y

# Install required packages
pkg install -y \
  postgresql15-server \
  postgresql15-client \
  redis \
  nginx \
  ca_root_nss \
  curl \
  sudo

# Enable services at boot
tee -a /etc/rc.conf << 'EOF'
postgresql_enable="YES"
redis_enable="YES"
nginx_enable="YES"
EOF
```

### 2. Database Setup

```bash
# Initialize PostgreSQL
service postgresql initdb

# Start PostgreSQL
service postgresql start

# Create database and user
su - postgres << 'EOF'
createdb rcommerce
createuser -P rcommerce
# Enter password when prompted
psql -c "GRANT ALL PRIVILEGES ON DATABASE rcommerce TO rcommerce;"
EOF
```

### 3. Redis Setup

```bash
# Start Redis
service redis start

# Verify Redis is running
redis-cli ping
# Should return: PONG
```

### 4. Create R Commerce User

```bash
# Create dedicated user
pw useradd -n rcommerce -s /bin/sh -d /usr/local/rcommerce -m

# Add to sudoers for maintenance (optional)
echo "rcommerce ALL=(ALL) NOPASSWD: /usr/sbin/service rcommerce *" >> /usr/local/etc/sudoers.d/rcommerce
```

### 5. Install R Commerce

```bash
# Download latest release
curl -L -o /usr/local/bin/rcommerce \
  "https://github.com/creativebastard/rcommerce/releases/latest/download/rcommerce-freebsd-amd64"

# Make executable
chmod +x /usr/local/bin/rcommerce

# Verify installation
rcommerce --version
```

### 6. Configuration

```bash
# Create configuration directory
mkdir -p /usr/local/etc/rcommerce

# Create configuration file
cat > /usr/local/etc/rcommerce/config.toml << 'EOF'
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
password = "your_secure_password"
pool_size = 20

[cache]
cache_type = "Redis"
redis_url = "redis://127.0.0.1:6379"

[logging]
level = "info"
format = "Json"

[media]
storage_type = "Local"
local_path = "/usr/local/rcommerce/uploads"
EOF

# Set permissions
chown -R rcommerce:rcommerce /usr/local/etc/rcommerce
chmod 600 /usr/local/etc/rcommerce/config.toml

# Create uploads directory
mkdir -p /usr/local/rcommerce/uploads
chown -R rcommerce:rcommerce /usr/local/rcommerce
```

### 7. rc.d Service Script

Create `/usr/local/etc/rc.d/rcommerce`:

```sh
#!/bin/sh
# PROVIDE: rcommerce
# REQUIRE: postgresql redis
# KEYWORD: shutdown

. /etc/rc.subr

name="rcommerce"
rcvar="rcommerce_enable"

load_rc_config $name

: ${rcommerce_enable:="NO"}
: ${rcommerce_config:="/usr/local/etc/rcommerce/config.toml"}
: ${rcommerce_user:="rcommerce"}
: ${rcommerce_group:="rcommerce"}
: ${rcommerce_dir:="/usr/local/rcommerce"}

command="/usr/local/bin/rcommerce"
procname="/usr/local/bin/rcommerce"
pidfile="/var/run/${name}.pid"

start_cmd="rcommerce_start"
stop_cmd="rcommerce_stop"
status_cmd="rcommerce_status"
reload_cmd="rcommerce_reload"

rcommerce_start() {
    echo "Starting ${name}."
    export RCOMMERCE_CONFIG=${rcommerce_config}
    cd ${rcommerce_dir}
    /usr/sbin/daemon -f -p ${pidfile} -u ${rcommerce_user} \
        ${command} server
}

rcommerce_stop() {
    echo "Stopping ${name}."
    if [ -f ${pidfile} ]; then
        kill $(cat ${pidfile})
        rm -f ${pidfile}
    fi
}

rcommerce_status() {
    if [ -f ${pidfile} ] && kill -0 $(cat ${pidfile}) 2>/dev/null; then
        echo "${name} is running as pid $(cat ${pidfile})."
    else
        echo "${name} is not running."
        return 1
    fi
}

rcommerce_reload() {
    echo "Reloading ${name} configuration..."
    if [ -f ${pidfile} ]; then
        kill -HUP $(cat ${pidfile})
    else
        echo "${name} is not running."
        return 1
    fi
}

run_rc_command "$1"
```

Enable the service:

```bash
chmod +x /usr/local/etc/rc.d/rcommerce
sysrc rcommerce_enable=YES
```

### 8. Nginx Reverse Proxy

```bash
cat > /usr/local/etc/nginx/nginx.conf << 'EOF'
user www;
worker_processes auto;
error_log /var/log/nginx/error.log;
pid /var/run/nginx.pid;

events {
    worker_connections 1024;
    use kqueue;
}

http {
    include mime.types;
    default_type application/octet-stream;

    log_format main '$remote_addr - $remote_user [$time_local] "$request" '
                    '$status $body_bytes_sent "$http_referer" '
                    '"$http_user_agent" "$http_x_forwarded_for"';

    access_log /var/log/nginx/access.log main;

    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_proxied any;
    gzip_comp_level 6;
    gzip_types text/plain text/css text/xml application/json application/javascript application/rss+xml application/atom+xml image/svg+xml;

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
    limit_req_zone $binary_remote_addr zone=login:10m rate=1r/s;

    upstream rcommerce {
        server 127.0.0.1:8080;
        keepalive 32;
    }

    server {
        listen 80;
        server_name _;
        
        # Security headers
        add_header X-Frame-Options "SAMEORIGIN" always;
        add_header X-Content-Type-Options "nosniff" always;
        add_header X-XSS-Protection "1; mode=block" always;
        add_header Referrer-Policy "strict-origin-when-cross-origin" always;

        # Health check endpoint (no rate limit)
        location /health {
            proxy_pass http://rcommerce;
            proxy_http_version 1.1;
            proxy_set_header Connection "";
        }

        # API endpoints with rate limiting
        location /api/ {
            limit_req zone=api burst=20 nodelay;
            
            proxy_pass http://rcommerce;
            proxy_http_version 1.1;
            proxy_set_header Connection "";
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            
            proxy_connect_timeout 30s;
            proxy_send_timeout 30s;
            proxy_read_timeout 30s;
        }

        # Static files
        location /uploads/ {
            alias /usr/local/rcommerce/uploads/;
            expires 1y;
            add_header Cache-Control "public, immutable";
        }

        # Default location
        location / {
            proxy_pass http://rcommerce;
            proxy_http_version 1.1;
            proxy_set_header Connection "";
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }
}
EOF

# Test nginx configuration
nginx -t

# Start nginx
service nginx start
```

### 9. Start R Commerce

```bash
# Start the service
service rcommerce start

# Check status
service rcommerce status

# View logs
tail -f /var/log/rcommerce.log
```

## SSL/TLS with Let's Encrypt

```bash
# Install certbot
pkg install py39-certbot

# Obtain certificate
certbot certonly --standalone -d yourdomain.com

# Update nginx configuration for SSL
# (See reverse proxy documentation for full SSL config)
```

## Monitoring

```bash
# Install monitoring tools
pkg install monit

# Create monit configuration
cat > /usr/local/etc/monit.d/rcommerce << 'EOF'
check process rcommerce matching "/usr/local/bin/rcommerce"
    start program = "/usr/sbin/service rcommerce start"
    stop program = "/usr/sbin/service rcommerce stop"
    if failed host 127.0.0.1 port 8080 protocol http
        request /health
        with timeout 10 seconds
        then restart
    if 5 restarts within 5 cycles then timeout
EOF

sysrc monit_enable=YES
service monit start
```

## Backup Strategy

```bash
# Database backup
pg_dump -U rcommerce rcommerce > /backup/rcommerce-$(date +%Y%m%d).sql

# File backup
tar -czf /backup/rcommerce-files-$(date +%Y%m%d).tar.gz /usr/local/rcommerce/uploads

# Automated backup script
cat > /usr/local/bin/backup-rcommerce.sh << 'EOF'
#!/bin/sh
BACKUP_DIR="/backup"
DATE=$(date +%Y%m%d)

# Database
pg_dump -U rcommerce rcommerce | gzip > $BACKUP_DIR/rcommerce-db-$DATE.sql.gz

# Files
tar -czf $BACKUP_DIR/rcommerce-files-$DATE.tar.gz /usr/local/rcommerce/uploads

# Keep only last 7 days
find $BACKUP_DIR -name "rcommerce-*" -mtime +7 -delete
EOF

chmod +x /usr/local/bin/backup-rcommerce.sh

# Add to cron
0 2 * * * /usr/local/bin/backup-rcommerce.sh
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Service won't start | Check `service rcommerce status` and logs |
| Database connection failed | Verify PostgreSQL is running and credentials |
| Port already in use | Check with `sockstat -4 -l` |
| Permission denied | Check file ownership with `ls -la` |
| Out of memory | Check with `top` or `vmstat` |

## See Also

- [Jail Deployment](jails.md) - Deploy with iocage jails
- [rc.d Service](rc.d.md) - Service management details
- [Nginx Reverse Proxy](../../operations/reverse-proxies/nginx.md)
- [Operations Overview](../../operations/index.md)
