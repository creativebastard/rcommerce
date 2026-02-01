#!/bin/bash
# Setup script to run on VMs to prepare them for building R Commerce

set -e

TARGET=$1

if [ -z "$TARGET" ]; then
    echo "Usage: $0 <target>"
    echo ""
    echo "Examples:"
    echo "  $0 x86_64-unknown-linux-gnu"
    echo "  $0 x86_64-unknown-freebsd"
    exit 1
fi

echo "Setting up VM for target: $TARGET"
echo ""

# Detect OS
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
elif [ -f /etc/freebsd-version ]; then
    OS="freebsd"
else
    echo "Cannot detect OS"
    exit 1
fi

echo "Detected OS: $OS"

# Install dependencies based on OS
case $OS in
    ubuntu|debian)
        echo "Installing dependencies for Debian/Ubuntu..."
        sudo apt-get update
        sudo apt-get install -y \
            build-essential \
            libssl-dev \
            pkg-config \
            curl \
            git \
            rsync
        ;;
    
    alpine)
        echo "Installing dependencies for Alpine..."
        sudo apk add --no-cache \
            build-base \
            openssl-dev \
            pkgconfig \
            curl \
            git \
            rsync \
            musl-dev
        ;;
    
    freebsd)
        echo "Installing dependencies for FreeBSD..."
        sudo pkg install -y \
            rust \
            curl \
            openssl \
            git \
            rsync
        ;;
    
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

# Install Rust if not already installed
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust already installed: $(rustc --version)"
fi

# Ensure cargo is in PATH for future sessions
if ! grep -q ".cargo/env" ~/.bashrc 2>/dev/null; then
    echo 'source $HOME/.cargo/env' >> ~/.bashrc
fi
if ! grep -q ".cargo/env" ~/.profile 2>/dev/null; then
    echo 'source $HOME/.cargo/env' >> ~/.profile
fi

# Add target
source $HOME/.cargo/env
rustup target add "$TARGET"

# Show installed versions
echo ""
echo "Setup complete!"
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo "Target installed: $TARGET"
