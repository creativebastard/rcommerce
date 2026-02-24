#!/bin/bash
#
# Cross-Compilation Build Script for R Commerce
# 
# Uses:
# - Native cargo for macOS targets (faster, avoids zig issues with some C libs)
# - cargo-zigbuild for Linux/FreeBSD targets (cross-compilation)
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
PACKAGE="rcommerce-cli"
DIST_DIR="./dist"

# Targets
MACOS_TARGETS=("aarch64-apple-darwin" "x86_64-apple-darwin")
LINUX_GNU_TARGETS=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "armv7-unknown-linux-gnueabihf")
LINUX_MUSL_TARGETS=("x86_64-unknown-linux-musl" "aarch64-unknown-linux-musl")
FREEBSD_TARGETS=("x86_64-unknown-freebsd")
FREEBSD_TARGETS=("x86_64-unknown-freebsd")
# NetBSD has rustls/aws-lc-sys issues - disabled for now
# NETBSD_TARGETS=("x86_64-unknown-netbsd")

ALL_TARGETS=("${MACOS_TARGETS[@]}" "${LINUX_GNU_TARGETS[@]}" "${LINUX_MUSL_TARGETS[@]}" "${FREEBSD_TARGETS[@]}")

# Functions
print_header() {
    echo ""
    echo -e "${CYAN}╔══════════════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║              R COMMERCE CROSS-COMPILATION BUILD                              ║${NC}"
    echo -e "${CYAN}╚══════════════════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "Version: ${GREEN}${VERSION}${NC}"
    echo -e "Package: ${GREEN}${PACKAGE}${NC}"
    echo ""
}

check_prerequisites() {
    echo -e "${BLUE}ℹ Checking prerequisites...${NC}"
    
    # Check cargo
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}✗ cargo not found${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ cargo installed${NC}"
    
    # Check cargo-zigbuild (for Linux/FreeBSD)
    if ! command -v cargo-zigbuild &> /dev/null; then
        echo -e "${YELLOW}⚠ cargo-zigbuild not found (needed for Linux/BSD builds)${NC}"
        echo -e "  Install with: ${CYAN}cargo install cargo-zigbuild${NC}"
        HAS_ZIGBUILD=false
    else
        echo -e "${GREEN}✓ cargo-zigbuild installed${NC}"
        HAS_ZIGBUILD=true
    fi
    
    # Check zig
    if ! command -v zig &> /dev/null; then
        echo -e "${YELLOW}⚠ zig not found (needed for Linux/FreeBSD builds)${NC}"
        echo -e "  Install with: ${CYAN}brew install zig${NC}"
    else
        echo -e "${GREEN}✓ zig installed${NC}"
    fi
    
    # Check lipo (for macOS universal binary)
    if ! command -v lipo &> /dev/null; then
        echo -e "${YELLOW}⚠ lipo not found (needed for macOS universal binary)${NC}"
    else
        echo -e "${GREEN}✓ lipo available${NC}"
    fi
}

build_target() {
    local target=$1
    local use_zig=$2
    
    echo ""
    echo -e "${BLUE}▶ Building for: ${CYAN}${target}${NC}"
    
    local output_dir="target/${target}/release"
    local binary_name="rcommerce"
    local output_path="${output_dir}/${binary_name}"
    
    if [ "$use_zig" = true ]; then
        # Use cargo-zigbuild for cross-compilation
        if ! command -v cargo-zigbuild &> /dev/null; then
            echo -e "${RED}✗ cargo-zigbuild required for ${target}${NC}"
            return 1
        fi
        
        SQLX_OFFLINE=true cargo zigbuild --release --package "${PACKAGE}" --target "${target}" 2>&1 | while read line; do
            echo "  ${line}"
        done
    else
        # Use native cargo (for macOS targets on macOS host)
        SQLX_OFFLINE=true cargo build --release --package "${PACKAGE}" --target "${target}" 2>&1 | while read line; do
            echo "  ${line}"
        done
    fi
    
    if [ -f "${output_path}" ]; then
        local size=$(ls -lh "${output_path}" | awk '{print $5}')
        echo -e "${GREEN}✓ Build successful${NC} (${size})"
        return 0
    else
        echo -e "${RED}✗ Binary not found at ${output_path}${NC}"
        return 1
    fi
}

