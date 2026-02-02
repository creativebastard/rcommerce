#!/bin/bash
# VM configuration for UTM-based builds
# Compatible with Bash 3.2 (macOS default)

# VM Definitions - using functions instead of associative arrays
# Format: TARGET|VM_NAME|OS_TYPE|ARCH|DOWNLOAD_URL|ISO_FILENAME|SETUP_SCRIPT

get_vm_target() {
    case "$1" in
        "ubuntu-x64") echo "x86_64-unknown-linux-gnu" ;;
        "alpine-x64") echo "x86_64-unknown-linux-musl" ;;
        "ubuntu-arm") echo "aarch64-unknown-linux-gnu" ;;
        "freebsd-x64") echo "x86_64-unknown-freebsd" ;;
        *) echo "" ;;
    esac
}

get_vm_name() {
    case "$1" in
        "ubuntu-x64") echo "rcommerce-ubuntu-x64" ;;
        "alpine-x64") echo "rcommerce-alpine-x64" ;;
        "ubuntu-arm") echo "rcommerce-ubuntu-arm" ;;
        "freebsd-x64") echo "rcommerce-freebsd-x64" ;;
        *) echo "" ;;
    esac
}

get_vm_os() {
    case "$1" in
        "ubuntu-x64"|"alpine-x64"|"ubuntu-arm") echo "linux" ;;
        "freebsd-x64") echo "freebsd" ;;
        *) echo "" ;;
    esac
}

get_vm_arch() {
    case "$1" in
        "ubuntu-x64"|"alpine-x64"|"freebsd-x64") echo "x86_64" ;;
        "ubuntu-arm") echo "arm64" ;;
        *) echo "" ;;
    esac
}

get_vm_url() {
    case "$1" in
        "ubuntu-x64") echo "https://releases.ubuntu.com/22.04/ubuntu-22.04.3-live-server-amd64.iso" ;;
        "alpine-x64") echo "https://dl-cdn.alpinelinux.org/alpine/v3.18/releases/x86_64/alpine-standard-3.18.4-x86_64.iso" ;;
        "ubuntu-arm") echo "https://cdimage.ubuntu.com/ubuntu-server/jammy/daily-live/current/jammy-live-server-arm64.iso" ;;
        "freebsd-x64") echo "https://download.freebsd.org/releases/ISO-IMAGES/14.3/FreeBSD-14.3-RELEASE-amd64-disc1.iso" ;;
        *) echo "" ;;
    esac
}

get_vm_iso() {
    case "$1" in
        "ubuntu-x64") echo "ubuntu-22.04.3-live-server-amd64.iso" ;;
        "alpine-x64") echo "alpine-standard-3.18.4-x86_64.iso" ;;
        "ubuntu-arm") echo "jammy-live-server-arm64.iso" ;;
        "freebsd-x64") echo "FreeBSD-14.3-RELEASE-amd64-disc1.iso" ;;
        *) echo "" ;;
    esac
}

get_vm_setup() {
    case "$1" in
        "ubuntu-x64"|"ubuntu-arm") echo "setup-debian.sh" ;;
        "alpine-x64") echo "setup-alpine.sh" ;;
        "freebsd-x64") echo "setup-freebsd.sh" ;;
        *) echo "" ;;
    esac
}

get_vm_info() {
    local key=$1
    local field=$2
    
    case $field in
        target) get_vm_target "$key" ;;
        name) get_vm_name "$key" ;;
        os) get_vm_os "$key" ;;
        arch) get_vm_arch "$key" ;;
        url) get_vm_url "$key" ;;
        iso) get_vm_iso "$key" ;;
        setup) get_vm_setup "$key" ;;
        *) echo "" ;;
    esac
}

# List all VM keys
list_vm_keys() {
    echo "ubuntu-x64 alpine-x64 ubuntu-arm freebsd-x64"
}

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
