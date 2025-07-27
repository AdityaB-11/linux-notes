# 🐧 Linux Notes - Native Notes & To-Do App

A beautiful, native notes and to-do application for Linux built with **Rust**, **GTK 4**, and **libadwaita**, distributed as a **Flatpak**.

## ✨ Features

- 📝 **Rich Notes**: Create and edit notes with a clean, distraction-free interface
- ✅ **To-Do Lists**: Manage tasks with checkable items and completion tracking
- 🎨 **Modern UI**: Beautiful GTK 4 interface with libadwaita styling
- 💾 **Auto-Save**: Automatic data persistence - never lose your notes
- 🚀 **Fast**: Native performance with Rust
- 📦 **Easy Install**: Distributed as Flatpak for universal Linux compatibility
- 🔒 **Privacy**: All data stored locally, no cloud dependency

## 🚀 Quick Start

### Option 1: One-Command Setup
```bash
# Setup everything automatically
make setup
make run
```

### Option 2: Manual Setup
```bash
# 1. Setup development environment
./scripts/setup.sh

# 2. Build and run
cargo run
```

### Option 3: Flatpak Build
```bash
# Build and install as Flatpak
make install-flatpak

# Run the installed Flatpak
flatpak run com.example.LinuxNotes
```

## 🏗️ Project Structure

```
linux-notes/
├── src/
│   ├── main.rs      # Application entry point and main UI
│   ├── data.rs      # Data models and persistence
│   └── ui.rs        # UI components and widgets
├── data/            # Application metadata
│   ├── com.example.LinuxNotes.desktop
│   ├── com.example.LinuxNotes.metainfo.xml
│   └── com.example.LinuxNotes.svg
├── scripts/         # Build and setup scripts
│   ├── setup.sh     # Development environment setup
│   ├── build.sh     # Development build
│   └── build-flatpak.sh  # Flatpak build
├── Cargo.toml       # Rust dependencies
├── Makefile         # Convenient build targets
└── com.example.LinuxNotes.yml  # Flatpak manifest
```

## 🛠️ Development

### Prerequisites
- Rust (latest stable)
- GTK 4 development libraries
- libadwaita development libraries
- pkg-config

### Available Commands
```bash
make setup          # Setup development environment
make build          # Build the application
make run            # Run in development mode
make check          # Check code without building
make fmt            # Format code
make test           # Run tests
make clean          # Clean build artifacts
make flatpak        # Build Flatpak package
make install-flatpak # Build and install Flatpak
make help           # Show all available commands
```

### Development Workflow
1. **Setup**: Run `make setup` to install all dependencies
2. **Code**: Edit the source files in `src/`
3. **Test**: Run `make run` to test your changes
4. **Package**: Run `make flatpak` to build the Flatpak

## 🏛️ Architecture

### Core Components

#### `main.rs`
- Application initialization
- Main window setup
- Stack switcher for Notes/To-Do views
- Application lifecycle management

#### `data.rs`
- `Note` struct with title, content, timestamps
- `TodoItem` struct with completion tracking
- `AppData` for managing collections
- Automatic JSON persistence to `~/.local/share/linux-notes/`

#### `ui.rs`
- `NotesView`: Notes list and editor interface
- `TodoView`: To-do list management interface
- Event handling and UI updates

### Data Flow
1. **Load**: App starts → Load data from JSON file
2. **Edit**: User makes changes → Update in-memory data
3. **Save**: Changes made → Auto-save to disk
4. **Sync**: UI automatically reflects data changes

## 🎨 UI Design

The app follows GNOME HIG (Human Interface Guidelines) with:
- **Header Bar**: Clean title bar with stack switcher
- **Two-Panel Layout**: List view + detail view for notes
- **Adaptive Layout**: Responsive design for different screen sizes
- **libadwaita Styling**: Modern, consistent appearance

## 📦 Flatpak Distribution

The app is packaged as a Flatpak for:
- **Universal compatibility** across Linux distributions
- **Sandboxed security** with limited filesystem access
- **Easy installation** via software centers
- **Automatic updates** through Flatpak repositories

### Flatpak Permissions
- `--share=ipc`: Inter-process communication
- `--socket=wayland`: Wayland display server
- `--socket=fallback-x11`: X11 fallback support
- `--device=dri`: Hardware acceleration
- `--filesystem=xdg-data/linux-notes:create`: Data storage access

## 🔧 Customization

### Changing App Identity
To customize the app for your own use:

1. **Update IDs**: Change `com.example.LinuxNotes` to your domain
2. **Metadata**: Edit files in `data/` directory
3. **Cargo.toml**: Update package information
4. **Flatpak manifest**: Update the `.yml` file

### Adding Features
The modular architecture makes it easy to add:
- New note formats (markdown, rich text)
- Additional views (calendar, tags)
- Export functionality
- Sync capabilities

## 🤝 Contributing

Contributions are welcome! Areas for improvement:
- [ ] Rich text editing
- [ ] Note categories/tags
- [ ] Search functionality
- [ ] Import/export features
- [ ] Keyboard shortcuts
- [ ] Dark mode toggle
- [ ] Note templates

## 📝 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **GTK Team**: For the excellent GTK 4 toolkit
- **GNOME Project**: For libadwaita and design guidelines
- **Rust Community**: For the amazing ecosystem
- **Flatpak**: For modern Linux app distribution

---

**Happy note-taking! 📝✨**
