# Local Development Setup

This guide walks you through setting up a local development environment for R Commerce.

## Prerequisites

- **Rust 1.70+** (install from [rustup.rs](https://rustup.rs/))
- **PostgreSQL 13+**
- **Redis 6+** (optional, for caching)
- **Git**

## Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/creativebastard/rcommerce.git
cd rcommerce
```

### 2. Set Up Database

#### Option A: PostgreSQL (Recommended)

```bash
# macOS
brew install postgresql@15
brew services start postgresql@15

# Create database
psql -U postgres -c "CREATE DATABASE rcommerce_dev;"
psql -U postgres -c "CREATE USER rcommerce_dev WITH PASSWORD 'devpass';"
psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE rcommerce_dev TO rcommerce_dev;"
```

### 3. Configure Environment

```bash
# Copy example configuration
cp config.example.toml config.development.toml

# Edit the configuration file
# Update database connection settings as needed
```

### 4. Build and Run

```bash
# Build the project
cargo build --release

# Run database migrations
cargo run --bin rcommerce -- db migrate

# Start the server
cargo run --bin rcommerce -- server
```

The API will be available at `http://localhost:8080`.

### 5. Verify Installation

```bash
# Health check
curl http://localhost:8080/health

# API info
curl http://localhost:8080/
```

## Development Configuration

### Database Configuration

**PostgreSQL:**
```toml
[database]
db_type = "Postgres"
host = "localhost"
port = 5432
database = "rcommerce_dev"
username = "rcommerce_dev"
password = "devpass"
```

### Optional: Redis Caching

```bash
# Install Redis
brew install redis  # macOS
brew services start redis

# Enable in config.toml
[cache]
cache_type = "Redis"
redis_url = "redis://localhost:6379"
```

## Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p rcommerce-core

# Run with output visible
cargo test -- --nocapture
```

## Development Tools

### Hot Reload

```bash
# Install cargo-watch
cargo install cargo-watch

# Watch for changes and rebuild
cargo watch -x 'run --bin rcommerce -- server'
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for security vulnerabilities
cargo audit
```

## IDE Setup

### VS Code

Recommended extensions:
- **rust-analyzer** - Rust language support
- **Even Better TOML** - TOML file support
- **CodeLLDB** - Debugging support

### IntelliJ / RustRover

- Install the Rust plugin
- Import the project as a Cargo project
- Set up run configurations for `cargo run` and `cargo test`

## Troubleshooting

### Build Failures

**Error: `ld: library not found for -lpq`**
```bash
# macOS: Install PostgreSQL client libraries
brew install libpq
brew link libpq --force
```

**Error: `sqlx` compile-time checks failing**
```bash
# Set SQLX_OFFLINE for builds without database
export SQLX_OFFLINE=true
cargo build
```

### Database Connection Issues

**PostgreSQL connection refused**
```bash
# Check PostgreSQL is running
brew services list | grep postgresql

# Restart PostgreSQL
brew services restart postgresql@15
```

### Port Already in Use

```bash
# Find process using port 8080
lsof -i :8080

# Kill the process or change port in config.toml
[server]
port = 8081
```

## Next Steps

- [CLI Reference](./cli-reference.md) - Learn the CLI commands
- [Testing Guide](./testing.md) - Write and run tests
- [Contributing Guide](./contributing.md) - Contribute to the project
