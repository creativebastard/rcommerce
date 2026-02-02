#!/bin/bash
# Stop a UTM VM

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/vm-config.sh"

VM_KEY=$1
FORCE=${2:-false}

if [ -z "$VM_KEY" ]; then
    echo "Usage: $0 <vm-key> [force]"
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
    echo "ERROR: VM '$VM_NAME' does not exist"
    exit 1
fi

echo "Stopping VM: $VM_NAME"

if [ "$FORCE" = "true" ] || [ "$FORCE" = "force" ]; then
    # Force stop
    osascript << EOF
tell application "UTM"
    set vm to virtual machine named "$VM_NAME"
    if status of vm is started then
        stop vm force
        return "VM force stopped"
    else
        return "VM is not running"
    end if
end tell
EOF
else
    # Graceful stop
    osascript << EOF
tell application "UTM"
    set vm to virtual machine named "$VM_NAME"
    if status of vm is started then
        stop vm
        return "VM stopped"
    else
        return "VM is not running"
    end if
end tell
EOF
fi

echo "VM '$VM_NAME' stopped"
