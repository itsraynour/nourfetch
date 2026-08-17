# nourfetch

```
  _  _  _____  _   _  ____  
 | \| |/ _ \ || | | |  _ \ 
 | .` | (_) | || |_| | |_) |
 |_|\_|\___/ \__/\___/|_| \_\
      N O U R F E T C H   
```

A lightning-fast, aesthetic, zero-dependency system information tool written in 100% pure Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Language-Rust%201.70+-orange.svg)](https://www.rust-lang.org/)
[![Dependencies](https://img.shields.io/badge/Dependencies-0-brightgreen.svg)](#features)
[![Binary Size](https://img.shields.io/badge/Binary%20Size-~280%20KB-success.svg)](#performance)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-informational.svg)](#installation)

---

## Features

- **Zero Dependencies**: Written purely in Rust standard library and raw native OS APIs (Win32 DXGI COM FFI & Registry on Windows, DRM sysfs & `/proc` on Linux, `system_profiler` on macOS).
- **Sub-5ms Execution**: Minimal memory allocation and fast native syscalls.
- **GPU & Dedicated VRAM Detection**:
  - Automatically queries dedicated VRAM capacity, used VRAM, and GPU models across multiple graphics cards.
  - Native DXGI video memory profiling for NVIDIA, AMD, and Intel GPUs.
- **30+ Built-in ASCII Logos**:
  - Official GNU collection: `gnu` (iconic horned head), `gnu_small`, `guix`.
  - Comprehensive Linux distributions: `arch`, `ubuntu`, `fedora`, `debian`, `nixos`, `alpine`, `pop_os`, `manjaro`, `gentoo`, `void`, `opensuse`, `artix`, `endeavouros`, `linuxmint`, `zorin`, `elementary`, `slackware`, `freebsd`, `redhat`, `rocky`, `garuda`, `parrot`, `tux`, `windows11`, `windows10`, `macos`, and custom `nour` badges.
- **Persistent vs Temporary Configuration**:
  - **One-Time Run**: Standard CLI flags apply only to the active execution.
  - **Lifetime Settings**: Save preferences permanently using `--save` / `-s`, `+` shorthand syntax (`nourfetch +theme cyberpunk +logo gnu`), or subcommands (`nourfetch set theme cyberpunk`).
- **TrueColor RGB Output**:
  - Full 24-bit TrueColor rendering with customizable progress bar characters (`smooth`, `block`, `circle`, `rounded`, `ascii`).
  - 10 curated color themes.
- **Compact Binary**: Standalone binary of ~280 KB.

---

## Output Examples

### Classic Layout (Default)
```
  ████████   ████████       Minxio@DESKTOP-OFS4MVS
  ████████   ████████      ────────────────────────
  ████████   ████████      󰣇 OS: Windows 11 Pro (25H2) [x86_64]
  ████████   ████████      󰌢 Host: Dell Inc. XPS 15 7590
                           󰌽 Kernel: Windows NT 26200.9168
  ████████   ████████      󰅐 Uptime: 35m
  ████████   ████████      󰏖 Packages: cargo (14)
  ████████   ████████      󰞷 Shell: PowerShell
  ████████   ████████      󰆍 Terminal: Windows Console (ConHost)
                           󰨇 WM/DE: Desktop Window Manager (DWM)
                           󰍛 CPU: Intel(R) Core(TM) i9-9980HK CPU @ 2.40GHz (16 threads) @ 2.40 GHz
                           󰢮 GPU: Intel(R) UHD Graphics 630 [128.0 MiB]
                           󰢮 GPU: NVIDIA GeForce GTX 1650 [3.85 GiB]
                           󰘚 Memory: 11.4 GiB / 23.7 GiB ███████░░░░░░░ (48.0%)
                           󰋊 Disk (C:): 171.1 GiB / 196.9 GiB ████████████░░ (87%)
                           󰂀 Battery: 100% ██████████████ [Charging]
                           󰍹 Display: 1536x864
                           
                           ████████████████████████
                           ████████████████████████
