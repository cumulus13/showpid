#!/bin/bash
set -e

echo "Building showpid for all platforms..."

# Ensure we have the targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin

# Install Linux dependencies
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    sudo apt-get update
    sudo apt-get install -y libx11-dev mingw-w64
fi

# Build for each platform
echo "Building for Linux..."
cargo build --release --target x86_64-unknown-linux-gnu

echo "Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu

echo "Building for macOS... (may not work from Linux)"
cargo build --release --target x86_64-apple-darwin || echo "macOS build failed (expected on Linux)"

echo "Build complete! Check target/ directory for binaries."