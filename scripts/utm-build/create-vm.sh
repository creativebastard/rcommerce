#!/bin/bash
# Create a UTM VM for building R Commerce

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/vm-config.sh"

VM_KEY=$1

if [ -z "$VM_KEY" ]; then
    echo "Usage: $0 <vm-key>"
    echo ""
    echo "Available VM keys:"
    for key in $(list_vm_keys); do
        local name=$(get_vm_info "$key" name)
        local target=$(get_vm_info "$key" target)
        echo "  $key - $name ($target)"
    done
    exit 1
fi

# Get VM details
VM_NAME=$(get_vm_info "$VM_KEY" name)
TARGET=$(get_vm_info "$VM_KEY" target)
OS_TYPE=$(get_vm_info "$VM_KEY" os)
ARCH=$(get_vm_info "$VM_KEY" arch)
DOWNLOAD_URL=$(get_vm_info "$VM_KEY" url)
ISO_FILENAME=$(get_vm_info "$VM_KEY" iso)

if [ -z "$VM_NAME" ]; then
    echo "ERROR: Unknown VM key: $VM_KEY"
    exit 1
fi

echo "Creating VM: $VM_NAME"
echo "Target: $TARGET"
echo "OS: $OS_TYPE ($ARCH)"
echo ""

# Check UTM is installed
if ! check_utm; then
    exit 1
fi

# Check if VM already exists
if vm_exists "$VM_NAME"; then
    echo "WARNING: VM '$VM_NAME' already exists"
    read -p "Delete and recreate? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Deleting existing VM..."
        "$SCRIPT_DIR/delete-vm.sh" "$VM_KEY"
    else
        echo "Aborting"
        exit 1
    fi
fi

# Create ISO cache directory
mkdir -p "$ISO_CACHE_DIR"

# Download ISO if not present
ISO_PATH="$ISO_CACHE_DIR/$ISO_FILENAME"
if [ ! -f "$ISO_PATH" ]; then
    echo "Downloading ISO..."
    echo "URL: $DOWNLOAD_URL"
    curl -L -o "$ISO_PATH" "$DOWNLOAD_URL"
else
    echo "Using cached ISO: $ISO_PATH"
fi

# Create VM directory
VM_PATH="$UTM_DIR/$VM_NAME.utm"
mkdir -p "$VM_PATH"

echo "Creating VM at: $VM_PATH"

# Create UTM configuration
# Note: This creates a basic VM config. UTM's AppleScript interface
# would be better for full automation

cat > "$VM_PATH/config.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>ConfigurationVersion</key>
    <integer>2</integer>
    <key>Display</key>
    <dict>
        <key>Width</key>
        <integer>1024</integer>
        <key>Height</key>
        <integer>768</integer>
    </dict>
    <key>System</key>
    <dict>
        <key>Architecture</key>
        <string>$ARCH</string>
        <key>CPUCount</key>
        <integer>$VM_CPU_COUNT</integer>
        <key>MemorySize</key>
        <integer>$VM_MEMORY_MB</integer>
        <key>BootDevice</key>
        <string>$ISO_PATH</string>
    </dict>
    <key>Network</key>
    <dict>
        <key>NetworkEnabled</key>
        <true/>
        <key>NetworkMode</key>
        <string>shared</string>
    </dict>
    <key>Serial</key>
    <dict>
        <key>Enabled</key>
        <true/>
    </dict>
</dict>
</plist>
EOF

# Create disk image
echo "Creating disk image (${VM_DISK_GB}GB)..."
DISK_PATH="$VM_PATH/Data.qcow2"
qemu-img create -f qcow2 "$DISK_PATH" ${VM_DISK_GB}G

echo ""
echo "VM created successfully!"
echo ""
echo "Next steps:"
echo "1. Start the VM: ./start-vm.sh $VM_KEY"
echo "2. Complete OS installation manually"
echo "3. Run setup script: ./setup-vm.sh $VM_KEY"
echo ""
echo "VM Location: $VM_PATH"
