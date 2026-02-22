#!/bin/bash
#
# Cross-compilation build script for R Commerce
# Uses cargo-zigbuild for cross-compilation without Docker
#
# Supports:
# - macOS (Apple Silicon, Intel)
# - Linux (x86_64, aarch64, armv7) - GNU and MUSL variants
# - FreeBSD (x86_64)
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Build configuration
PACKAGE="rcommerce-cli"
BINARY_NAME="rcommerce"
VERSION=$(grep "^version" Cargo.toml | head -1 | sed 's/.*= "\(.*\)".*/\1/')
BUILD_DIR="./dist"

# Targets to build
TARGETS=(
    # macOS
    "aarch64-apple-darwin"
    "x86_64-apple-darwin"
    
    # Linux (GNU)
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "armv7-unknown-linux-gnueabihf"
    
    # Linux (MUSL - static binaries)
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-musl"
    
    # FreeBSD
    "x86_64-unknown-freebsd"
)

print_header() {
    echo -e "${BLUE}"
    echo "╔══════════════════════════════════════════════════════════════════════════════╗"
    echo "║              R COMMERCE CROSS-COMPILATION BUILD                              ║"
    echo "╚══════════════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    echo "Version: $VERSION"
    echo "Package: $PACKAGE"
    echo ""
}

print_target() {
    echo -e "${YELLOW}▶ Building for: $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

check_prerequisites() {
    print_info "Checking prerequisites..."
    
    # Check cargo-zigbuild
    if ! command -v cargo-zigbuild &> /dev/null; then
        print_error "cargo-zigbuild not found. Install with: cargo install cargo-zigbuild"
        exit 1
    fi
    print_success "cargo-zigbuild installed"
    
    # Check zig
    if ! command -v zig &> /dev/null; then
        print_error "zig not found. Install with: brew install zig"
        exit 1
    fi
    print_success "zig installed"
    
    # Create build directory
    mkdir -p "$BUILD_DIR"
}

build_target() {
    local target=$1
    local features=""
    
    print_target "$target"
    
    # Determine output filename
    local os=""
    local arch=""
    local libc=""
    
    case "$target" in
        *-apple-darwin)
            os="macos"
            ;;
        *-linux-*)
            os="linux"
            ;;
        *-freebsd)
            os="freebsd"
            ;;
    esac
    
    case "$target" in
        x86_64-*)
            arch="x86_64"
            ;;
        aarch64-*)
            arch="aarch64"
            ;;
        armv7-*)
            arch="armv7"
            ;;
    esac
    
    case "$target" in
        *-musl)
            libc="musl"
            ;;
        *-gnu*)
            libc="gnu"
            ;;
    esac
    
    # Build output name
    local output_name="${BINARY_NAME}-${VERSION}-${os}-${arch}"
    if [ -n "$libc" ]; then
        output_name="${output_name}-${libc}"
    fi
    
    # Set environment variables for SQLx offline mode
    export SQLX_OFFLINE=true
    
    # Build
    local start_time=$(date +%s)
    
    if cargo zigbuild --release --package "$PACKAGE" --target "$target" $features 2>&1 | tee "/tmp/build-${target}.log"; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        
        # Copy binary
        local source_path="target/${target}/release/${BINARY_NAME}"
        if [ "$target" = "aarch64-apple-darwin" ] || [ "$target" = "x86_64-apple-darwin" ]; then
            # Native builds use different path
            source_path="target/release/${BINARY_NAME}"
        fi
        
        local dest_path="${BUILD_DIR}/${output_name}"
        
        if [ -f "$source_path" ]; then
            cp "$source_path" "$dest_path"
            chmod +x "$dest_path"
            
            # Get file size
            local size=$(du -h "$dest_path" | cut -f1)
            print_success "Built in ${duration}s (${size}) -> ${output_name}"
        else
            print_error "Binary not found at $source_path"
            return 1
        fi
    else
        print_error "Build failed for $target"
        return 1
    fi
}

