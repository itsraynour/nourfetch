# nourfetch

```
  _  _  _____  _   _  ____  
 | \| |/ _ \ || | | |  _ \ 
 | .` | (_) | || |_| | |_) |
 |_|\_|\___/ \__/\___/|_| \_\
      N O U R F E T C H   
```

A fast, lightweight, zero-dependency system information tool written in pure Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Language-Rust%201.70+-orange.svg)](https://www.rust-lang.org/)
[![Dependencies](https://img.shields.io/badge/Dependencies-0-brightgreen.svg)](#features)
[![Binary Size](https://img.shields.io/badge/Binary%20Size-265%20KB-success.svg)](#performance)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-informational.svg)](#installation)

---

## Features

- **Zero Third-Party Dependencies**: Built exclusively with Rust standard library and native operating system interfaces (Win32 FFI & Registry on Windows; `/proc` and `/sys` virtual filesystems on Linux).
- **Fast Startup**: Executes and renders in under 5 milliseconds.
- **Small Binary Footprint**: Single standalone binary of ~265 KB.
- **24-bit TrueColor Support**: Native RGB linear interpolation gradients, visual progress bars, and palette blocks.
- **Multiple Layout Modes**:
  - Classic side-by-side view (default)
  - Card/Box view (`--modern`)
  - Compact inline view (`--compact`)
  - Structured JSON output (`--json`)
- **OS Logos & Themes**: 15+ built-in ASCII logos and 10 color schemes.
- **Offline & Private**: No background services, no telemetry, and zero network calls.

---

## Output Examples

### Classic Layout (Default)
```
  ████████   ████████       rayane@workstation
  ████████   ████████      ─────────────────────
  ████████   ████████      󰣇 OS: Windows 11 Pro (25H2) [x86_64]
  ████████   ████████      󰌢 Host: Dell Inc. XPS 15 7590
                           󰌽 Kernel: Windows NT 26200.9168
  ████████   ████████      󰅐 Uptime: 2h 15m
  ████████   ████████      󰏖 Packages: cargo (15)
  ████████   ████████      󰞷 Shell: PowerShell
  ████████   ████████      󰆍 Terminal: Windows Console (ConHost)
                           󰨇 WM/DE: Desktop Window Manager (DWM)
                           󰍛 CPU: Intel(R) Core(TM) i9-9980HK CPU @ 2.40GHz (16 threads) @ 2.40 GHz
                           󰢮 GPU: NVIDIA GeForce GTX 1650
                           󰘚 Memory: 14.3 GiB / 23.7 GiB ████████░░░░░░ (60.4%)
                           󰋊 Disk (C:): 168.3 GiB / 196.9 GiB ████████████░░ (85%)
                           󰂀 Battery: 100% ██████████████ [Charging]
                           󰍹 Display: 1536x864
                           
                           ████████████████████████
                           ████████████████████████
```

### Modern Card Layout (`--modern`)
```
╭── rayane@workstation ──────────────────────────────────────────────╮
├  ◈ SYSTEM ────────────────────────────────────────────────────────┤
│  󰣇 OS: Windows 11 Pro (25H2) [x86_64]                              │
│  󰌢 Host: Dell Inc. XPS 15 7590                                     │
│  󰌽 Kernel: Windows NT 26200.9168                                   │
│  󰅐 Uptime: 2h 15m                                                  │
│  󰞷 Shell: PowerShell                                               │
│  󰆍 Terminal: Windows Console (ConHost)                             │
│  󰏖 Packages: cargo (15)                                            │
├  ◈ HARDWARE ──────────────────────────────────────────────────────┤
│  󰍛 CPU: Intel(R) Core(TM) i9-9980HK CPU @ 2.40GHz (16T) @ 2.40 GHz │
│  󰢮 GPU: NVIDIA GeForce GTX 1650                                    │
│  󰍹 Display: 1536x864                                               │
├  ◈ RESOURCES ─────────────────────────────────────────────────────┤
│  󰘚 RAM: 14.3 GiB / 23.7 GiB ██████░░░░ (60.4%)                     │
│  󰋊 Disk (C:): 168.3 GiB / 196.9 GiB █████████░ (85%)              │
│  󰂀 Battery: 100% ██████████ [Charging]                             │
╰────────────────────────────────────────────────────────────────────╯
```

### Compact Layout (`--compact`)
```
 rayane@workstation │ 󰣇 Windows 11 Pro │ 󰌽 Windows NT 26200 │ 󰅐 2h 15m │ 󰘚 14.3 GiB/23.7 GiB (60%) │ 󰞷 PowerShell
```

---

## Comparison

| Feature | `neofetch` (Bash) | `fastfetch` (C) | `nourfetch` (Rust) |
| :--- | :---: | :---: | :---: |
| **Language** | Bash | C | **Pure Rust** |
| **Dependencies** | Multiple external tools | Dynamic C libraries | **0 (Zero)** |
| **Binary Size** | Script | ~2 - 4 MB | **~265 KB** |
| **Execution Time** | ~150 - 500 ms | ~10 - 20 ms | **< 5 ms** |
| **Windows Support** | MSYS / WSL | Win32 / POSIX | **Native Win32 FFI & Registry** |
| **24-bit TrueColor** | Partial | Full | **Built-in RGB Gradients** |
| **Card UI Layout** | No | No | **Yes (`--modern`)** |
| **JSON Output** | No | Yes | **Yes (`--json`)** |

---

## Installation

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/itsraynour/nourfetch/main/install.ps1 | iex
```

### Linux & macOS
```bash
curl -fsSL https://raw.githubusercontent.com/itsraynour/nourfetch/main/install.sh | sh
```

### From Source
```bash
git clone https://github.com/itsraynour/nourfetch.git
cd nourfetch
cargo build --release
cargo install --path .
```

---

## Usage

```
USAGE:
    nourfetch [OPTIONS]

OPTIONS:
    -h, --help                 Print help information
    -v, --version              Print version information
        --install              Install nourfetch to system PATH
    -t, --theme <NAME>         Select color theme (nour, cyberpunk, dracula, nord, etc.)
    -l, --logo <NAME>          Select ASCII logo (windows11, arch, ubuntu, nour, etc.)
        --layout <LAYOUT>      Select layout style (classic, modern, compact)
        --compact              Render minimal inline summary
        --modern               Render modern boxed card layout
        --json                 Output system specs in JSON format
        --no-color             Disable ANSI color output
        --no-nerd-fonts        Disable Nerd Font icons
        --gen-config           Generate default config.toml file
        --list-themes          List available color themes
        --list-logos           List available ASCII logos
```

---

## Configuration

Generate a default configuration file with:
```bash
nourfetch --gen-config
```

Configuration path:
- **Windows**: `%APPDATA%\nourfetch\config.toml`
- **Linux / macOS**: `~/.config/nourfetch/config.toml`

```toml
# nourfetch configuration
theme = "nour"
layout = "classic"
logo = "auto"
nerd_fonts = true
show_bars = true
bar_style = "smooth"
bar_width = 14
color_blocks = true
```

### Available Themes
- `nour` (default)
- `cyberpunk`
- `catppuccin`
- `dracula`
- `nord`
- `sunset`
- `matrix`
- `tokyonight`
- `gruvbox`
- `minimal`

---

## License

MIT License. See [LICENSE](LICENSE) for details.
