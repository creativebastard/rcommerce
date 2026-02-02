#!/bin/bash
# Delete a UTM VM

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/vm-config.sh"

VM_KEY=$1

if [ -z "$VM_KEY" ]; then
    echo "Usage: $0 <vm-key>"
    echo ""
    echo "Available VMs:"
    for key in $(list_vm_keys); do
        local name=$(get_vm_info "$key" name)
        if vm_exists "$name"; then
            echo "  $key - $name"
        fi
    done
    exit 1
fi

VM_NAME=$(get_vm_info "$VM_KEY" name)

if [ -z "$VM_NAME" ]; then
    echo "ERROR: Unknown VM key: $VM_KEY"
    exit 1
fi

if ! vm_exists "$VM_NAME"; then
    echo "VM '$VM_NAME' does not exist"
    exit 0
fi

echo "WARNING: This will delete VM '$VM_NAME' and all its data"
read -p "Are you sure? (y/N) " -n 1 -r
echo

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted"
    exit 1
fi

echo "Stopping VM if running..."
"$SCRIPT_DIR/stop-vm.sh" "$VM_KEY" force 2>/dev/null || true

echo "Deleting VM..."

# Use UTM's AppleScript to delete
osascript << EOF
tell application "UTM"
    set vm to virtual machine named "$VM_NAME"
    delete vm
end tell
EOF

echo "VM '$VM_NAME' deleted"
