#!/bin/bash
# Create all UTM VMs for R Commerce builds

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/vm-config.sh"

echo "========================================"
echo "Creating UTM VMs for R Commerce"
echo "========================================"
echo ""

# Check UTM is installed
if ! check_utm; then
    exit 1
fi

# Check SSH key exists
if [ ! -f "$HOME/.ssh/id_rsa.pub" ]; then
    echo "ERROR: SSH public key not found at ~/.ssh/id_rsa.pub"
    echo "Generate one with: ssh-keygen -t rsa -b 4096"
    exit 1
fi

# Create each VM
for VM_KEY in $(list_vm_keys); do
    VM_NAME=$(get_vm_info "$VM_KEY" name)
    
    if vm_exists "$VM_NAME"; then
        echo "VM '$VM_NAME' already exists, skipping"
        continue
    fi
    
    echo ""
    echo "========================================"
    echo "Creating VM: $VM_KEY"
    echo "========================================"
    
    if "$SCRIPT_DIR/create-vm.sh" "$VM_KEY"; then
        echo "✓ VM created: $VM_NAME"
    else
        echo "✗ Failed to create VM: $VM_NAME"
    fi
done

echo ""
echo "========================================"
echo "VM Creation Complete"
echo "========================================"
echo ""
echo "Next steps:"
echo "1. Start each VM and complete OS installation"
echo "   ./start-vm.sh <vm-key>"
echo ""
echo "2. After OS installation, run setup:"
echo "   ./setup-vm.sh <vm-key>"
echo ""
echo "3. Once all VMs are ready, build releases:"
echo "   ./build-all.sh 0.1.0"