create_universal_binary() {
    echo ""
    echo -e "${BLUE}ℹ Creating macOS universal binary...${NC}"
    
    local arm64_bin="target/aarch64-apple-darwin/release/rcommerce"
    local x86_64_bin="target/x86_64-apple-darwin/release/rcommerce"
    local universal_bin="${DIST_DIR}/rcommerce-macos-universal"
    
    if [ ! -f "$arm64_bin" ]; then
        echo -e "${YELLOW}⚠ ARM64 binary not found, skipping universal binary${NC}"
        return 1
    fi
    
    if [ ! -f "$x86_64_bin" ]; then
        echo -e "${YELLOW}⚠ x86_64 binary not found, skipping universal binary${NC}"
        return 1
    fi
    
    mkdir -p "${DIST_DIR}"
    lipo -create "$arm64_bin" "$x86_64_bin" -output "$universal_bin"
    
    local size=$(ls -lh "$universal_bin" | awk '{print $5}')
    echo -e "${GREEN}✓ Universal binary created${NC} (${size})"
    echo "  Location: ${universal_bin}"
    
    # Verify
    echo "  Architectures: $(lipo -archs "$universal_bin")"
}

copy_artifacts() {
    echo ""
    echo -e "${BLUE}ℹ Copying build artifacts...${NC}"
    
    mkdir -p "${DIST_DIR}"
    
    # macOS native binaries
    if [ -f "target/aarch64-apple-darwin/release/rcommerce" ]; then
        cp "target/aarch64-apple-darwin/release/rcommerce" "${DIST_DIR}/rcommerce-macos-arm64"
        echo -e "${GREEN}✓${NC} rcommerce-macos-arm64"
    fi
    
    if [ -f "target/x86_64-apple-darwin/release/rcommerce" ]; then
        cp "target/x86_64-apple-darwin/release/rcommerce" "${DIST_DIR}/rcommerce-macos-x86_64"
        echo -e "${GREEN}✓${NC} rcommerce-macos-x86_64"
    fi
    
    # Linux GNU binaries
    for target in "${LINUX_GNU_TARGETS[@]}"; do
        if [ -f "target/${target}/release/rcommerce" ]; then
            local name=$(echo "$target" | sed 's/-unknown//g' | sed 's/-gnueabihf//g')
            cp "target/${target}/release/rcommerce" "${DIST_DIR}/rcommerce-${name}"
            echo -e "${GREEN}✓${NC} rcommerce-${name}"
        fi
    done
    
    # Linux MUSL binaries (static)
    for target in "${LINUX_MUSL_TARGETS[@]}"; do
        if [ -f "target/${target}/release/rcommerce" ]; then
            local name=$(echo "$target" | sed 's/-unknown//g' | sed 's/-musl//g')
            cp "target/${target}/release/rcommerce" "${DIST_DIR}/rcommerce-${name}-static"
            echo -e "${GREEN}✓${NC} rcommerce-${name}-static"
        fi
    done
    
    # FreeBSD binaries
    for target in "${FREEBSD_TARGETS[@]}"; do
        if [ -f "target/${target}/release/rcommerce" ]; then
            local name=$(echo "$target" | sed 's/-unknown//g')
            cp "target/${target}/release/rcommerce" "${DIST_DIR}/rcommerce-${name}"
            echo -e "${GREEN}✓${NC} rcommerce-${name}"
        fi
    done
    
    # NetBSD binaries (disabled - see https://github.com/rustls/rustls/issues/...)    
    # for target in "${NETBSD_TARGETS[@]}"; do
    #     if [ -f "target/${target}/release/rcommerce" ]; then
    #         local name=$(echo "$target" | sed 's/-unknown//g')
    #         cp "target/${target}/release/rcommerce" "${DIST_DIR}/rcommerce-${name}"
    #         echo -e "${GREEN}✓${NC} rcommerce-${name}"
    #     fi
    # done
}

