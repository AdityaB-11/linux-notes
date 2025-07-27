#!/bin/bash
# Development build script

set -e

echo "🔧 Building Linux Notes (Development)..."

# Check if GTK4 and libadwaita are installed
if ! pkg-config --exists gtk4; then
    echo "❌ GTK 4 development libraries not found!"
    echo "Install with: sudo apt install libgtk-4-dev (Ubuntu/Debian)"
    echo "             sudo dnf install gtk4-devel (Fedora)"
    echo "             sudo pacman -S gtk4 (Arch)"
    exit 1
fi

if ! pkg-config --exists libadwaita-1; then
    echo "❌ libadwaita development libraries not found!"
    echo "Install with: sudo apt install libadwaita-1-dev (Ubuntu/Debian)"
    echo "             sudo dnf install libadwaita-devel (Fedora)"
    echo "             sudo pacman -S libadwaita (Arch)"
    exit 1
fi

# Build the application
echo "🦀 Building with Cargo..."
cargo build --release

echo "✅ Build complete! Run with: cargo run"