```

### Modern Card Layout (`--modern`)
```
╭── Minxio@DESKTOP-OFS4MVS ──────────────────────────────────────────╮
├  ◈ SYSTEM ────────────────────────────────────────────────────────┤
│  󰣇 OS: Windows 11 Pro (25H2) [x86_64]                              │
│  󰌢 Host: Dell Inc. XPS 15 7590                                     │
│  󰌽 Kernel: Windows NT 26200.9168                                   │
│  󰅐 Uptime: 35m                                                     │
│  󰞷 Shell: PowerShell                                               │
│  󰆍 Terminal: Windows Console (ConHost)                             │
│  󰏖 Packages: cargo (14)                                            │
├  ◈ HARDWARE ──────────────────────────────────────────────────────┤
│  󰍛 CPU: Intel(R) Core(TM) i9-9980HK CPU @ 2.40GHz (16T) @ 2.40 GHz │
│  󰢮 GPU: Intel(R) UHD Graphics 630 (128.0 MiB)                      │
│  󰢮 GPU: NVIDIA GeForce GTX 1650 (3.85 GiB)                         │
│  󰍹 Display: 1536x864                                               │
├  ◈ RESOURCES ─────────────────────────────────────────────────────┤
│  󰘚 RAM: 11.4 GiB / 23.7 GiB ████░░░░░░ (48.0%)                    │
│  󰋊 Disk (C:): 171.1 GiB / 196.9 GiB █████████░ (87%)              │
│  󰂀 Battery: 100% ██████████ [Charging]                             │
╰────────────────────────────────────────────────────────────────────╯
```

### GNU Logo (`--logo gnu`)
```
    ,-''"""`-.           Minxio@DESKTOP-OFS4MVS
   ,'  ,-.  ,-. `.      ────────────────────────
  /   (   )(   )  \     󰣇 OS: Windows 11 Pro (25H2) [x86_64]
 |     `-'  `-'    |    󰌢 Host: Dell Inc. XPS 15 7590
 |   _          _  |    󰌽 Kernel: Windows NT 26200.9168
 \  ( )  .--.  ( ) /    󰅐 Uptime: 35m
  `.    (    )    ,'    󰏖 Packages: cargo (14)
    `--. `..' ,--'      󰞷 Shell: PowerShell
        `----'          󰆍 Terminal: Windows Console (ConHost)
                        󰨇 WM/DE: Desktop Window Manager (DWM)
                        󰍛 CPU: Intel(R) Core(TM) i9-9980HK CPU @ 2.40GHz (16 threads) @ 2.40 GHz
                        󰢮 GPU: NVIDIA GeForce GTX 1650 [3.85 GiB]
                        󰘚 Memory: 11.4 GiB / 23.7 GiB ███████░░░░░░░ (48.0%)
                        󰋊 Disk (C:): 171.1 GiB / 196.9 GiB ████████████░░ (87%)
```

---

## Comparison

| Feature | `neofetch` (Bash) | `fastfetch` (C) | `nourfetch` (Rust) |
| :--- | :---: | :---: | :---: |
| **Language** | Bash | C | **Pure Rust** |
| **Dependencies** | Multiple external tools | Dynamic C libraries | **0 (Zero)** |
| **Binary Size** | Script | ~2 - 4 MB | **~280 KB** |
| **Execution Time** | ~150 - 500 ms | ~10 - 20 ms | **< 5 ms** |
| **VRAM & GPU Detection** | Limited / External | Dynamic libraries | **Native Win32 DXGI / DRM** |
| **Windows Support** | MSYS / WSL | Win32 / POSIX | **Native Win32 COM & Registry** |
| **Persistent Config Syntax** | Config file only | Config file only | **`+` Syntax, `--save`, Subcommands** |
| **24-bit TrueColor** | Partial | Full | **Built-in RGB Gradients** |

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

### Via Cargo (crates.io)
```bash
cargo install nourfetch
```

### Via Winget (Windows Package Manager)
```cmd
winget install itsraynour.nourfetch
```

### From Source
```bash
git clone https://github.com/itsraynour/nourfetch.git
cd nourfetch
cargo build --release
cargo run --release -- --install
```

---

## Uninstallation

To completely uninstall `nourfetch` and remove all associated configuration files:

### Via nourfetch CLI
```bash
nourfetch uninstall
# or skip confirmation prompt:
nourfetch uninstall -y
```

### Windows (PowerShell One-Liner)
```powershell
irm https://raw.githubusercontent.com/itsraynour/nourfetch/main/uninstall.ps1 | iex
```

### Linux & macOS (Shell One-Liner)
```bash
curl -fsSL https://raw.githubusercontent.com/itsraynour/nourfetch/main/uninstall.sh | sh
```

---

## CLI Usage

```
USAGE:
    nourfetch [OPTIONS]
    nourfetch set <KEY> <VALUE>
    nourfetch config <path|get|set|reset>
    nourfetch uninstall [-y]

ONE-TIME RUN OPTIONS (Temporary):
    -h, --help                 Print help information
    -v, --version              Print version information
        --install              Install nourfetch to system PATH
    -u, --uninstall            Completely uninstall nourfetch and clean all files
    -y, --yes, --force         Skip confirmation prompt during uninstall
    -t, --theme <NAME>         Select color theme for this run
    -l, --logo <NAME>          Select ASCII logo for this run
        --layout <LAYOUT>      Select layout style (classic, modern, compact)
        --modern               Render modern boxed card layout
        --compact              Render minimal inline summary
        --classic              Render classic side-by-side layout
        --no-nerd-fonts        Disable Nerd Font icons for this run
        --no-color             Disable ANSI color output
        --json                 Output complete system & GPU specs in JSON
        --list-themes, --themes  List all available color themes
        --list-logos, --logos    List all available ASCII logos (GNU, Distros, Windows, Mac)
        --gen-config           Generate/restore default config.toml file

PERMANENT SETTINGS (Saved to config.toml):
    -s, --save                 Save all provided flags permanently into config
    +theme <NAME>, +t <NAME>   Set theme permanently (e.g. nourfetch +theme cyberpunk)
    +logo <NAME>, +l <NAME>    Set logo permanently (e.g. nourfetch +logo gnu)
    +layout <LAYOUT>           Set layout permanently (e.g. nourfetch +layout modern)
    +modern, +compact          Set layout mode permanently
    +nerd-fonts, +no-nerd-fonts Set icon preferences permanently

SUBCOMMANDS:
    set <KEY> <VAL>            Set config value (e.g. nourfetch set theme dracula)
    config path                Show config file path
    config get [KEY]           Print current saved configuration
    config set <KEY> <VAL>     Set and persist a config property
    config reset               Reset config back to default settings
    uninstall [-y|--force]     Completely remove nourfetch and its configs from the system
```

---

## Configuration File (`config.toml`)

Configuration path:
- **Windows**: `%APPDATA%\nourfetch\config.toml`
- **Linux / macOS**: `~/.config/nourfetch/config.toml`

```toml
# nourfetch configuration
# Auto-generated and maintained by nourfetch

# Theme palette to use
# Options: "nour", "cyberpunk", "catppuccin", "dracula", "nord", "sunset", "matrix", "tokyonight", "gruvbox", "minimal"
theme = "nour"

# ASCII logo
# Options: "auto", "gnu", "gnu_small", "guix", "arch", "ubuntu", "fedora", "debian", "nixos", "alpine",
#          "pop_os", "manjaro", "gentoo", "void", "opensuse", "artix", "endeavouros",
#          "linuxmint", "zorin", "elementary", "slackware", "freebsd", "redhat", "rocky", "garuda",
#          "parrot", "tux", "windows11", "windows10", "macos", "nour", "badge"
logo = "auto"

# Layout style: "classic", "modern", "compact"
layout = "classic"

# Enable Nerd Font icons (true / false)
nerd_fonts = true

# Display color palette blocks at the bottom (true / false)
color_blocks = true

# Display progress bars for RAM/GPU/Disk (true / false)
show_bars = true

# Progress bar character style: "smooth", "block", "circle", "rounded", "ascii"
bar_style = "smooth"

# Progress bar width in characters
bar_width = 14
```

---

## Themes & Logos

### Available Color Themes
- `nour` (Electric Indigo & Violet)
- `cyberpunk` (Neon Cyan & Hot Pink)
- `catppuccin` (Mauve, Peach, Lavender)
- `dracula` (Purple, Pink, Green)
- `nord` (Polar Frost Blue)
- `sunset` (Amber, Coral, Rose)
- `matrix` (Hacker Phosphor Green)
- `tokyonight` (Tokyo Storm Blue & Violet)
- `gruvbox` (Earthy Retro Amber)
- `minimal` (Monochrome White & Slate)

### Available ASCII Logos (30+)
- **GNU**: `gnu`, `gnu_small`, `guix`
- **Linux Distros**: `arch`, `ubuntu`, `fedora`, `debian`, `nixos`, `alpine`, `pop_os`, `manjaro`, `gentoo`, `void`, `opensuse`, `artix`, `endeavouros`, `linuxmint`, `zorin`, `elementary`, `slackware`, `redhat`, `rocky`, `garuda`, `parrot`, `tux`
- **BSD**: `freebsd`
- **Windows**: `windows11`, `windows10`, `windows_modern`
- **macOS**: `macos`
- **Signatures**: `nour`, `badge`

---

## Security & Privacy

`nourfetch` is designed with strict security principles:
1. **Zero External Dependencies**: Zero third-party crates to prevent supply-chain vulnerabilities.
2. **Zero Network Connections**: Does not send telemetry, ping servers, or track usage.
3. **Pure Native Syscalls**: Direct query through OS kernel APIs in memory.

---

## License

MIT License. See [LICENSE](LICENSE) for details.

