#!/bin/bash
# Setup a UTM VM after OS installation

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
TARGET=$(get_vm_info "$VM_KEY" target)
OS_TYPE=$(get_vm_info "$VM_KEY" os)
SETUP_SCRIPT=$(get_vm_info "$VM_KEY" setup)

if [ -z "$VM_NAME" ]; then
    echo "ERROR: Unknown VM key: $VM_KEY"
    exit 1
fi

if ! vm_exists "$VM_NAME"; then
    echo "ERROR: VM '$VM_NAME' does not exist"
    exit 1
fi

echo "Setting up VM: $VM_NAME"
echo "Target: $TARGET"
echo "OS Type: $OS_TYPE"
echo ""

# Start VM if not running
echo "Starting VM..."
"$SCRIPT_DIR/start-vm.sh" "$VM_KEY" >/dev/null 2>&1 || true

# Wait for VM to be reachable
echo "Waiting for VM to be reachable..."
MAX_RETRIES=30
RETRY_COUNT=0

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    VM_IP=$(get_vm_ip "$VM_NAME")
    
    if [ -n "$VM_IP" ]; then
        # Try SSH with password first (initial setup)
        if sshpass -p "$SSH_PASSWORD" ssh -q -o StrictHostKeyChecking=no -o ConnectTimeout=5 "$SSH_USER@$VM_IP" exit 2>/dev/null; then
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

echo ""
echo "Installing SSH key..."

# Copy SSH key to VM
sshpass -p "$SSH_PASSWORD" ssh-copy-id -o StrictHostKeyChecking=no "$SSH_USER@$VM_IP" 2>/dev/null || true

echo ""
echo "Running setup script on VM..."

# Run the appropriate setup script based on OS
ssh -o StrictHostKeyChecking=no "$SSH_USER@$VM_IP" << EOF
    # Create builder user if doesn't exist
    if ! id "$SSH_USER" &>/dev/null; then
        sudo useradd -m -s /bin/bash "$SSH_USER"
        echo "$SSH_USER:$SSH_PASSWORD" | sudo chpasswd
        echo "$SSH_USER ALL=(ALL) NOPASSWD:ALL" | sudo tee /etc/sudoers.d/99-$SSH_USER
    fi
    
    # Install Rust and dependencies
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source \$HOME/.cargo/env
    rustup target add $TARGET
    
    # Install OS-specific dependencies
    if command -v apt-get &> /dev/null; then
        # Debian/Ubuntu
        sudo apt-get update
        sudo apt-get install -y build-essential libssl-dev pkg-config curl git rsync
    elif command -v apk &> /dev/null; then
        # Alpine
        sudo apk add --no-cache build-base openssl-dev pkgconfig curl git rsync musl-dev
    elif command -v pkg &> /dev/null; then
        # FreeBSD
        sudo pkg install -y rust curl openssl git rsync
    fi
    
    echo "Setup complete!"
    rustc --version
    cargo --version
EOF

echo ""
echo "VM setup complete!"
echo "You can now build with: ./build-target.sh $VM_KEY"
