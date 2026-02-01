# VM-Based Release Build System

This directory contains scripts for building R Commerce release binaries using virtual machines.

## Supported Targets

| Target | VM Type | Base OS |
|--------|---------|---------|
| x86_64-unknown-linux-gnu | QEMU/UTM | Ubuntu/Debian |
| x86_64-unknown-linux-musl | QEMU/UTM | Alpine Linux |
| aarch64-unknown-linux-gnu | QEMU/UTM | Ubuntu/Debian ARM64 |
| x86_64-unknown-freebsd | QEMU/Bhyve | FreeBSD 14 |

## Prerequisites

### macOS
- QEMU (via Homebrew: `brew install qemu`)
- UTM (GUI for QEMU) - optional but recommended
- SSH key pair for VM access

### VM Setup
1. Create a VM for each target platform
2. Install Rust and required dependencies
3. Configure SSH access with key authentication
4. Install required packages (see below)

## VM Configuration

### Linux (Ubuntu/Debian)
```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y build-essential libssl-dev pkg-config curl

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add targets
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-gnu
```

### FreeBSD
```bash
# Install dependencies
pkg install -y rust curl openssl

# Add target
rustup target add x86_64-unknown-freebsd
```

## Usage

```bash
# Build all targets
./scripts/vm-build/build-all.sh 0.1.0

# Build specific target
./scripts/vm-build/build-target.sh x86_64-unknown-linux-gnu 0.1.0
```

## VM Inventory

Edit `vms.conf` to configure your VMs:

```
# Format: TARGET,VM_NAME,IP_ADDRESS,SSH_PORT,SSH_USER
x86_64-unknown-linux-gnu,ubuntu-build,192.168.64.2,22,builder
x86_64-unknown-freebsd,freebsd-build,192.168.64.3,22,builder
```
