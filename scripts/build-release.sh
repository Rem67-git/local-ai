#!/bin/bash
# Build release binary and package for offline distribution

set -e

echo "🔨 Building Local AI release..."

# Build core Rust
cd core
cargo build --release --all --locked

# Build Tauri app
cd ../apps/desktop
npm install
npm run build
npm run tauri build

# Package artifacts
echo "📦 Packaging release..."
mkdir -p dist
cp core/target/release/local-ai dist/
cp -r apps/desktop/src-tauri/target/release/bundle dist/

echo "✅ Release built: dist/"
echo "   - Binary: dist/local-ai"
echo "   - Desktop app: dist/bundle/"

# Verify offline capability
echo "🔍 Verifying offline capability..."
./local-ai doctor
./local-ai offline-test

echo "✅ Offline packaging complete!"
