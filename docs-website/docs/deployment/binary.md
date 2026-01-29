# Binary Deployment

Deploy R Commerce as a standalone binary without containers.

## Download

```bash
# Linux AMD64
curl -L -o rcommerce https://github.com/captainjez/gocart/releases/latest/download/rcommerce-linux-amd64

# Linux ARM64
curl -L -o rcommerce https://github.com/captainjez/gocart/releases/latest/download/rcommerce-linux-arm64

# macOS AMD64
curl -L -o rcommerce https://github.com/captainjez/gocart/releases/latest/download/rcommerce-darwin-amd64

# macOS ARM64
curl -L -o rcommerce https://github.com/captainjez/gocart/releases/latest/download/rcommerce-darwin-arm64

chmod +x rcommerce
```

## Run

```bash
# Set config path
export RCOMMERCE_CONFIG=/etc/rcommerce/config.toml

# Start server
./rcommerce server

# Run migrations
./rcommerce migrate
```

See platform-specific guides for service management:
- [Linux systemd](./linux/systemd.md)
- [FreeBSD](./freebsd/jails.md)
- [macOS](./macos/launchd.md)
