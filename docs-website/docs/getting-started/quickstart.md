# Quick Start Guide

Get R Commerce up and running in minutes with this quick start guide.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust 1.70+** - [Install from rustup.rs](https://rustup.rs/)
- **PostgreSQL 13+** or **MySQL 8+** or **SQLite 3+**
- **Redis 6+** (optional, for caching)

## Installation

### Option 1: Build from Source

```bash
# Clone the repository
git clone https://gitee.com/captainjez/gocart.git
cd gocart

# Build the project
cargo build --release

# The binary will be at:
# target/release/rcommerce
```

### Option 2: Docker (Recommended for Quick Start)

```bash
# Clone the repository
git clone https://gitee.com/captainjez/gocart.git
cd gocart

# Start all services
docker-compose up -d

# Check status
docker-compose ps
```

## Configuration

### 1. Create Configuration File

Create a `config/development.toml` file:

```toml
[server]
host = "127.0.0.1"
port = 8080
log_level = "debug"

[database]
type = "postgres"
host = "localhost"
port = 5432
username = "rcommerce_dev"
password = "devpass"
database = "rcommerce_dev"
pool_size = 5

[cache]
provider = "memory"  # Use in-memory cache for development

[payments]
default_gateway = "mock"

[logging]
level = "debug"
format = "text"

[features]
development_mode = true
debug_api = true
```

### 2. Set Up Database

**PostgreSQL:**

```bash
# Create database
psql -U postgres -c "CREATE DATABASE rcommerce_dev;"
psql -U postgres -c "CREATE USER rcommerce_dev WITH PASSWORD 'devpass';"
psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE rcommerce_dev TO rcommerce_dev;"
```

**MySQL:**

```bash
mysql -u root -p <<EOF
CREATE DATABASE rcommerce_dev;
CREATE USER 'rcommerce_dev'@'localhost' IDENTIFIED BY 'devpass';
GRANT ALL PRIVILEGES ON rcommerce_dev.* TO 'rcommerce_dev'@'localhost';
FLUSH PRIVILEGES;
EOF
```

**SQLite:**

No setup required - the database file will be created automatically.

### 3. Run Migrations

```bash
# Run database migrations
cargo run -- migrate run
```

## Running the Server

### Development Mode

```bash
# Run with hot reload
cargo watch -x run

# Or run directly
cargo run

# With specific config
cargo run -- --config config/development.toml
```

### Production Mode

```bash
# Build release binary
cargo build --release

# Run with production config
./target/release/rcommerce --config config/production.toml
```

## Verify Installation

### Health Check

```bash
curl http://localhost:8080/health
```

Expected response:

```json
{
  "status": "healthy",
  "version": "0.1.0",
  "database": "connected",
  "cache": "connected",
  "timestamp": "2024-01-23T14:13:35Z"
}
```

### Create Your First Product

```bash
curl -X POST http://localhost:8080/api/v1/products \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "name": "Test Product",
    "slug": "test-product",
    "description": "A test product",
    "price": 29.99,
    "status": "active"
  }'
```

## Next Steps

- [Installation Guide](installation.md) - Detailed installation instructions
- [Configuration Guide](configuration.md) - Complete configuration reference
- [API Reference](../api-reference/index.md) - Start building with the API
- [Development Guide](../development/index.md) - Set up your development environment

## Troubleshooting

### Port Already in Use

```bash
# Find process using port 8080
lsof -i :8080

# Kill the process or use a different port
# Edit config/development.toml and change the port
```

### Database Connection Failed

```bash
# Check PostgreSQL is running
pg_isready -h localhost -p 5432

# Check credentials
psql -U rcommerce_dev -d rcommerce_dev -h localhost -W
```

### Build Errors

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

## Getting Help

- **Documentation**: Browse the full documentation
- **GitHub Issues**: Report bugs and request features
- **Discord**: Join the community for real-time help
