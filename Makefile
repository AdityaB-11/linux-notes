.PHONY: setup build run clean flatpak install-flatpak

# Setup development environment
setup:
	@./scripts/setup.sh

# Build the application
build:
	@./scripts/build.sh

# Run the application in development mode
run:
	@cargo run

# Clean build artifacts
clean:
	@cargo clean
	@rm -rf build-dir .flatpak-builder generated-sources.json

# Build Flatpak
flatpak:
	@./scripts/build-flatpak.sh

# Install Flatpak locally
install-flatpak: flatpak
	@echo "✅ Flatpak installed! Run with: flatpak run com.example.LinuxNotes"

# Check code
check:
	@cargo check

# Format code
fmt:
	@cargo fmt

# Run tests
test:
	@cargo test

# Show help
help:
	@echo "Available targets:"
	@echo "  setup         - Setup development environment"
	@echo "  build         - Build the application"
	@echo "  run           - Run the application"
	@echo "  check         - Check code without building"
	@echo "  fmt           - Format code"
	@echo "  test          - Run tests"
	@echo "  clean         - Clean build artifacts"
	@echo "  flatpak       - Build Flatpak package"
	@echo "  install-flatpak - Build and install Flatpak locally"
	@echo "  help          - Show this help"
