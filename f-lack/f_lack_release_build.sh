#!/bin/bash

# Function to install dependencies
install_dependencies() {
    echo "Installing required packages..."
    # Try to update, but don't fail if some repos are unavailable
    sudo apt-get update || true

    # Install required packages, continue if some repos fail
    sudo apt-get install -y \
        build-essential \
        mingw-w64 \
        gcc-multilib \
        gcc-mingw-w64-x86-64 || {
        echo "Warning: Some packages might not have installed correctly"
        echo "Continuing anyway..."
    }
}

# Function to install Rust targets
install_targets() {
    echo "Installing Rust targets..."
    rustup target add x86_64-unknown-linux-gnu || {
        echo "Failed to add linux target, but it might already be installed"
    }
    rustup target add x86_64-pc-windows-gnu || {
        echo "Failed to add windows target. This is required for Windows builds"
        exit 1
    }
}

# Create release directory
RELEASE_DIR="releases"
mkdir -p $RELEASE_DIR

echo "Setting up cross-compilation environment..."
install_dependencies
install_targets

# Build for Linux (native)
echo "Building for Linux..."
cargo build --release || {
    echo "Linux build failed!"
    exit 1
}
cp target/release/f_lack $RELEASE_DIR/f_lack_linux
chmod +x $RELEASE_DIR/f_lack_linux

# Build for Windows
echo "Building for Windows..."
RUSTFLAGS="-C linker=x86_64-w64-mingw32-gcc" \
    cargo build --release --target x86_64-pc-windows-gnu || {
    echo "Windows build failed!"
    exit 1
}
cp target/x86_64-pc-windows-gnu/release/f_lack.exe $RELEASE_DIR/f_lack_windows.exe

echo "Note: macOS build was skipped as it requires building on macOS"
echo ""
echo "Build complete! Binaries are in the $RELEASE_DIR directory:"
ls -lh $RELEASE_DIR
