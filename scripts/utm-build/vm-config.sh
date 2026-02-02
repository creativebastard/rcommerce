#!/bin/bash
# VM configuration for UTM-based builds

# VM Definitions
# Format: TARGET|VM_NAME|OS_TYPE|ARCH|DOWNLOAD_URL|ISO_FILENAME|SETUP_SCRIPT

declare -A VMS=(
    ["ubuntu-x64"]="x86_64-unknown-linux-gnu|rcommerce-ubuntu-x64|linux|x86_64|https://releases.ubuntu.com/22.04/ubuntu-22.04.3-live-server-amd64.iso|ubuntu-22.04.3-live-server-amd64.iso|setup-debian.sh"
    
    ["alpine-x64"]="x86_64-unknown-linux-musl|rcommerce-alpine-x64|linux|x86_64|https://dl-cdn.alpinelinux.org/alpine/v3.18/releases/x86_64/alpine-standard-3.18.4-x86_64.iso|alpine-standard-3.18.4-x86_64.iso|setup-alpine.sh"
    
    ["ubuntu-arm"]="aarch64-unknown-linux-gnu|rcommerce-ubuntu-arm|linux|arm64|https://cdimage.ubuntu.com/ubuntu-server/jammy/daily-live/current/jammy-live-server-arm64.iso|jammy-live-server-arm64.iso|setup-debian.sh"
    
    ["freebsd-x64"]="x86_64-unknown-freebsd|rcommerce-freebsd-x64|freebsd|x86_64|https://download.freebsd.org/ftp/releases/ISO-IMAGES/14.0/FreeBSD-14.0-RELEASE-amd64-disc1.iso|FreeBSD-14.0-RELEASE-amd64-disc1.iso|setup-freebsd.sh"
)

# VM Hardware specs
VM_CPU_COUNT=4
VM_MEMORY_MB=4096
VM_DISK_GB=20

# SSH configuration
SSH_USER="builder"
SSH_PASSWORD="rcommerce"
SSH_PORT=22

# Directories
UTM_DIR="$HOME/Library/Containers/com.utmapp.UTM/Data/Documents"
ISO_CACHE_DIR="$HOME/.cache/rcommerce/iso"

# Get VM info by key
get_vm_info() {
    local key=$1
    local field=$2
    
    local info="${VMS[$key]}"
    IFS='|' read -r TARGET VM_NAME OS_TYPE ARCH DOWNLOAD_URL ISO_FILENAME SETUP_SCRIPT <<< "$info"
    
    case $field in
        target) echo "$TARGET" ;;
        name) echo "$VM_NAME" ;;
        os) echo "$OS_TYPE" ;;
        arch) echo "$ARCH" ;;
        url) echo "$DOWNLOAD_URL" ;;
        iso) echo "$ISO_FILENAME" ;;
        setup) echo "$SETUP_SCRIPT" ;;
        *) echo "" ;;
    esac
}

# List all VM keys
list_vm_keys() {
    echo "${!VMS[@]}"
}

# Check if UTM is installed
check_utm() {
    if [ ! -d "/Applications/UTM.app" ]; then
        echo "ERROR: UTM is not installed"
        echo "Download from: https://mac.getutm.app/"
        return 1
    fi
    return 0
}

# Check if VM exists in UTM
vm_exists() {
    local vm_name=$1
    local utm_vm_path="$UTM_DIR/$vm_name.utm"
    
    if [ -d "$utm_vm_path" ]; then
        return 0
    else
        return 1
    fi
}

# Get VM IP address (requires VM to be running)
get_vm_ip() {
    local vm_name=$1
    
    # Try to get IP from UTM's DHCP leases
    local lease_file="/var/db/dhcpd_leases"
    if [ -f "$lease_file" ]; then
        # Parse DHCP leases to find VM IP
        # This is a simplified approach - may need adjustment
        grep -A 5 "name=\"$vm_name\"" "$lease_file" 2>/dev/null | grep "ip_address" | head -1 | cut -d'=' -f2 | tr -d ';'
    fi
}
