# R Commerce Demo Server

A lightweight Rust server for hosting the R Commerce demo frontend without exposing API credentials.

## Features

- 🔒 **Secure**: API keys never exposed to browser
- 🚀 **Fast**: Single Rust binary, no NodeJS
- 📁 **Static Files**: Serves HTML/CSS/JS directly
- 🔄 **API Proxy**: Forwards requests to R Commerce backend
- ⚙️ **Configurable**: Via CLI, config file, or environment variables
- 🐳 **Deployable**: Works behind Caddy, Nginx, etc.

## Quick Start

### 1. Create an API Key

```bash
# Create API key for demo
rcommerce api-key create --name "Demo Server" --scopes "products:read,orders:write,carts:write"

# Save the key - you'll need it for configuration
```

### 2. Configure

**Option A: CLI flags**
```bash
rcommerce-demo \
  --api-url http://localhost:8080 \
  --api-key ak_yourprefix.yoursecret \
  --bind 0.0.0.0:3000
```

**Option B: Config file**
```bash
# Create config file
cat > demo-server.toml << 'EOF'
bind = "0.0.0.0:3000"
api_url = "http://localhost:8080"
api_key = "ak_yourprefix.yoursecret"
static_dir = "demo-frontend"
cors = false
timeout_secs = 30
log_level = "info"
EOF

# Run with config
rcommerce-demo --config demo-server.toml
```

**Option C: Environment variables**
```bash
export RC_DEMO_BIND="0.0.0.0:3000"
export RC_DEMO_API_URL="http://localhost:8080"
export RC_DEMO_API_KEY="ak_yourprefix.yoursecret"
export RC_DEMO_STATIC_DIR="demo-frontend"

rcommerce-demo
```

### 3. Access Demo

Open http://localhost:3000 in your browser.

## Architecture

```
┌─────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Browser   │────▶│  Demo Server    │────▶│  R Commerce     │
│             │◄────│  (Rust)         │◄────│  API            │
└─────────────┘     └─────────────────┘     └─────────────────┘
                         │
                    ┌────┴────────────┐
                    │  API Key        │  ← Injected server-side
                    │  (Hidden)       │     Never in browser
                    └─────────────────┘
```

The demo server:
1. Serves static files from `demo-frontend/` directory
2. Proxies `/api/*` requests to R Commerce backend
3. Injects the API key in the Authorization header
4. Returns responses to the browser

## Configuration Options

| Option | CLI | Config | Environment | Default |
|--------|-----|--------|-------------|---------|
| Bind address | `--bind` | `bind` | `RC_DEMO_BIND` | `0.0.0.0:3000` |
| API URL | `--api-url` | `api_url` | `RC_DEMO_API_URL` | (required) |
| API Key | `--api-key` | `api_key` | `RC_DEMO_API_KEY` | (required) |
| Static dir | `--static-dir` | `static_dir` | `RC_DEMO_STATIC_DIR` | `demo-frontend` |
| Enable CORS | `--cors` | `cors` | - | `false` |
| Timeout | - | `timeout_secs` | - | `30` |
| Log level | - | `log_level` | `RUST_LOG` | `info` |

## Deployment

### With Caddy (Recommended)

```caddyfile
# Caddyfile
demo.yourdomain.com {
    reverse_proxy localhost:3000
    
    # Optional: Enable HTTPS automatically
    tls your@email.com
}
```

Run Caddy and the demo server:
```bash
# Terminal 1: Start demo server
rcommerce-demo --bind 127.0.0.1:3000 --api-key $API_KEY

# Terminal 2: Start Caddy
caddy run
```

### With Nginx

```nginx
server {
    listen 80;
    server_name demo.yourdomain.com;
    
    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### Systemd Service

```ini
# /etc/systemd/system/rcommerce-demo.service
[Unit]
Description=R Commerce Demo Server
After=network.target

[Service]
Type=simple
User=rcommerce
WorkingDirectory=/opt/rcommerce
ExecStart=/usr/local/bin/rcommerce-demo --config /etc/rcommerce/demo-server.toml
Restart=on-failure
Environment="RUST_LOG=info"

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl enable rcommerce-demo
sudo systemctl start rcommerce-demo
```

### Docker

```dockerfile
FROM debian:bookworm-slim

# Install demo server binary
COPY rcommerce-demo /usr/local/bin/

# Copy static files
COPY demo-frontend /var/lib/rcommerce/demo-frontend

# Config
ENV RC_DEMO_BIND="0.0.0.0:3000"
ENV RC_DEMO_STATIC_DIR="/var/lib/rcommerce/demo-frontend"

EXPOSE 3000

CMD ["rcommerce-demo"]
```

Build and run:
```bash
docker build -t rcommerce-demo .
docker run -d \
  -p 3000:3000 \
  -e RC_DEMO_API_URL="http://host.docker.internal:8080" \
  -e RC_DEMO_API_KEY="your-api-key" \
  rcommerce-demo
```

## Building from Source

```bash
# Build release binary
cargo build --release -p rcommerce-demo-server

# Binary location: target/release/rcommerce-demo
```

## Security Notes

1. **API Key Protection**: The API key is never sent to the browser. It only exists in the server config/environment.

2. **Customer JWTs**: Customer authentication tokens (JWTs) are still stored in browser localStorage - this is expected behavior for a frontend.

3. **CORS**: Only enable CORS (`--cors`) in development. For production, serve from the same domain.

4. **HTTPS**: Always use HTTPS in production. Use Caddy or Nginx as a reverse proxy with TLS.

## Differences from Direct API Access

| Feature | Direct API | Via Demo Server |
|---------|-----------|-----------------|
| API Key in browser | ❌ Required | ✅ Hidden |
| CORS issues | ❌ Possible | ✅ Solved |
| Static file serving | ❌ Separate server | ✅ Built-in |
| Setup complexity | ❌ Higher | ✅ Lower |

## Troubleshooting

### "API key is required" error
Make sure you've set the API key via CLI, config file, or environment variable:
```bash
export RC_DEMO_API_KEY="ak_yourprefix.yoursecret"
```

### "Static directory does not exist" error
Check that the `static_dir` path exists and contains the demo frontend files:
```bash
ls demo-frontend/
```

### API requests failing
Check that R Commerce API is running and accessible:
```bash
curl http://localhost:8080/health
```

### CORS errors in browser
Enable CORS for development:
```bash
rcommerce-demo --cors
```

For production, serve from the same domain instead of enabling CORS.

## License

AGPL-3.0
