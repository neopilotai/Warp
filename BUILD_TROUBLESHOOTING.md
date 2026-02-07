# Build Troubleshooting Guide

## Common Build Errors and Solutions

### Error: `linker 'cc' not found`

**Cause**: The C compiler is not available in the build environment. This happens when Rust dependencies with C components (like `libc`, `proc-macro2`, `quote`) try to compile their build scripts.

**Affected Dependencies**:
- `libc` - C standard library bindings
- `proc-macro2` - Procedural macro support
- `quote` - Macro quoting utilities
- `serde_core` - Serialization core

**Solution 1: Vercel Deployment** (RECOMMENDED)
We've already configured this in `vercel.json`. The installation command now installs:
```bash
apt-get install -y build-essential pkg-config libssl-dev
```

This installs:
- `build-essential` - GCC compiler, Make, and other essentials
- `pkg-config` - Package configuration utility
- `libssl-dev` - OpenSSL development headers

**Solution 2: Local Development**
If you're building locally and see this error:

**macOS**:
```bash
xcode-select --install
```

**Linux (Ubuntu/Debian)**:
```bash
sudo apt-get update
sudo apt-get install build-essential pkg-config libssl-dev
```

**Linux (Fedora/RHEL)**:
```bash
sudo dnf install gcc gcc-c++ make pkg-config openssl-devel
```

**Windows**:
Install Visual Studio Build Tools or MinGW with C++ support.

### Error: `Could not find OpenSSL`

**Cause**: OpenSSL development headers are missing.

**Solution**:
```bash
# macOS
brew install openssl

# Linux
sudo apt-get install libssl-dev

# Then set PKG_CONFIG_PATH
export PKG_CONFIG_PATH="/usr/local/opt/openssl/lib/pkgconfig"
```

### Build Hanging or Timing Out

**Cause**: Cargo compilation can take time, especially on first build with full optimization.

**Solution**:
- First build with debug symbols (faster): `cargo build`
- Subsequent release builds will be cached
- Vercel has a 30-minute build timeout, which should be sufficient for Warp

### Git Submodule Errors

**Error**: `fatal: No such file or directory`

**Cause**: Git submodules (themes, keysets, workflows) weren't initialized.

**Solution**:
```bash
git submodule update --init --recursive
```

This is now handled automatically in `build.sh`.

### Out of Disk Space During Build

**Cause**: Cargo can use significant disk space during compilation.

**Solution**:
1. Clean previous builds: `cargo clean`
2. Try building in release mode with optimizations:
   ```bash
   CARGO_PROFILE_RELEASE_OPT_LEVEL=3 cargo build --release
   ```
3. Vercel instances have sufficient disk space, but monitor build logs

## Build Environment Variables

The following environment variables are set in `vercel.json`:

```json
"env": {
  "CARGO_PROFILE_RELEASE_OPT_LEVEL": "3"
}
```

This enables full compiler optimizations for smaller, faster binaries.

## Local Build Instructions

### Prerequisites
- Rust 1.70+ (install from https://rustup.rs/)
- C compiler (GCC, Clang, or MSVC)
- OpenSSL development headers
- Git with submodule support

### Build Steps

```bash
# Clone repository
git clone https://github.com/neopilotai/Warp.git
cd Warp

# Initialize submodules
git submodule update --init --recursive

# Install build dependencies (Linux/macOS)
# macOS
brew install openssl pkg-config

# Ubuntu/Debian
sudo apt-get install build-essential pkg-config libssl-dev

# Build the project
cargo build --release

# Run examples
cargo run --example task_manager --release
cargo run --example build_monitor --release
cargo run --example config_manager --release

# Run tests
cargo test --release
```

### Build Artifacts

After a successful build:
- **Release binary**: `target/release/warp`
- **Library**: `target/release/libwarp_terminal_apps.{a,so,dylib}`
- **Examples**: Built into `target/release/examples/`

## Performance Tips

1. **Use sccache for faster rebuilds**:
   ```bash
   cargo install sccache
   export RUSTC_WRAPPER=sccache
   cargo build --release
   ```

2. **Use mold for faster linking** (Linux):
   ```bash
   cargo install cargo-binstall
   cargo binstall mold
   export RUSTFLAGS="-C link-arg=-fuse-ld=mold"
   cargo build --release
   ```

3. **Parallel compilation**:
   ```bash
   CARGO_BUILD_JOBS=8 cargo build --release
   ```

## Debugging Build Issues

Enable verbose output:
```bash
# Verbose compilation
cargo build -vv

# Show all commands
CARGO_VERBOSE=true cargo build --release

# Keep temp files for inspection
TMPDIR=/tmp/cargo-debug cargo build -vv
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build and Test

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          submodules: recursive
      
      - uses: dtolnay/rust-toolchain@stable
      
      - uses: Swatinem/rust-cache@v2
      
      - name: Install dependencies
        run: sudo apt-get install -y build-essential pkg-config libssl-dev
      
      - name: Build
        run: cargo build --release
      
      - name: Test
        run: cargo test --release
```

## Getting Help

If you encounter a build issue not listed here:

1. Check the full error message for specific file/line information
2. Run with verbose flags: `cargo build -vv`
3. Try a clean build: `cargo clean && cargo build`
4. Check system dependencies are installed
5. Open an issue on GitHub with:
   - Your OS and version
   - Rust version (`rustc --version`)
   - Full error output (with `-vv` flag)
   - Steps to reproduce
