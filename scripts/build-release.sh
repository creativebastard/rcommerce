#!/bin/bash
# Build release binaries for all platforms using cross

set -e

VERSION=${1:-0.1.0}
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
)

echo "Building R Commerce v$VERSION release binaries..."
echo ""

# Create release directory
mkdir -p release

# Build for each target
for target in "${TARGETS[@]}"; do
    echo "Building for $target..."
    
    if [[ "$target" == *"darwin"* ]]; then
        # macOS targets - build natively
        SQLX_OFFLINE=true cargo build --release --target "$target" -p rcommerce-cli
    else
        # Linux targets - use cross
        SQLX_OFFLINE=true cross build --release --target "$target" -p rcommerce-cli
    fi
    
    # Package the binary
    if [ -f "target/$target/release/rcommerce" ]; then
        cp "target/$target/release/rcommerce" "release/rcommerce-${VERSION}-${target}"
        gzip "release/rcommerce-${VERSION}-${target}"
        echo "  ✓ Built release/rcommerce-${VERSION}-${target}.gz"
    else
        echo "  ✗ Build failed for $target"
    fi
    echo ""
done

# Generate checksums
echo "Generating SHA256 checksums..."
cd release
sha256sum *.gz > SHA256SUMS.txt
cat SHA256SUMS.txt
cd ..

echo ""
echo "Release binaries built in ./release/"
