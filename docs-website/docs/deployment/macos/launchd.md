# macOS Deployment with launchd

Deploy R Commerce on macOS using launchd for service management.

## Use Cases

- Development environment on macOS
- Small-scale production on Mac servers
- Local testing before cloud deployment
- macOS-based CI/CD runners

## Prerequisites

```bash
# Install Homebrew if not present
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install postgresql@15 redis

# Start services
brew services start postgresql@15
brew services start redis
```

## Installation

### 1. Create Directories

```bash
sudo mkdir -p /usr/local/rcommerce
sudo mkdir -p /usr/local/etc/rcommerce
sudo mkdir -p /usr/local/var/log/rcommerce
sudo mkdir -p /usr/local/var/rcommerce/uploads
```

### 2. Download Binary

```bash
# For Apple Silicon (M1/M2/M3)
curl -L -o /usr/local/rcommerce/rcommerce \
  https://github.com/captainjez/gocart/releases/latest/download/rcommerce-darwin-arm64

# For Intel Macs
curl -L -o /usr/local/rcommerce/rcommerce \
  https://github.com/captainjez/gocart/releases/latest/download/rcommerce-darwin-amd64

chmod +x /usr/local/rcommerce/rcommerce
```

Or build from source:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://gitee.com/captainjez/gocart.git
cd gocart
cargo build --release
sudo cp target/release/rcommerce /usr/local/rcommerce/
```

### 3. Configuration

```bash
sudo tee /usr/local/etc/rcommerce/config.toml << 'EOF'
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
pool_size = 10

[cache]
cache_type = "Redis"
redis_url = "redis://localhost:6379"

[security.jwt]
secret = "${JWT_SECRET}"
expiry_hours = 24

[media]
storage_type = "Local"
local_path = "/usr/local/var/rcommerce/uploads"

[logging]
level = "info"
format = "json"
EOF
```

### 4. Environment File

```bash
sudo tee /usr/local/etc/rcommerce/environment << 'EOF'
DB_PASSWORD=your_secure_password
JWT_SECRET=your_jwt_secret_min_32_chars
RCOMMERCE_CONFIG=/usr/local/etc/rcommerce/config.toml
RUST_LOG=info
EOF

sudo chmod 600 /usr/local/etc/rcommerce/environment
```

### 5. Create launchd Plist

Create `/Library/LaunchDaemons/com.rcommerce.rcommerce.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.rcommerce.rcommerce</string>
    
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/rcommerce/rcommerce</string>
        <string>server</string>
    </array>
    
    <key>EnvironmentVariables</key>
    <dict>
        <key>RCOMMERCE_CONFIG</key>
        <string>/usr/local/etc/rcommerce/config.toml</string>
        <key>RUST_LOG</key>
        <string>info</string>
    </dict>
    
    <key>WorkingDirectory</key>
    <string>/usr/local/rcommerce</string>
    
    <key>StandardOutPath</key>
    <string>/usr/local/var/log/rcommerce/output.log</string>
    
    <key>StandardErrorPath</key>
    <string>/usr/local/var/log/rcommerce/error.log</string>
    
    <key>KeepAlive</key>
    <true/>
    
    <key>RunAtLoad</key>
    <true/>
    
    <key>ThrottleInterval</key>
    <integer>5</integer>
    
    <key>ProcessType</key>
    <string>Background</string>
    
    <key>UserName</key>
    <string>_rcommerce</string>
    
    <key>GroupName</key>
    <string>_rcommerce</string>
    
    <key>SoftResourceLimits</key>
    <dict>
        <key>NumberOfFiles</key>
        <integer>65536</integer>
    </dict>
</dict>
</plist>
```

### 6. Create User

```bash
# Create system user
sudo dscl . -create /Users/_rcommerce
sudo dscl . -create /Users/_rcommerce UserShell /usr/bin/false
sudo dscl . -create /Users/_rcommerce NFSHomeDirectory /usr/local/rcommerce
sudo dscl . -create /Users/_rcommerce PrimaryGroupID 80

# Set ownership
sudo chown -R _rcommerce:_rcommerce /usr/local/rcommerce
sudo chown -R _rcommerce:_rcommerce /usr/local/var/rcommerce
```

### 7. Database Setup

```bash
# Create database
createuser -s rcommerce
createdb -O rcommerce rcommerce

# Set password
psql -c "ALTER USER rcommerce WITH PASSWORD 'your_secure_password';"

# Run migrations
sudo -u _rcommerce /usr/local/rcommerce/rcommerce migrate
```

### 8. Start Service

```bash
# Load and start
sudo launchctl load /Library/LaunchDaemons/com.rcommerce.rcommerce.plist
sudo launchctl start com.rcommerce.rcommerce

# Check status
sudo launchctl list | grep rcommerce

# View logs
tail -f /usr/local/var/log/rcommerce/output.log
tail -f /usr/local/var/log/rcommerce/error.log
```

## Reverse Proxy (nginx)

Install nginx via Homebrew:

```bash
brew install nginx

# Create config
cat > /opt/homebrew/etc/nginx/servers/rcommerce.conf << 'EOF'
upstream rcommerce {
    server 127.0.0.1:8080;
    keepalive 32;
}

server {
    listen 80;
    server_name localhost;
    
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
    }
}
EOF

brew services start nginx
```

## Service Management

```bash
# Start
sudo launchctl start com.rcommerce.rcommerce

# Stop
sudo launchctl stop com.rcommerce.rcommerce

# Restart
sudo launchctl stop com.rcommerce.rcommerce
sudo launchctl start com.rcommerce.rcommerce

# Unload
sudo launchctl unload /Library/LaunchDaemons/com.rcommerce.rcommerce.plist

# Reload after changes
sudo launchctl unload /Library/LaunchDaemons/com.rcommerce.rcommerce.plist
sudo launchctl load /Library/LaunchDaemons/com.rcommerce.rcommerce.plist
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Service won't start | Check Console.app for errors |
| Permission denied | Verify file ownership with `_rcommerce` user |
| Port already in use | Change port or stop conflicting service |
| Database connection failed | Check PostgreSQL is running: `brew services list` |

## Development Setup

For local development, use directly without launchd:

```bash
# In project directory
cargo run -- server

# Or with hot reload
cargo watch -x "run -- server"
```
