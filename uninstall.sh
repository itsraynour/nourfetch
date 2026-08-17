#!/bin/sh
# nourfetch - Uninstaller for Linux and macOS
# Usage: curl -fsSL https://raw.githubusercontent.com/itsraynour/nourfetch/main/uninstall.sh | sh

set -e

echo "Uninstalling nourfetch..."

REMOVED=0

if [ -f "/usr/local/bin/nourfetch" ]; then
    if [ -w "/usr/local/bin" ] || [ "$(id -u)" -eq 0 ]; then
        rm -f "/usr/local/bin/nourfetch"
        echo "  Removed /usr/local/bin/nourfetch"
        REMOVED=1
    elif command -v sudo >/dev/null 2>&1; then
        sudo rm -f "/usr/local/bin/nourfetch"
        echo "  Removed /usr/local/bin/nourfetch (sudo)"
        REMOVED=1
    else
        echo "  Notice: Please run 'sudo rm /usr/local/bin/nourfetch' to remove system binary."
    fi
fi

if [ -n "$HOME" ] && [ -f "$HOME/.local/bin/nourfetch" ]; then
    rm -f "$HOME/.local/bin/nourfetch"
    echo "  Removed $HOME/.local/bin/nourfetch"
    REMOVED=1
fi

if [ -n "$HOME" ] && [ -f "$HOME/.cargo/bin/nourfetch" ]; then
    rm -f "$HOME/.cargo/bin/nourfetch"
    echo "  Removed $HOME/.cargo/bin/nourfetch"
    REMOVED=1
fi

if [ -n "$XDG_CONFIG_HOME" ] && [ -d "$XDG_CONFIG_HOME/nourfetch" ]; then
    rm -rf "$XDG_CONFIG_HOME/nourfetch"
    echo "  Removed directory: $XDG_CONFIG_HOME/nourfetch"
    REMOVED=1
fi

if [ -n "$HOME" ] && [ -d "$HOME/.config/nourfetch" ]; then
    rm -rf "$HOME/.config/nourfetch"
    echo "  Removed directory: $HOME/.config/nourfetch"
    REMOVED=1
fi

echo ""
if [ "$REMOVED" -eq 1 ]; then
    echo "nourfetch has been completely uninstalled."
else
    echo "No installed files or configurations found."
fi
echo ""
