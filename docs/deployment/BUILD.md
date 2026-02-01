# Building R Commerce Binaries

This guide covers building R Commerce binaries for different platforms.

## Quick Start

### Build for Current Platform (Host)

```bash
# Build release binary
cargo build --release -p rcommerce-cli

# Binary location: target/release/rcommerce
```

### Using the Build Script

```bash
# Build for host platform
./scripts/build-release.sh

# Build with specific version
./scripts/build-release.sh 0.1.0
```

## Supported Platforms

| Platform | Architecture | Target Triple |
|----------|--------------|---------------|
| macOS | Intel (x86_64) | `x86_64-apple-darwin` |
| macOS | Apple Silicon (ARM64) | `aarch64-apple-darwin` |
| Linux | x86_64 | `x86_64-unknown-linux-gnu` |
| Linux | ARM64 | `aarch64-unknown-linux-gnu` |
| FreeBSD | x86_64 | `x86_64-unknown-freebsd` |

## Prerequisites

### All Platforms

- [Rust](https://rustup.rs/) 1.70.0 or later
- PostgreSQL 14+ (for runtime)

### macOS

```bash
# Install Xcode command line tools
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add targets for cross-compilation
rustup target add aarch64-apple-darwin  # For Apple Silicon
```

### Linux

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# For cross-compilation, install cross
cargo install cross --git https://github.com/cross-rs/cross
```

### FreeBSD

```bash
# Install Rust
pkg install rust

# Or use rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Cross-Compilation

### Using `cross` (Recommended for Linux)

The `cross` tool uses Docker containers for cross-compilation:

```bash
# Install cross
cargo install cross --git https://github.com/cross-rs/cross

# Build for Linux ARM64
cross build --release --target aarch64-unknown-linux-gnu -p rcommerce-cli

# Build for Linux x86_64
cross build --release --target x86_64-unknown-linux-gnu -p rcommerce-cli
```

### Using Docker

```bash
# Build Linux binary using Docker
docker build -f Dockerfile.build --target export --output type=local,dest=./release .

# The binary will be in ./release/rcommerce
```

### macOS Cross-Compilation

macOS can cross-compile between Intel and Apple Silicon:

```bash
# On Intel Mac, build for Apple Silicon
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin -p rcommerce-cli

# On Apple Silicon Mac, build for Intel
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin -p rcommerce-cli
```

### FreeBSD Cross-Compilation

FreeBSD cross-compilation requires special setup. Options:

1. **Use a FreeBSD VM** (recommended)
2. **Use GitHub Actions** (see `.github/workflows/release.yml`)
3. **Build natively on FreeBSD**

## Automated Builds with GitHub Actions

The repository includes a GitHub Actions workflow that automatically builds binaries for all platforms when you push a tag:

```bash
# Create a new version tag
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0
```

The workflow will:
1. Build for macOS (Intel and Apple Silicon)
2. Build for Linux (x86_64 and ARM64)
3. Build for FreeBSD (x86_64)
4. Create a GitHub Release with all binaries
5. Generate SHA256 checksums

## Binary Sizes

Typical release binary sizes (compressed):

| Platform | Size |
|----------|------|
| macOS Intel | ~8 MB |
| macOS Apple Silicon | ~8 MB |
| Linux x86_64 | ~10 MB |
| Linux ARM64 | ~10 MB |
| FreeBSD x86_64 | ~10 MB |

## Verification

After building, verify the binary:

```bash
# Check binary info
file rcommerce

# Check dynamic libraries (macOS/Linux)
ldd rcommerce        # Linux
otool -L rcommerce   # macOS

# Test the binary
./rcommerce --version
./rcommerce server --help
```

## Troubleshooting

### "linker 'cc' not found"

Install a C compiler:
- macOS: `xcode-select --install`
- Ubuntu/Debian: `sudo apt-get install build-essential`
- FreeBSD: `pkg install gcc`

### OpenSSL errors

Install OpenSSL development libraries:
- Ubuntu/Debian: `sudo apt-get install libssl-dev pkg-config`
- macOS: OpenSSL is included with Xcode
- FreeBSD: `pkg install openssl`

### Cross-compilation failures

1. Ensure the target is installed: `rustup target add <target>`
2. For Linux cross-compilation, use `cross` instead of `cargo`
3. Check Docker is running (required for `cross`)

### Binary won't run on target system

The binary may be dynamically linked. Either:
1. Build a static binary (see below)
2. Ensure target system has required libraries

## Advanced: Static Linking

For fully static Linux binaries:

```bash
# Install musl target
rustup target add x86_64-unknown-linux-musl

# Build with musl (static linking)
cargo build --release --target x86_64-unknown-linux-musl -p rcommerce-cli
```

Note: Static linking with musl may have performance implications and PostgreSQL client libraries may still require dynamic linking.

## Distribution

### Creating a Release Archive

```bash
VERSION="0.1.0"
PLATFORM="x86_64-linux"

tar czf rcommerce-${VERSION}-${PLATFORM}.tar.gz \
    rcommerce \
    README.md \
    LICENSE \
    config/
```

### Docker Image

```bash
# Build Docker image
docker build -t rcommerce:latest .

# Run container
docker run -p 8080:8080 -e DATABASE_URL=postgres://... rcommerce:latest
```

## Next Steps

After building:
1. Test the binary: `./rcommerce server`
2. Set up configuration (see [Configuration Guide](CONFIGURATION.md))
3. Deploy to your infrastructure (see [Deployment Guide](DEPLOYMENT.md))
