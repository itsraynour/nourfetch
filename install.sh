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

# Determine install location
INSTALL_DIR="/usr/local/bin"
USE_SUDO=0

if [ -w "$INSTALL_DIR" ]; then
    USE_SUDO=0
elif command -v sudo >/dev/null 2>&1 && (sudo -n true 2>/dev/null || [ -t 0 ] || [ -c /dev/tty ]); then
    # Try using sudo if user has permissions or interactive terminal
    if sudo -v 2>/dev/null; then
        USE_SUDO=1
    else
        INSTALL_DIR="$HOME/.local/bin"
        USE_SUDO=0
        mkdir -p "$INSTALL_DIR"
    fi
else
    INSTALL_DIR="$HOME/.local/bin"
    USE_SUDO=0
    mkdir -p "$INSTALL_DIR"
fi

TARGET="$INSTALL_DIR/nourfetch"
TMP_BIN="/tmp/nourfetch_$$"

cleanup() {
    rm -f "$TMP_BIN"
}
trap cleanup EXIT INT TERM

DOWNLOAD_SUCCESS=0

if [ -f "./target/release/nourfetch" ]; then
    cp "./target/release/nourfetch" "$TMP_BIN"
    chmod +x "$TMP_BIN"
    DOWNLOAD_SUCCESS=1
elif [ -n "$ASSET" ]; then
    URL="https://github.com/$REPO/releases/latest/download/$ASSET"
    echo "Downloading nourfetch from $URL..."
    if command -v curl >/dev/null 2>&1; then
        if curl -fsSL "$URL" -o "$TMP_BIN" 2>/dev/null; then
            DOWNLOAD_SUCCESS=1
        fi
    elif command -v wget >/dev/null 2>&1; then
        if wget -qO "$TMP_BIN" "$URL" 2>/dev/null; then
            DOWNLOAD_SUCCESS=1
        fi
    fi
fi

if [ "$DOWNLOAD_SUCCESS" -eq 1 ]; then
    chmod +x "$TMP_BIN"
    if [ "$USE_SUDO" -eq 1 ]; then
        sudo cp "$TMP_BIN" "$TARGET"
        sudo chmod +x "$TARGET"
    else
        cp "$TMP_BIN" "$TARGET"
        chmod +x "$TARGET"
    fi
    echo "Installed nourfetch to $TARGET"
else
    echo "Binary asset not found in release. Building from source via Cargo..."
    if command -v cargo >/dev/null 2>&1; then
        cargo install --git "https://github.com/$REPO.git" --root "$(dirname "$(dirname "$TARGET")")" || cargo install --git "https://github.com/$REPO.git"
    else
        echo "Error: Pre-built binary not found and 'cargo' is not installed."
        echo "Please install Rust (https://rustup.rs) or download the binary from:"
        echo "https://github.com/$REPO/releases"
        exit 1
    fi
fi

echo "nourfetch installed successfully."

# Check if INSTALL_DIR is in PATH
case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    echo ""
    echo "Note: '$INSTALL_DIR' is not in your current PATH."
    
    for RC in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
      if [ -f "$RC" ] && [ -w "$RC" ]; then
        if ! grep -q "$INSTALL_DIR" "$RC" 2>/dev/null; then
          echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$RC"
        fi
      fi
    done

    echo "To use 'nourfetch' in this terminal, run:"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
    echo "or:"
    echo "  source ~/.bashrc"
    echo ""
    ;;
esac

echo "Run 'nourfetch' in your terminal."
echo ""

if [ -x "$TARGET" ]; then
    "$TARGET"
fi