build_macos_universal() {
    print_info "Creating macOS universal binary..."
    
    local arm64_path="${BUILD_DIR}/${BINARY_NAME}-${VERSION}-macos-aarch64"
    local x86_64_path="${BUILD_DIR}/${BINARY_NAME}-${VERSION}-macos-x86_64"
    local universal_path="${BUILD_DIR}/${BINARY_NAME}-${VERSION}-macos-universal"
    
    if [ -f "$arm64_path" ] && [ -f "$x86_64_path" ]; then
        if lipo -create -output "$universal_path" "$arm64_path" "$x86_64_path"; then
            local size=$(du -h "$universal_path" | cut -f1)
            print_success "Universal binary created (${size}) -> ${BINARY_NAME}-${VERSION}-macos-universal"
        else
            print_error "Failed to create universal binary"
        fi
    else
        print_info "Skipping universal binary (missing components)"
    fi
}

create_checksums() {
    print_info "Creating checksums..."
    
    cd "$BUILD_DIR"
    
    # Create SHA256 checksums
    sha256sum * > SHA256SUMS.txt 2>/dev/null || shasum -a 256 * > SHA256SUMS.txt
    
    print_success "Checksums saved to SHA256SUMS.txt"
    
    cd - > /dev/null
}

show_usage() {
    cat << EOF
Usage: $0 [OPTIONS] [TARGETS...]

Cross-compilation build script for R Commerce

OPTIONS:
    -h, --help          Show this help
    -l, --list          List available targets
    --macos-only        Build only macOS targets
    --linux-only        Build only Linux targets
    --musl-only         Build only MUSL (static) targets

TARGETS:
    Specific target triples to build (e.g., x86_64-unknown-linux-musl)

EXAMPLES:
    # Build all targets
    $0

    # Build specific target
    $0 x86_64-unknown-linux-musl

    # Build all macOS targets
    $0 --macos-only

    # Build static Linux binaries only
    $0 --musl-only

EOF
}

list_targets() {
    echo "Available targets:"
    for target in "${TARGETS[@]}"; do
        echo "  - $target"
    done
}

# Parse arguments
BUILD_TARGETS=()

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_usage
            exit 0
            ;;
        -l|--list)
            list_targets
            exit 0
            ;;
        --macos-only)
            BUILD_TARGETS=("aarch64-apple-darwin" "x86_64-apple-darwin")
            shift
            ;;
        --linux-only)
            BUILD_TARGETS=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "armv7-unknown-linux-gnueabihf")
            shift
            ;;
        --musl-only)
            BUILD_TARGETS=("x86_64-unknown-linux-musl" "aarch64-unknown-linux-musl")
            shift
            ;;
        --freebsd-only)
            BUILD_TARGETS=("x86_64-unknown-freebsd")
            shift
            ;;
        -*)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
        *)
            BUILD_TARGETS+=("$1")
            shift
            ;;
    esac
done

# If no targets specified, build all
if [ ${#BUILD_TARGETS[@]} -eq 0 ]; then
    BUILD_TARGETS=("${TARGETS[@]}")
fi

# Main execution
print_header
check_prerequisites

print_info "Building ${#BUILD_TARGETS[@]} target(s)..."
echo ""

FAILED_TARGETS=()
for target in "${BUILD_TARGETS[@]}"; do
    if ! build_target "$target"; then
        FAILED_TARGETS+=("$target")
    fi
    echo ""
done

# Create macOS universal binary
build_macos_universal

# Create checksums
create_checksums

echo ""
echo "═══════════════════════════════════════════════════════════════════"
if [ ${#FAILED_TARGETS[@]} -eq 0 ]; then
    print_success "All builds completed successfully!"
    print_info "Binaries location: ${BUILD_DIR}/"
    print_info "Checksums: ${BUILD_DIR}/SHA256SUMS.txt"
    exit 0
else
    print_error "Some builds failed:"
    for target in "${FAILED_TARGETS[@]}"; do
        echo "  - $target"
    done
    exit 1
fi
