#!/bin/bash
#
# Build release binaries for multiple platforms
# Usage: ./scripts/build-release.sh [version]
# Default version is extracted from Cargo.toml

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get version from argument or Cargo.toml
VERSION="${1:-$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)}"
PROJECT_NAME="rcommerce"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Building R Commerce Release Binaries${NC}"
echo -e "${BLUE}Version: $VERSION${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Create release directory
RELEASE_DIR="release/${VERSION}"
mkdir -p "$RELEASE_DIR"

# Function to build for a target
build_target() {
    local target=$1
    local name=$2
    local ext="${3:-}"
    
    echo -e "${YELLOW}Building for $target...${NC}"
    
    if cargo build --release --target "$target" -p rcommerce-cli 2>/dev/null; then
        local binary_path="target/${target}/release/rcommerce${ext}"
        if [ -f "$binary_path" ]; then
            local output_name="${PROJECT_NAME}-${VERSION}-${name}${ext}"
            cp "$binary_path" "${RELEASE_DIR}/${output_name}"
            
            # Compress the binary
            if command -v gzip >/dev/null 2>&1; then
                gzip -c "${RELEASE_DIR}/${output_name}" > "${RELEASE_DIR}/${output_name}.gz"
                echo -e "${GREEN}✓ Built: ${output_name}.gz ($(du -h "${RELEASE_DIR}/${output_name}.gz" | cut -f1))${NC}"
            else
                echo -e "${GREEN}✓ Built: ${output_name} ($(du -h "${RELEASE_DIR}/${output_name}" | cut -f1))${NC}"
            fi
        else
            echo -e "${RED}✗ Binary not found: $binary_path${NC}"
        fi
    else
        echo -e "${RED}✗ Failed to build for $target${NC}"
        echo -e "${YELLOW}  You may need to install the target: rustup target add $target${NC}"
    fi
}

# Check if cross-compilation tools are available
check_cross_compile() {
    echo -e "${BLUE}Checking cross-compilation setup...${NC}"
    
    # Check for cross tool
    if command -v cross >/dev/null 2>&1; then
        echo -e "${GREEN}✓ cross tool found${NC}"
        USE_CROSS=true
    else
        echo -e "${YELLOW}⚠ cross tool not found. Install with: cargo install cross${NC}"
        USE_CROSS=false
    fi
    
    # Check for Docker (needed for cross)
    if command -v docker >/dev/null 2>&1; then
        echo -e "${GREEN}✓ Docker found${NC}"
    else
        echo -e "${YELLOW}⚠ Docker not found. Cross-compilation may not work.${NC}"
    fi
    echo ""
}

# Build for host platform (always works)
build_host() {
    echo -e "${BLUE}Building for host platform...${NC}"
    
    local host_target=$(rustc -vV | grep host | cut -d' ' -f2)
    local host_os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local host_arch=$(uname -m)
    
    echo -e "${YELLOW}Host: $host_target${NC}"
    
    cargo build --release -p rcommerce-cli
    
    local ext=""
    if [[ "$host_os" == "windows"* ]] || [[ "$host_os" == "msys"* ]]; then
        ext=".exe"
    fi
    
    local binary_path="target/release/rcommerce${ext}"
    local output_name="${PROJECT_NAME}-${VERSION}-${host_arch}-${host_os}${ext}"
    
    cp "$binary_path" "${RELEASE_DIR}/${output_name}"
    
    if command -v gzip >/dev/null 2>&1; then
        gzip -c "${RELEASE_DIR}/${output_name}" > "${RELEASE_DIR}/${output_name}.gz"
        echo -e "${GREEN}✓ Built: ${output_name}.gz ($(du -h "${RELEASE_DIR}/${output_name}.gz" | cut -f1))${NC}"
    else
        echo -e "${GREEN}✓ Built: ${output_name} ($(du -h "${RELEASE_DIR}/${output_name}" | cut -f1))${NC}"
    fi
    echo ""
}

# Build using cross tool
build_with_cross() {
    local target=$1
    local name=$2
    local ext="${3:-}"
    
    echo -e "${YELLOW}Building for $target using cross...${NC}"
    
    if cross build --release --target "$target" -p rcommerce-cli 2>/dev/null; then
        local binary_path="target/${target}/release/rcommerce${ext}"
        if [ -f "$binary_path" ]; then
            local output_name="${PROJECT_NAME}-${VERSION}-${name}${ext}"
            cp "$binary_path" "${RELEASE_DIR}/${output_name}"
            
            if command -v gzip >/dev/null 2>&1; then
                gzip -c "${RELEASE_DIR}/${output_name}" > "${RELEASE_DIR}/${output_name}.gz"
                echo -e "${GREEN}✓ Built: ${output_name}.gz ($(du -h "${RELEASE_DIR}/${output_name}.gz" | cut -f1))${NC}"
            else
                echo -e "${GREEN}✓ Built: ${output_name} ($(du -h "${RELEASE_DIR}/${output_name}" | cut -f1))${NC}"
            fi
        else
            echo -e "${RED}✗ Binary not found: $binary_path${NC}"
        fi
    else
        echo -e "${RED}✗ Failed to build for $target${NC}"
    fi
}

# Main build process
main() {
    check_cross_compile
    
    # Always build for host first
    build_host
    
    # Build for other platforms if cross is available
    if [ "$USE_CROSS" = true ]; then
        echo -e "${BLUE}Building for other platforms using cross...${NC}"
        
        # Linux x86_64
        build_with_cross "x86_64-unknown-linux-gnu" "x86_64-linux"
        
        # Linux ARM64
        build_with_cross "aarch64-unknown-linux-gnu" "arm64-linux"
        
        # macOS x86_64
        build_with_cross "x86_64-apple-darwin" "x86_64-macos"
        
        # macOS ARM64 (M1/M2)
        build_with_cross "aarch64-apple-darwin" "arm64-macos"
        
        # FreeBSD x86_64
        build_with_cross "x86_64-unknown-freebsd" "x86_64-freebsd"
        
    else
        echo -e "${YELLOW}Skipping cross-platform builds. Install cross for multi-platform builds:${NC}"
        echo -e "${YELLOW}  cargo install cross${NC}"
        echo ""
    fi
    
    # Generate checksums
    echo -e "${BLUE}Generating checksums...${NC}"
    cd "$RELEASE_DIR"
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum * > SHA256SUMS.txt
        echo -e "${GREEN}✓ SHA256SUMS.txt created${NC}"
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 * > SHA256SUMS.txt
        echo -e "${GREEN}✓ SHA256SUMS.txt created${NC}"
    else
        echo -e "${YELLOW}⚠ No checksum tool found${NC}"
    fi
    cd - >/dev/null
    
    echo ""
    echo -e "${BLUE}========================================${NC}"
    echo -e "${GREEN}Release build complete!${NC}"
    echo -e "${BLUE}Binaries location: ${RELEASE_DIR}/${NC}"
    echo -e "${BLUE}========================================${NC}"
    ls -lh "$RELEASE_DIR"
}

main
