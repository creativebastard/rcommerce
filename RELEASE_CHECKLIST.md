# R Commerce Release Checklist

## Pre-Release Checklist

### Code Quality
- [ ] All tests pass: `cargo test --workspace`
- [ ] No compiler warnings: `cargo build --release`
- [ ] Code formatted: `cargo fmt`
- [ ] Clippy clean: `cargo clippy -- -D warnings`
- [ ] Documentation updated

### Version Updates
- [ ] Update version in `Cargo.toml` (root and all crates)
- [ ] Update `CHANGELOG.md`
- [ ] Update version in documentation
- [ ] Tag release: `git tag -a v0.1.0 -m "Release v0.1.0"`

### Testing
- [ ] Run MVP tests: `./scripts/test_api_mvp.sh`
- [ ] Test database migrations
- [ ] Test on clean database
- [ ] Verify all API endpoints work

## Building Release Binaries

### Option 1: Local Build (Current Platform)

```bash
# Build release binary
cargo build --release -p rcommerce-cli

# Binary location: target/release/rcommerce
```

### Option 2: Build Script (Host Platform)

```bash
./scripts/build-release.sh 0.1.0
```

### Option 3: GitHub Actions (All Platforms)

```bash
# Push tag to trigger GitHub Actions
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

## Platform-Specific Builds

### macOS

```bash
# Intel Mac
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin -p rcommerce-cli

# Apple Silicon
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin -p rcommerce-cli
```

### Linux

```bash
# Install cross for cross-compilation
cargo install cross --git https://github.com/cross-rs/cross

# x86_64
cross build --release --target x86_64-unknown-linux-gnu -p rcommerce-cli

# ARM64
cross build --release --target aarch64-unknown-linux-gnu -p rcommerce-cli
```

### FreeBSD

```bash
# Build natively on FreeBSD
cargo build --release -p rcommerce-cli

# Or use GitHub Actions (see .github/workflows/release.yml)
```

## Release Artifacts

Each release should include:

1. **Binaries** (compressed with gzip):
   - `rcommerce-{VERSION}-x86_64-macos.gz`
   - `rcommerce-{VERSION}-arm64-macos.gz`
   - `rcommerce-{VERSION}-x86_64-linux.gz`
   - `rcommerce-{VERSION}-arm64-linux.gz`
   - `rcommerce-{VERSION}-x86_64-freebsd.gz`

2. **Checksums**:
   - `SHA256SUMS.txt`

3. **Documentation**:
   - `README.md`
   - `CHANGELOG.md`
   - Installation instructions

## Docker Image

```bash
# Build Docker image
docker build -t rcommerce:{VERSION} .
docker tag rcommerce:{VERSION} rcommerce:latest

# Push to registry
docker push rcommerce:{VERSION}
docker push rcommerce:latest
```

## Post-Release

- [ ] Create GitHub Release with notes
- [ ] Upload binaries to release page
- [ ] Update documentation site
- [ ] Announce release
- [ ] Monitor for issues

## Binary Verification

After building, verify:

```bash
# Check binary works
./rcommerce --version

# Check architecture
file ./rcommerce

# Check dynamic libraries
otool -L ./rcommerce  # macOS
ldd ./rcommerce       # Linux
```

## Troubleshooting

### Build Failures

1. **Out of memory**: Use `cargo build --release -j1` to limit parallelism
2. **Missing dependencies**: Install PostgreSQL development libraries
3. **Cross-compilation fails**: Ensure Docker is running for `cross`

### Runtime Issues

1. **Library not found**: Binary may need static linking or dependencies installed
2. **Permission denied**: Ensure binary has execute permissions: `chmod +x rcommerce`

## Current Build Status

- **macOS x86_64**: ✅ Working (7.2 MB binary)
- **macOS ARM64**: ✅ Supported (cross-compile)
- **Linux x86_64**: ✅ Supported (cross-compile with `cross`)
- **Linux ARM64**: ✅ Supported (cross-compile with `cross`)
- **FreeBSD x86_64**: ✅ Supported (GitHub Actions or native build)
