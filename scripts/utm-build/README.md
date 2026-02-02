# UTM-Based Release Build System

This directory contains scripts for automating R Commerce release builds using UTM VMs on macOS.

## Prerequisites

- macOS with Apple Silicon or Intel
- [UTM](https://mac.getutm.app/) installed
- QEMU (installed by UTM)
- SSH key pair (`~/.ssh/id_rsa` and `~/.ssh/id_rsa.pub`)

## Supported Build Targets

| Target | VM Name | Base Image |
|--------|---------|------------|
| x86_64-unknown-linux-gnu | rcommerce-ubuntu-x64 | Ubuntu Server 22.04 LTS (x86_64) |
| x86_64-unknown-linux-musl | rcommerce-alpine-x64 | Alpine Linux (x86_64) |
| aarch64-unknown-linux-gnu | rcommerce-ubuntu-arm | Ubuntu Server 22.04 LTS (ARM64) |
| x86_64-unknown-freebsd | rcommerce-freebsd-x64 | FreeBSD 14.0 (x86_64) |

## Quick Start

### 1. Create all VMs
```bash
./scripts/utm-build/create-all-vms.sh
```

### 2. Start VMs and wait for them to boot
```bash
./scripts/utm-build/start-all-vms.sh
```

### 3. Build releases
```bash
./scripts/utm-build/build-all.sh 0.1.0
```

### 4. Stop VMs when done
```bash
./scripts/utm-build/stop-all-vms.sh
```

## Individual VM Management

```bash
# Create a specific VM
./scripts/utm-build/create-vm.sh ubuntu-x64

# Start a VM
./scripts/utm-build/start-vm.sh rcommerce-ubuntu-x64

# Build for specific target
./scripts/utm-build/build-target.sh x86_64-unknown-linux-gnu 0.1.0

# Stop a VM
./scripts/utm-build/stop-vm.sh rcommerce-ubuntu-x64

# Delete a VM
./scripts/utm-build/delete-vm.sh rcommerce-ubuntu-x64
```

## VM Specifications

Each VM is configured with:
- 4 CPU cores
- 4 GB RAM
- 20 GB storage
- Shared network with host
- SSH access on port 22

## SSH Configuration

The VMs are configured with your public key at `~/.ssh/id_rsa.pub`. To access:

```bash
ssh builder@<vm-ip>
```

Default user: `builder`
Default password (for initial setup): `rcommerce`

## Troubleshooting

### VM won't start
Check UTM permissions in System Preferences > Security & Privacy

### Can't connect via SSH
1. Ensure VM has fully booted
2. Check VM IP address in UTM console
3. Verify SSH key is in `~/.ssh/id_rsa.pub`

### Build fails
Check the VM has enough disk space:
```bash
ssh builder@<vm-ip> df -h
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         macOS Host                          │
│  ┌──────────────────────────────────────────────────────┐  │
│  │          UTM Virtual Machines                        │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌─────────┐ │  │
│  │  │ Ubuntu   │ │ Alpine   │ │ Ubuntu   │ │ FreeBSD │ │  │
│  │  │ x86_64   │ │ x86_64   │ │ ARM64    │ │ x86_64  │ │  │
│  │  └──────────┘ └──────────┘ └──────────┘ └─────────┘ │  │
│  └──────────────────────────────────────────────────────┘  │
│                            │                                │
│                    SSH + rsync                             │
│                            │                                │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           Build Scripts (scripts/utm-build)          │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```
