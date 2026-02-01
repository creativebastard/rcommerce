#!/bin/bash
# Build release binaries for all platforms using cross
# Usage: ./scripts/build-release.sh [VERSION]

set -e

VERSION=${1:-0.1.0}
RELEASE_DIR="release"

# All targets we want to build
LINUX_TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-gnu"
)

FREEBSD_TARGETS=(
    "x86_64-unknown-freebsd"
)

MACOS_TARGETS=(
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
)

ALL_TARGETS=("${LINUX_TARGETS[@]}" "${FREEBSD_TARGETS[@]}" "${MACOS_TARGETS[@]}")

echo "========================================"
echo "Building R Commerce v$VERSION"
echo "========================================"
echo ""

# Check for required tools
if ! command -v cross &> /dev/null; then
    echo "Error: 'cross' is not installed."
    echo "Install with: cargo install cross --git https://github.com/cross-rs/cross"
    exit 1
fi

# Create release directory
mkdir -p "$RELEASE_DIR"

# Function to build for a target
build_target() {
    local target=$1
    local use_cross=$2
    
    echo "Building for $target..."
    
    if [ "$use_cross" = "true" ]; then
        # Use cross for cross-compilation
        if ! SQLX_OFFLINE=true cross build --release --target "$target" -p rcommerce-cli 2>&1; then
            echo "  ✗ Failed to build for $target"
            return 1
        fi
    else
        # Native build
        if ! SQLX_OFFLINE=true cargo build --release --target "$target" -p rcommerce-cli 2>&1; then
            echo "  ✗ Failed to build for $target"
            return 1
        fi
    fi
    
    # Package the binary
    local binary_path="target/$target/release/rcommerce"
    if [ -f "$binary_path" ]; then
        local output_name="rcommerce-${VERSION}-${target}"
        cp "$binary_path" "$RELEASE_DIR/$output_name"
        gzip -f "$RELEASE_DIR/$output_name"
        echo "  ✓ Built $RELEASE_DIR/${output_name}.gz"
        return 0
    else
        echo "  ✗ Binary not found: $binary_path"
        return 1
    fi
}

# Track successes and failures
SUCCESS=()
FAILED=()

# Build Linux targets (using cross)
echo "--- Linux Targets ---"
for target in "${LINUX_TARGETS[@]}"; do
    if build_target "$target" "true"; then
        SUCCESS+=("$target")
    else
        FAILED+=("$target")
    fi
    echo ""
done

# Build FreeBSD targets (using cross)
echo "--- FreeBSD Targets ---"
for target in "${FREEBSD_TARGETS[@]}"; do
    if build_target "$target" "true"; then
        SUCCESS+=("$target")
    else
        FAILED+=("$target")
    fi
    echo ""
done

# Build macOS targets (native)
echo "--- macOS Targets ---"
for target in "${MACOS_TARGETS[@]}"; do
    if build_target "$target" "false"; then
        SUCCESS+=("$target")
    else
        FAILED+=("$target")
    fi
    echo ""
done

# Generate checksums
echo "--- Generating Checksums ---"
cd "$RELEASE_DIR"
if ls *.gz 1> /dev/null 2>&1; then
    sha256sum *.gz > SHA256SUMS.txt
    echo "SHA256 checksums:"
    cat SHA256SUMS.txt
else
    echo "No archives found"
fi
cd ..

echo ""
echo "========================================"
echo "Build Summary"
echo "========================================"
echo "Successful: ${#SUCCESS[@]}"
for t in "${SUCCESS[@]}"; do
    echo "  ✓ $t"
done

if [ ${#FAILED[@]} -gt 0 ]; then
    echo ""
    echo "Failed: ${#FAILED[@]}"
    for t in "${FAILED[@]}"; do
        echo "  ✗ $t"
    done
    exit 1
fi

echo ""
echo "Release binaries in ./$RELEASE_DIR/"
echo "Done!"
