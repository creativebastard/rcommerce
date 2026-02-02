#!/bin/bash
# Start a UTM VM

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
            echo "  $key - $name (exists)"
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
    echo "Create it first with: ./create-vm.sh $VM_KEY"
    exit 1
fi

echo "Starting VM: $VM_NAME"

# Use UTM's AppleScript interface to start VM
osascript << EOF
tell application "UTM"
    set vm to virtual machine named "$VM_NAME"
    if status of vm is stopped then
        start vm
        repeat while status of vm is not started
            delay 1
        end repeat
        return "VM started"
    else
        return "VM is already running"
    end if
end tell
EOF

echo ""
echo "VM '$VM_NAME' is starting..."
echo "Wait for it to fully boot before connecting"
echo ""

# Try to get IP address
sleep 5
VM_IP=$(get_vm_ip "$VM_NAME")

if [ -n "$VM_IP" ]; then
    echo "VM IP address: $VM_IP"
    echo "Connect with: ssh $SSH_USER@$VM_IP"
else
    echo "Could not determine VM IP address"
    echo "Check UTM console for IP address"
fi
