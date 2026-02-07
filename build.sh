#!/bin/bash
# This script handles building Warp with git submodules properly

set -e

# Install build tools required for compiling Rust dependencies
echo "Installing build tools..."
apt-get update -qq
apt-get install -y -qq build-essential pkg-config libssl-dev 2>/dev/null || echo "Build tools already present or error installing"

# Initialize and update git submodules
echo "Initializing git submodules..."
git submodule update --init --recursive || true

# Build with Cargo
echo "Building Warp with Cargo..."
cargo build --release

echo "Build complete!"
