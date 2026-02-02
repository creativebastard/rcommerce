#!/bin/bash
# Build R Commerce for all configured UTM VMs

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/vm-config.sh"

VERSION=${1:-0.1.0}

echo "========================================"
echo "R Commerce Release Build v$VERSION"
echo "Using UTM VMs"
echo "========================================"
echo ""

# Check UTM is installed
if ! check_utm; then
    exit 1
fi

# Track results
SUCCESS=()
FAILED=()

# Build for each configured VM
for VM_KEY in $(list_vm_keys); do
    VM_NAME=$(get_vm_info "$VM_KEY" name)
    TARGET=$(get_vm_info "$VM_KEY" target)
    
    # Skip if VM doesn't exist
    if ! vm_exists "$VM_NAME"; then
        echo "Skipping $VM_KEY - VM not created yet"
        echo "Create with: ./create-vm.sh $VM_KEY"
        echo ""
        continue
    fi
    
    echo "========================================"
    echo "Building: $TARGET"
    echo "VM: $VM_NAME"
    echo "========================================"
    
    if "$SCRIPT_DIR/build-target.sh" "$VM_KEY" "$VERSION"; then
        SUCCESS+=("$TARGET")
        echo "✓ Build successful for $TARGET"
    else
        FAILED+=("$TARGET")
        echo "✗ Build failed for $TARGET"
    fi
    echo ""
done

# Summary
echo "========================================"
echo "Build Summary"
echo "========================================"

echo "Successful: ${#SUCCESS[@]}"
for t in "${SUCCESS[@]}"; do
    echo "  ✓ $t"
done

if [ ${#FAILED[@]} -gt 0 ]; then
    echo ""
    echo "Failed: ${#FAILED[@]}"
    for t in "${FAILED[@]}"; do
        echo "  ✗ $t"
    done
fi

# Show artifacts
RELEASE_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)/release"
echo ""
echo "Release artifacts in: $RELEASE_DIR/"
if [ -f "$RELEASE_DIR/SHA256SUMS.txt" ]; then
    echo ""
    echo "SHA256 Checksums:"
    cat "$RELEASE_DIR/SHA256SUMS.txt"
fi

if [ ${#FAILED[@]} -gt 0 ]; then
    exit 1
fi

echo ""
echo "All builds completed successfully!"
