#!/bin/bash
set -e

echo "🔨 Building Squad..."

# Build the project in release mode
cargo build --release

echo "✅ Build complete!"
echo ""
echo "Binary location: target/release/squad"
echo ""
echo "To install locally:"
echo "  cargo install --path ."
echo ""
echo "Or copy the binary:"
echo "  cp target/release/squad /usr/local/bin/"