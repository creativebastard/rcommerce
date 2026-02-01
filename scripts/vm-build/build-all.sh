#!/bin/bash
# Build R Commerce for all configured VM targets

set -e

VERSION=${1:-0.1.0}
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RELEASE_DIR="$PROJECT_ROOT/release"
CONFIG_FILE="$SCRIPT_DIR/vms.conf"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_section() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}"
}

log_section "R Commerce Release Build v$VERSION"

# Check if config file exists
if [ ! -f "$CONFIG_FILE" ]; then
    log_error "VM configuration file not found: $CONFIG_FILE"
    log_info "Copy vms.conf.example to vms.conf and configure your VMs"
    exit 1
fi

# Create release directory
mkdir -p "$RELEASE_DIR"
rm -f "$RELEASE_DIR/SHA256SUMS.txt"

# Read all targets from config file
TARGETS=$(grep -v '^#' "$CONFIG_FILE" | grep -v '^$' | cut -d',' -f1)

if [ -z "$TARGETS" ]; then
    log_error "No targets configured in $CONFIG_FILE"
    exit 1
fi

# Track results
SUCCESS=()
FAILED=()

# Build for each target
for TARGET in $TARGETS; do
    log_section "Building: $TARGET"
    
    if "$SCRIPT_DIR/build-target.sh" "$TARGET" "$VERSION"; then
        SUCCESS+=("$TARGET")
    else
        FAILED+=("$TARGET")
        log_error "Build failed for $TARGET"
    fi
    echo ""
done

# Summary
log_section "Build Summary"

echo -e "${GREEN}Successful builds: ${#SUCCESS[@]}${NC}"
for t in "${SUCCESS[@]}"; do
    echo -e "  ${GREEN}✓${NC} $t"
done

if [ ${#FAILED[@]} -gt 0 ]; then
    echo ""
    echo -e "${RED}Failed builds: ${#FAILED[@]}${NC}"
    for t in "${FAILED[@]}"; do
        echo -e "  ${RED}✗${NC} $t"
    done
fi

# Show checksums
echo ""
log_info "SHA256 Checksums:"
cat "$RELEASE_DIR/SHA256SUMS.txt" 2>/dev/null || echo "No checksums generated"

echo ""
log_info "Release artifacts in: $RELEASE_DIR/"
ls -lh "$RELEASE_DIR/"/*.gz 2>/dev/null || echo "No artifacts found"

if [ ${#FAILED[@]} -gt 0 ]; then
    exit 1
fi

log_info "All builds completed successfully!"