generate_checksums() {
    echo ""
    echo -e "${BLUE}ℹ Creating checksums...${NC}"
    
    if command -v sha256sum &> /dev/null; then
        (cd "${DIST_DIR}" && sha256sum rcommerce-* > SHA256SUMS.txt)
    else
        (cd "${DIST_DIR}" && shasum -a 256 rcommerce-* > SHA256SUMS.txt)
    fi
    
    echo -e "${GREEN}✓ Checksums saved to SHA256SUMS.txt${NC}"
}

print_summary() {
    echo ""
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}✓ Build complete!${NC}"
    echo ""
    echo -e "Artifacts in ${CYAN}${DIST_DIR}/${NC}:"
    ls -lh "${DIST_DIR}"/rcommerce-* 2>/dev/null | awk '{print "  " $9 " (" $5 ")"}'
    echo ""
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════════${NC}"
}

# Main
main() {
    print_header
    check_prerequisites
    
    local targets_to_build=()
    local failed_targets=()
    local build_macos=false
    local build_linux=false
    local build_musl=false
    local build_freebsd=false
    
    # Parse arguments
    if [ $# -eq 0 ]; then
        # Build all by default
        build_macos=true
        build_linux=true
        build_musl=true
        build_freebsd=true
    else
        for arg in "$@"; do
            case "$arg" in
                --macos-only)
                    build_macos=true
                    ;;
                --linux-only)
                    build_linux=true
                    ;;
                --musl-only)
                    build_musl=true
                    ;;
                --freebsd-only)
                    build_freebsd=true
                    ;;
                --netbsd-only)
                    echo -e "${YELLOW}⚠ NetBSD cross-compilation disabled (aws-lc-sys compatibility issues)${NC}"
                    echo -e "  Build natively on NetBSD instead, or use FreeBSD target"
                    exit 0
                    ;;
                aarch64-apple-darwin|x86_64-apple-darwin|x86_64-unknown-linux-gnu|aarch64-unknown-linux-gnu|armv7-unknown-linux-gnueabihf|x86_64-unknown-linux-musl|aarch64-unknown-linux-musl|x86_64-unknown-freebsd)
                    targets_to_build+=("$arg")
                    ;;
                *)
                    echo -e "${RED}Unknown target or option: $arg${NC}"
                    echo "Usage: $0 [--macos-only|--linux-only|--musl-only|--freebsd-only|<target-triple>]"
                    exit 1
                    ;;
            esac
        done
    fi
    
    # Determine which targets to build
    if [ ${#targets_to_build[@]} -eq 0 ]; then
        if [ "$build_macos" = true ]; then
            targets_to_build+=("${MACOS_TARGETS[@]}")
        fi
        if [ "$build_linux" = true ]; then
            targets_to_build+=("${LINUX_GNU_TARGETS[@]}")
        fi
        if [ "$build_musl" = true ]; then
            targets_to_build+=("${LINUX_MUSL_TARGETS[@]}")
        fi
        if [ "$build_freebsd" = true ]; then
            targets_to_build+=("${FREEBSD_TARGETS[@]}")
        fi
        
    fi
    
    echo -e "${BLUE}ℹ Building ${#targets_to_build[@]} target(s)...${NC}"
    
    # Build each target
    for target in "${targets_to_build[@]}"; do
        # Determine if we should use zig for this target
        local use_zig=false
        case "$target" in
            *-linux-*|*-freebsd)
                use_zig=true
                ;;
        esac
        
        if ! build_target "$target" "$use_zig"; then
            failed_targets+=("$target")
        fi
    done
    
    # Create universal binary for macOS (only if both targets built successfully)
    if [ "$build_macos" = true ] && [[ ! " ${failed_targets[@]} " =~ "aarch64-apple-darwin" ]] && [[ ! " ${failed_targets[@]} " =~ "x86_64-apple-darwin" ]]; then
        create_universal_binary
    fi
    
    # Copy artifacts
    copy_artifacts
    
    # Generate checksums
    generate_checksums
    
    # Print summary
    print_summary
    
    # Report failures
    if [ ${#failed_targets[@]} -gt 0 ]; then
        echo ""
        echo -e "${RED}✗ Some builds failed:${NC}"
        for target in "${failed_targets[@]}"; do
            echo "  - $target"
        done
        exit 1
    fi
}

main "$@"
