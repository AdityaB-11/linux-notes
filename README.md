# Linux Notes

A native notes and to-do app for Linux built with Rust, GTK 4, and distributed as a Flatpak.

## Features

- 📝 Create and edit notes with a clean interface
- ✅ Manage to-do lists with checkable items
- 🎨 Modern GTK 4 interface with libadwaita styling
- 💾 Automatic data persistence
- 🐧 Native Linux integration
- 📦 Distributed as Flatpak for easy installation

## Building from Source

### Prerequisites

- Rust (latest stable)
- GTK 4 development libraries
- libadwaita development libraries
- Flatpak Builder (for Flatpak build)

### Development Build

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt install libgtk-4-dev libadwaita-1-dev

# Clone and build
git clone <repository-url>
cd linux-notes
cargo build --release
```

### Running

```bash
cargo run
```

### Building Flatpak

First, generate the Cargo sources for offline building:

```bash
# Install flatpak-cargo-generator
pip3 install flatpak-cargo-generator

# Generate sources
flatpak-cargo-generator.py Cargo.lock -o generated-sources.json

# Build Flatpak
flatpak-builder build-dir com.example.LinuxNotes.yml --force-clean

# Install locally
flatpak-builder --user --install --force-clean build-dir com.example.LinuxNotes.yml
```

## Architecture

The application is structured with:

- `main.rs` - Main application entry point and UI setup
- `data.rs` - Data models and persistence logic
- `ui.rs` - User interface components and widgets

Data is automatically saved to `~/.local/share/linux-notes/data.json`.

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
