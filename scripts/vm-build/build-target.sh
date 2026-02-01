#!/bin/bash
# Build R Commerce for a specific target using a VM

set -e

TARGET=$1
VERSION=${2:-0.1.0}
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RELEASE_DIR="$PROJECT_ROOT/release"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
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

# Check if target is specified
if [ -z "$TARGET" ]; then
    echo "Usage: $0 <target> [version]"
    echo ""
    echo "Supported targets:"
    echo "  x86_64-unknown-linux-gnu"
    echo "  x86_64-unknown-linux-musl"
    echo "  aarch64-unknown-linux-gnu"
    echo "  x86_64-unknown-freebsd"
    exit 1
fi

# Read VM configuration
CONFIG_FILE="$SCRIPT_DIR/vms.conf"
if [ ! -f "$CONFIG_FILE" ]; then
    log_error "VM configuration file not found: $CONFIG_FILE"
    log_info "Copy vms.conf.example to vms.conf and configure your VMs"
    exit 1
fi

# Parse VM config for target
VM_INFO=$(grep "^$TARGET," "$CONFIG_FILE" 2>/dev/null || true)
if [ -z "$VM_INFO" ]; then
    log_error "No VM configured for target: $TARGET"
    log_info "Add the VM configuration to $CONFIG_FILE"
    exit 1
fi

# Parse VM details
IFS=',' read -r VM_TARGET VM_NAME VM_IP VM_PORT VM_USER VM_KEY <<< "$VM_INFO"

log_info "Building for target: $TARGET"
log_info "Using VM: $VM_NAME ($VM_IP:$VM_PORT)"

# Expand SSH key path
VM_KEY="${VM_KEY/#\~/$HOME}"

# Check SSH connectivity
log_info "Checking VM connectivity..."
if ! ssh -q -o BatchMode=yes -o ConnectTimeout=5 -p "$VM_PORT" -i "$VM_KEY" "$VM_USER@$VM_IP" exit 2>/dev/null; then
    log_error "Cannot connect to VM at $VM_IP:$VM_PORT"
    log_info "Make sure the VM is running and SSH key is configured"
    exit 1
fi
log_info "VM is reachable"

# Create remote build directory
REMOTE_BUILD_DIR="/tmp/rcommerce-build-$VERSION"
log_info "Setting up remote build directory..."
ssh -p "$VM_PORT" -i "$VM_KEY" "$VM_USER@$VM_IP" "mkdir -p $REMOTE_BUILD_DIR && rm -rf $REMOTE_BUILD_DIR/*"

# Copy source code to VM
log_info "Copying source code to VM..."
rsync -az --exclude='target' --exclude='.git' --exclude='release' \
    -e "ssh -p $VM_PORT -i $VM_KEY" \
    "$PROJECT_ROOT/" "$VM_USER@$VM_IP:$REMOTE_BUILD_DIR/"

# Build on VM
log_info "Building on VM (this may take a while)..."
ssh -p "$VM_PORT" -i "$VM_KEY" "$VM_USER@$VM_IP" << EOF
    cd $REMOTE_BUILD_DIR
    
    # Ensure Rust is in PATH
    source \$HOME/.cargo/env 2>/dev/null || true
    
    # Verify target is installed
    rustup target add $TARGET 2>/dev/null || true
    
    # Build
    echo "Starting build for $TARGET..."
    SQLX_OFFLINE=true cargo build --release --target $TARGET -p rcommerce-cli
    
    # Check if build succeeded
    if [ ! -f "target/$TARGET/release/rcommerce" ]; then
        echo "Build failed - binary not found"
        exit 1
    fi
    
    echo "Build completed successfully"
EOF

if [ $? -ne 0 ]; then
    log_error "Build failed on VM"
    exit 1
fi

# Copy binary back
mkdir -p "$RELEASE_DIR"
OUTPUT_NAME="rcommerce-${VERSION}-${TARGET}"
log_info "Copying binary back to host..."
scp -P "$VM_PORT" -i "$VM_KEY" \
    "$VM_USER@$VM_IP:$REMOTE_BUILD_DIR/target/$TARGET/release/rcommerce" \
    "$RELEASE_DIR/$OUTPUT_NAME"

# Compress
gzip -f "$RELEASE_DIR/$OUTPUT_NAME"

# Cleanup remote build directory
log_info "Cleaning up remote build directory..."
ssh -p "$VM_PORT" -i "$VM_KEY" "$VM_USER@$VM_IP" "rm -rf $REMOTE_BUILD_DIR"

log_info "Build complete: $RELEASE_DIR/${OUTPUT_NAME}.gz"

# Generate checksum
cd "$RELEASE_DIR"
sha256sum "${OUTPUT_NAME}.gz" >> SHA256SUMS.txt

log_info "Done!"
