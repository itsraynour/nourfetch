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

DOWNLOAD_SUCCESS=0

if [ -f "./target/release/nourfetch" ]; then
    cp "./target/release/nourfetch" "$TARGET"
    chmod +x "$TARGET"
    DOWNLOAD_SUCCESS=1
    echo "Installed local build to $TARGET"
elif [ -n "$ASSET" ]; then
    URL="https://github.com/$REPO/releases/latest/download/$ASSET"
    echo "Downloading nourfetch from $URL..."
    if command -v curl >/dev/null 2>&1; then
        if curl -fsSL "$URL" -o "$TARGET" 2>/dev/null; then
            DOWNLOAD_SUCCESS=1
        fi
    elif command -v wget >/dev/null 2>&1; then
        if wget -qO "$TARGET" "$URL" 2>/dev/null; then
            DOWNLOAD_SUCCESS=1
        fi
    fi
fi

if [ "$DOWNLOAD_SUCCESS" -eq 1 ]; then
    chmod +x "$TARGET"
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
    echo "⚠️  Note: '$INSTALL_DIR' is not in your current PATH."
    
    # Try to automatically add to shell config
    ADDED_TO_RC=0
    if [ -n "$SHELL" ]; then
      case "$SHELL" in
        */zsh)
          SHELL_RC="$HOME/.zshrc"
          ;;
        */bash)
          SHELL_RC="$HOME/.bashrc"
          ;;
        *)
          SHELL_RC="$HOME/.profile"
          ;;
      esac
    else
      SHELL_RC="$HOME/.profile"
    fi

    if [ -f "$SHELL_RC" ] || [ -w "$HOME" ]; then
      if ! grep -q "$INSTALL_DIR" "$SHELL_RC" 2>/dev/null; then
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_RC"
        echo "Added $INSTALL_DIR to $SHELL_RC."
        echo "Please run: source $SHELL_RC (or restart your terminal) to use 'nourfetch'."
        ADDED_TO_RC=1
      fi
    fi

    if [ "$ADDED_TO_RC" -eq 0 ]; then
      echo "To fix this, add the following line to your ~/.bashrc or ~/.zshrc:"
      echo "    export PATH=\"$INSTALL_DIR:\$PATH\""
    fi
    ;;
esac

echo "Run 'nourfetch' in your terminal."

if [ -x "$TARGET" ]; then
    "$TARGET"
fi

