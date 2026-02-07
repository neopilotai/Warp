#!/bin/bash
# This script handles building Warp with git submodules properly

set -e

# Initialize and update git submodules
echo "Initializing git submodules..."
git submodule update --init --recursive || true

# Build with Cargo
echo "Building Warp with Cargo..."
cargo build --release

echo "Build complete!"
