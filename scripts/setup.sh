#!/bin/bash
# Setup development environment for Linux Notes

set -e

echo "🛠️  Setting up Linux Notes development environment..."

# Function to install packages based on distro
install_packages() {
    if command -v apt &> /dev/null; then
        # Ubuntu/Debian
        echo "📦 Installing packages for Ubuntu/Debian..."
        sudo apt update
        sudo apt install -y curl build-essential pkg-config libgtk-4-dev libadwaita-1-dev
        
        # Install Rust if not present
        if ! command -v cargo &> /dev/null; then
            echo "🦀 Installing Rust..."
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source ~/.cargo/env
        fi
        
    elif command -v dnf &> /dev/null; then
        # Fedora
        echo "📦 Installing packages for Fedora..."
        sudo dnf install -y curl gcc pkg-config gtk4-devel libadwaita-devel
        
        if ! command -v cargo &> /dev/null; then
            echo "🦀 Installing Rust..."
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source ~/.cargo/env
        fi
        
    elif command -v pacman &> /dev/null; then
        # Arch Linux
        echo "📦 Installing packages for Arch Linux..."
        sudo pacman -S --needed curl base-devel pkg-config gtk4 libadwaita
        
        if ! command -v cargo &> /dev/null; then
            echo "🦀 Installing Rust..."
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source ~/.cargo/env
        fi
        
    else
        echo "❌ Unsupported distribution. Please install the following manually:"
        echo "   - Rust (https://rustup.rs/)"
        echo "   - GTK 4 development libraries"
        echo "   - libadwaita development libraries"
        echo "   - pkg-config"
        echo "   - C compiler (gcc/clang)"
        exit 1
    fi
}

# Install packages
install_packages

# Verify installations
echo "🔍 Verifying installations..."

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found in PATH. Please run: source ~/.cargo/env"
    exit 1
fi

if ! pkg-config --exists gtk4; then
    echo "❌ GTK 4 development libraries not found!"
    exit 1
fi

if ! pkg-config --exists libadwaita-1; then
    echo "❌ libadwaita development libraries not found!"
    exit 1
fi

echo "✅ Development environment setup complete!"
echo "🚀 You can now build the project with: ./scripts/build.sh"
echo "🏃 Or run directly with: cargo run"
