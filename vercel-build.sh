#!/bin/bash
# Vercel build hook for Rust projects

set -e

# Pre-build setup - ensure we have minimal tools needed
echo "[Vercel Build] Setting up Rust build environment..."

# Check if cc is available, if not try to work around it
if ! command -v cc &> /dev/null; then
    echo "[Vercel Build] C compiler not found - attempting workaround..."
    export CC=gcc
    export CXX=g++
fi

# Alternative: Use vendored sources if possible
export CARGO_NET_OFFLINE=false

# Disable building C dependencies if possible
export CARGO_CFG_TARGET_OS=linux
export CARGO_CFG_TARGET_ARCH=x86_64

echo "[Vercel Build] Installing build essentials..."
apt-get update -y >/dev/null 2>&1 || true
apt-get install -y build-essential pkg-config libssl-dev >/dev/null 2>&1 || {
    echo "[Vercel Build] Note: Could not install build tools via apt-get"
    echo "[Vercel Build] Attempting to proceed with system tools..."
}

# Update submodules
echo "[Vercel Build] Initializing git submodules..."
git config --global credential.helper store || true
git submodule update --init --recursive 2>/dev/null || true

# Build
echo "[Vercel Build] Building Warp..."
cargo build --release 2>&1 || {
    echo "[Vercel Build] Standard build failed, trying alternative approach..."
    cargo build --release --target x86_64-unknown-linux-gnu || exit 1
}

echo "[Vercel Build] Build completed successfully!"
