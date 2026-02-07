# Warp Deployment Guide

## Vercel Rust/C Compiler Issue

Warp is a Rust project with dependencies that require C compilation (like `tokio`, `crossterm`, etc.). Vercel's serverless build environment may not have the C compiler (`cc`/`gcc`) pre-installed, causing build failures.

### Solutions

#### Solution 1: GitHub Actions + Self-Hosted Runner (Recommended)
Deploy and test Warp using GitHub Actions for full control over build tools.

```yaml
name: Build and Deploy Warp
on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Install build tools
        run: |
          sudo apt-get update
          sudo apt-get install -y build-essential pkg-config libssl-dev
      - name: Build Warp
        run: cargo build --release
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: warp-release
          path: target/release/
```

#### Solution 2: Docker Build + Registry Push
Build a Docker image and push to a container registry.

```dockerfile
FROM rust:latest as builder
WORKDIR /warp
COPY . .
RUN apt-get update && apt-get install -y build-essential pkg-config libssl-dev
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /warp/target/release/* /usr/local/bin/
ENTRYPOINT ["warp"]
```

Then push to Docker Hub or AWS ECR:
```bash
docker build -t yourorg/warp:latest .
docker push yourorg/warp:latest
```

#### Solution 3: AWS Lambda + Custom Runtime
Package Warp as an AWS Lambda custom runtime.

```bash
# Create deployment package
mkdir -p lambda-build
cd lambda-build
cargo build --release --target x86_64-unknown-linux-gnu
mkdir -p target/lambda
cp target/x86_64-unknown-linux-gnu/release/warp target/lambda/bootstrap
cd target/lambda && zip function.zip bootstrap
```

#### Solution 4: Railway or Fly.io Deployment
These platforms have better Rust support with pre-installed build tools.

**Railway:**
```yaml
# railway.toml
[build]
builder = "dockerfile"

[deploy]
numReplicas = 1
```

**Fly.io:**
```toml
# fly.toml
app = "warp-app"

[build]
  builder = "cargo"
  buildpacks = ["paketobuildpacks/cargo-dist"]

[env]
  BUILDPACK_WORKSPACE_PATH = "."
```

#### Solution 5: Local Build + Manual Upload
Build locally and upload pre-compiled binaries.

```bash
# Local build
cargo build --release

# Create release archive
tar -czf warp-release.tar.gz target/release/

# Upload to your hosting
scp warp-release.tar.gz user@your-server:/opt/
```

### Why Vercel Struggles with Warp

- **Rust Compiler Chain**: Needs `rustc`, `cargo`, and linking tools
- **C Dependencies**: Libraries like `tokio`, `openssl`, `libc` require C compilation
- **Limited Build Environment**: Vercel's serverless builds have limited tools/time
- **Large Compilation**: Rust projects take time to compile

### Recommended Deployment Strategy

For production Warp deployments:

1. **Development**: Build locally with full toolchain
2. **CI/CD**: Use GitHub Actions for automated builds
3. **Hosting**: Deploy to Docker, AWS, Railway, or Fly.io
4. **Distribution**: Publish binaries to GitHub Releases

### Local Development Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install build tools (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev

# Clone and build
git clone https://github.com/neopilotai/Warp.git
cd Warp
git submodule update --init --recursive
cargo build --release

# Run example tools
cargo run --example task_manager --release
cargo run --example build_monitor --release
cargo run --example config_manager --release
```

### Troubleshooting

**Error: `cc` not found**
```bash
# Install build essentials
sudo apt-get install build-essential
```

**Error: `pkg-config` not found**
```bash
sudo apt-get install pkg-config
```

**Error: OpenSSL library not found**
```bash
sudo apt-get install libssl-dev
```

**Error: `git: submodule` not working**
```bash
git config --global credential.helper store
git submodule update --init --recursive
```

### What's Coming

Future versions of Warp will:
- Offer pre-built binaries for Linux, macOS, and Windows
- Support WebAssembly for browser-based tools
- Provide easier Docker/container integration
- Optimize for faster compilation on limited hardware

For now, recommend GitHub Actions, Docker, or Railway for CI/CD deployments.
