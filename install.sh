#!/bin/sh
# nourfetch - Installer for Linux and macOS
# Usage: curl -fsSL https://raw.githubusercontent.com/itsraynour/nourfetch/main/install.sh | sh

set -e

REPO="itsraynour/nourfetch"
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux*)
    case "$ARCH" in
      x86_64) ASSET="nourfetch-linux-x86_64" ;;
      *) echo "Unsupported Linux architecture: $ARCH. Attempting build from source..."; ASSET="" ;;
    esac
    ;;
  darwin*)
    case "$ARCH" in
      x86_64) ASSET="nourfetch-macos-x86_64" ;;
      arm64|aarch64) ASSET="nourfetch-macos-arm64" ;;
      *) echo "Unsupported macOS architecture: $ARCH"; ASSET="" ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

TARGET="$INSTALL_DIR/nourfetch"

if [ -f "./target/release/nourfetch" ]; then
    cp "./target/release/nourfetch" "$TARGET"
    chmod +x "$TARGET"
    echo "Installed local build to $TARGET"
elif [ -n "$ASSET" ]; then
    URL="https://github.com/$REPO/releases/latest/download/$ASSET"
    echo "Downloading nourfetch from $URL..."
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$URL" -o "$TARGET"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO "$TARGET" "$URL"
    else
        echo "Error: curl or wget is required."
        exit 1
    fi
    chmod +x "$TARGET"
    echo "Installed nourfetch to $TARGET"
else
    if command -v cargo >/dev/null 2>&1; then
        echo "Building from source using cargo..."
        cargo install --git "https://github.com/$REPO"
    else
        echo "Error: Could not install binary. Please install Rust to build from source."
        exit 1
    fi
fi

echo "nourfetch installed successfully."
echo "Run 'nourfetch' in your terminal."

if [ -x "$TARGET" ]; then
    "$TARGET"
fi
