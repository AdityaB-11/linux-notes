#!/bin/bash
# Flatpak build script

set -e

echo "📦 Building Linux Notes Flatpak..."

# Check if flatpak-builder is available
if ! command -v flatpak-builder &> /dev/null; then
    echo "❌ flatpak-builder not found!"
    echo "Install with: sudo apt install flatpak-builder (Ubuntu/Debian)"
    echo "             sudo dnf install flatpak-builder (Fedora)"
    echo "             sudo pacman -S flatpak-builder (Arch)"
    exit 1
fi

# Check if runtime is installed
if ! flatpak list --runtime | grep -q "org.gnome.Platform.*45"; then
    echo "📥 Installing required Flatpak runtime..."
    flatpak install --user flathub org.gnome.Platform//45 org.gnome.Sdk//45 org.freedesktop.Sdk.Extension.rust-stable//23.08
fi

# Generate Cargo sources for offline building
if ! command -v flatpak-cargo-generator.py &> /dev/null; then
    echo "📥 Installing flatpak-cargo-generator..."
    pip3 install --user flatpak-cargo-generator
fi

echo "🔧 Generating Cargo sources..."
flatpak-cargo-generator.py Cargo.lock -o generated-sources.json

echo "🏗️  Building Flatpak..."
flatpak-builder build-dir com.example.LinuxNotes.yml --force-clean

echo "📦 Installing Flatpak locally..."
flatpak-builder --user --install --force-clean build-dir com.example.LinuxNotes.yml

echo "✅ Flatpak build complete!"
echo "🚀 Run with: flatpak run com.example.LinuxNotes"
