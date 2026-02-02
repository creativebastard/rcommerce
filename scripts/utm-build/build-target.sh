#!/bin/bash
# Build R Commerce for a specific target using UTM VM

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/vm-config.sh"

VM_KEY=$1
VERSION=${2:-0.1.0}

if [ -z "$VM_KEY" ]; then
    echo "Usage: $0 <vm-key> [version]"
    echo ""
    echo "Available VMs:"
    for key in $(list_vm_keys); do
        local name=$(get_vm_info "$key" name)
        local target=$(get_vm_info "$key" target)
        if vm_exists "$name"; then
            echo "  $key - $target"
        fi
    done
    exit 1
fi

VM_NAME=$(get_vm_info "$VM_KEY" name)
TARGET=$(get_vm_info "$VM_KEY" target)

if [ -z "$VM_NAME" ]; then
    echo "ERROR: Unknown VM key: $VM_KEY"
    exit 1
fi

if ! vm_exists "$VM_NAME"; then
    echo "ERROR: VM '$VM_NAME' does not exist"
    echo "Create it first with: ./create-vm.sh $VM_KEY"
    exit 1
fi

echo "========================================"
echo "Building R Commerce v$VERSION"
echo "Target: $TARGET"
echo "VM: $VM_NAME"
echo "========================================"
echo ""

# Start VM if not running
echo "Ensuring VM is running..."
"$SCRIPT_DIR/start-vm.sh" "$VM_KEY" >/dev/null 2>&1 || true

# Wait for VM to be reachable
echo "Waiting for VM to be reachable..."
MAX_RETRIES=30
RETRY_COUNT=0

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    VM_IP=$(get_vm_ip "$VM_NAME")
    
    if [ -n "$VM_IP" ]; then
        if ssh -q -o BatchMode=yes -o ConnectTimeout=5 "$SSH_USER@$VM_IP" exit 2>/dev/null; then
            echo "VM is reachable at $VM_IP"
            break
        fi
    fi
    
    RETRY_COUNT=$((RETRY_COUNT + 1))
    echo "  Waiting... ($RETRY_COUNT/$MAX_RETRIES)"
    sleep 10
done

if [ $RETRY_COUNT -eq $MAX_RETRIES ]; then
    echo "ERROR: VM did not become reachable"
    exit 1
fi

# Use the generic VM build script with our discovered IP
export VM_IP
export VM_PORT="$SSH_PORT"
export VM_USER="$SSH_USER"

# Create temporary config for vm-build system
TEMP_CONFIG="$SCRIPT_DIR/../vm-build/vms.conf"
mkdir -p "$(dirname "$TEMP_CONFIG")"

# Backup existing config if present
if [ -f "$TEMP_CONFIG" ]; then
    mv "$TEMP_CONFIG" "$TEMP_CONFIG.bak"
fi

# Write temporary config
echo "$TARGET,$VM_NAME,$VM_IP,$SSH_PORT,$SSH_USER,$HOME/.ssh/id_rsa" > "$TEMP_CONFIG"

# Run the build
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
"$SCRIPT_DIR/../vm-build/build-target.sh" "$TARGET" "$VERSION"
BUILD_STATUS=$?

# Restore original config
if [ -f "$TEMP_CONFIG.bak" ]; then
    mv "$TEMP_CONFIG.bak" "$TEMP_CONFIG"
else
    rm -f "$TEMP_CONFIG"
fi

exit $BUILD_STATUS
