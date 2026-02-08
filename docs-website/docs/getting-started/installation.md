# Installation Guide

This guide covers detailed installation instructions for R Commerce on various platforms.

## System Requirements

### Minimum Requirements

- **CPU**: 2 cores
- **RAM**: 2 GB
- **Storage**: 10 GB
- **OS**: Linux, macOS, or FreeBSD

### Recommended Requirements

- **CPU**: 4+ cores
- **RAM**: 4+ GB
- **Storage**: 50+ GB SSD
- **Network**: 100 Mbps+ bandwidth

## Prerequisites

### All Platforms

```bash
# Rust 1.70+ (Install from https://rustup.rs)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# PostgreSQL 13+ (Required)
# See database setup below

# Optional but recommended
# - pkg-config
# - OpenSSL development headers
```

### Platform-Specific Prerequisites

**FreeBSD:**

```bash
# Install system packages
pkg install -y \
  postgresql15-client \
  pkgconf \
  openssl \
  ca_root_nss

# For additional features
pkg install -y \
  redis \
  nginx \
  haproxy
```

**Linux (Debian/Ubuntu):**

```bash
# Install build dependencies
apt-get update
apt-get install -y \
  build-essential \
  pkg-config \
  libssl-dev \
  postgresql-client \
  libpq-dev
apt-get install -y \
  libmysqlclient-dev
```

**Linux (CentOS/RHEL/Fedora):**

```bash
# Install build dependencies
yum groupinstall -y "Development Tools"
yum install -y \
  openssl-devel \
  postgresql-devel \
  pkgconfig
```

**macOS:**

```bash
# Install Xcode command line tools
xcode-select --install

# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install \
  postgresql@15 \
  pkg-config
```

## Build Steps (All Platforms)

```bash
# Clone repository
git clone https://github.com/creativebastard/rcommerce.git
cd gocart

# Build release binary
cargo build --release

# Binary location
target/release/rcommerce

# Run tests
cargo test --release
```

## Initial Setup

After building, use the interactive setup wizard to configure your instance:

```bash
# Run setup wizard
./target/release/rcommerce setup

# The wizard will guide you through:
# - Database configuration and migrations
# - Optional data import from existing stores
# - Server, cache, and security settings
# - TLS/SSL (including Let's Encrypt)
# - Payment gateways and notifications
```

**Setup Wizard Options:**

```bash
# Save config to specific location
./target/release/rcommerce setup -o /etc/rcommerce/config.toml

# If you skip the wizard, configure manually:
# 1. Create config.toml (see Configuration Guide)
# 2. Run migrations: ./target/release/rcommerce db migrate -c config.toml
# 3. Start server: ./target/release/rcommerce server -c config.toml
```

## Docker Installation

### Using Docker Compose (Recommended)

```bash
# Clone the repository
git clone https://github.com/creativebastard/rcommerce.git
cd gocart

# Create environment file
cp .env.example .env
# Edit .env with your configuration

# Start with Docker Compose
docker-compose up -d

# Check logs
docker-compose logs -f rcommerce

# Verify health
curl http://localhost:8080/health
```

### Manual Docker Build

```dockerfile
# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifest and build dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --bin rcommerce || true

# Copy source code
COPY . .

# Build application
RUN cargo build --release --bin rcommerce

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create service user
RUN groupadd -r rcommerce && useradd -r -g rcommerce rcommerce

# Copy binary from builder
COPY --from=builder /app/target/release/rcommerce /usr/local/bin/
COPY --from=builder /app/config/default.toml /etc/rcommerce/config.toml

# Create directories
RUN mkdir -p /var/lib/rcommerce /var/log/rcommerce && \
    chown -R rcommerce:rcommerce /var/lib/rcommerce /var/log/rcommerce

# Switch to service user
USER rcommerce

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

# Start application
CMD ["rcommerce"]
```

Build and run:

```bash
# Build image
docker build -t rcommerce:latest .

# Run container
docker run -d \
  --name rcommerce \
  -p 8080:8080 \
  -v $(pwd)/config/production.toml:/etc/rcommerce/config.toml:ro \
  rcommerce:latest
```

## Binary Installation

### Download Pre-built Binaries

When available, download pre-built binaries from the releases page:

```bash
# Download for Linux x86_64
wget https://github.com/captainjez/gocart/releases/download/v0.1.0/rcommerce-linux-x86_64.tar.gz
tar -xzf rcommerce-linux-x86_64.tar.gz
sudo mv rcommerce /usr/local/bin/

# Download for macOS ARM64
wget https://github.com/captainjez/gocart/releases/download/v0.1.0/rcommerce-macos-arm64.tar.gz
tar -xzf rcommerce-macos-arm64.tar.gz
sudo mv rcommerce /usr/local/bin/
```

### Install via Cargo

```bash
# Install from crates.io (when published)
cargo install rcommerce-cli

# Verify installation
rcommerce --version
```

## Package Manager Installation

### Homebrew (macOS/Linux)

```bash
# Using Homebrew (when available)
brew tap rcommerce/tap
brew install rcommerce
```

### APT (Debian/Ubuntu)

```bash
# Add repository (when available)
sudo apt-add-repository 'deb https://apt.rcommerce.app stable main'
sudo apt update
sudo apt install rcommerce
```

### FreeBSD pkg

```bash
# Using pkg (when available)
sudo pkg install rcommerce

# From ports
cd /usr/ports/commerce/rcommerce && sudo make install
```

## Post-Installation

### 1. Create Configuration Directory

```bash
sudo mkdir -p /etc/rcommerce
sudo mkdir -p /var/lib/rcommerce
sudo mkdir -p /var/log/rcommerce
```

### 2. Create Service User

```bash
# Linux
sudo useradd -r -s /bin/false rcommerce
sudo chown -R rcommerce:rcommerce /var/lib/rcommerce /var/log/rcommerce

# FreeBSD
sudo pw useradd rcommerce -d /var/lib/rcommerce -s /bin/false
sudo chown -R rcommerce:rcommerce /var/lib/rcommerce /var/log/rcommerce
```

### 3. Set Up Configuration

```bash
# Copy example configuration
sudo cp config/production.toml /etc/rcommerce/config.toml

# Edit configuration
sudo nano /etc/rcommerce/config.toml
```

### 4. Test Installation

```bash
# Test configuration
rcommerce config validate

# Test database connection
rcommerce db check

# Run server
rcommerce server start
```

## Troubleshooting

### Build Failures

**Error: `linker 'cc' not found`**

```bash
# Install build tools
# Ubuntu/Debian
sudo apt-get install build-essential

# CentOS/RHEL
sudo yum groupinstall "Development Tools"

# macOS
xcode-select --install
```

**Error: `openssl-sys` build failure**

```bash
# Install OpenSSL development headers
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# CentOS/RHEL
sudo yum install openssl-devel pkgconfig

# macOS
brew install openssl pkg-config
export PKG_CONFIG_PATH="/opt/homebrew/opt/openssl/lib/pkgconfig"
```

**Error: `pq-sys` build failure (PostgreSQL)**

```bash
# Install PostgreSQL development headers
# Ubuntu/Debian
sudo apt-get install libpq-dev

# CentOS/RHEL
sudo yum install postgresql-devel

# macOS
brew install postgresql@15
```

### Runtime Issues

**Permission Denied**

```bash
# Fix permissions
sudo chown -R rcommerce:rcommerce /var/lib/rcommerce /var/log/rcommerce
sudo chmod 750 /var/lib/rcommerce /var/log/rcommerce
```

**Port Already in Use**

```bash
# Find process using port
sudo lsof -i :8080

# Kill process or change port in configuration
```

## Next Steps

- [Configuration Guide](configuration.md) - Configure your installation
- [Development Guide](../development/index.md) - Set up development environment
- [Deployment Guide](../deployment/index.md) - Deploy to production
