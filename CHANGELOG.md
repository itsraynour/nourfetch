# Changelog

All notable changes to **nourfetch** are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.5] - 2026-08-16

### Fixed
- **Windows Self-Uninstallation Reliability**:
  - Implemented atomic rename mechanism for Windows executables prior to deletion to bypass Windows file locks immediately.
  - Replaced legacy cmd scheduling with clean background PowerShell execution for zero-trace binary cleanup.
  - Enhanced binary path matching to accurately resolve Windows canonical and environment paths.

---

## [1.0.4] - 2026-08-16

### Added
- **Complete Uninstallation & Clean Removal**:
  - Added `nourfetch uninstall` subcommand and `nourfetch --uninstall` / `-u` flags.
  - Added `-y` / `--yes` / `-f` / `--force` flags for silent non-interactive uninstallation.
  - Safe interactive confirmation prompt preventing accidental removal.
  - Complete wipe of configuration directories (`%APPDATA%\nourfetch`, `~/.config/nourfetch`).
  - Seamless binary removal across Windows, Linux, and macOS (with detached self-deletion scheduling on Windows).
  - Standalone one-line uninstaller scripts: `uninstall.ps1`, `uninstall.sh`, and `uninstall.bat`.


---

## [1.0.3] - 2026-08-16

### Added
- **Complete Uninstallation & Clean Removal**:
  - Added `nourfetch uninstall` subcommand and `nourfetch --uninstall` / `-u` flags.
  - Added `-y` / `--yes` / `-f` / `--force` flags for silent non-interactive uninstallation.
  - Safe interactive confirmation prompt preventing accidental removal.
  - Complete wipe of configuration directories (`%APPDATA%\nourfetch`, `~/.config/nourfetch`).
  - Seamless binary removal across Windows, Linux, and macOS (with detached self-deletion scheduling on Windows).
  - Standalone one-line uninstaller scripts: `uninstall.ps1`, `uninstall.sh`, and `uninstall.bat`.

---

## [1.0.2] - 2026-08-16

### Added
- **Hardware & GPU**: Native zero-dependency GPU and dedicated VRAM detection engine.
  - Windows: Win32 DXGI COM FFI (`IDXGIFactory1`, `IDXGIAdapter1`, `IDXGIAdapter3`) and registry fallback.
  - Linux: Linux DRM sysfs, `nvidia-smi`, `/sys/class/drm/`, and `lspci`.
  - macOS: `system_profiler SPDisplaysDataType` parser.
- **Logos**: Added new ASCII logos:
  - Official GNU collection: `gnu`, `gnu_small`, `guix`.
  - Linux distributions: `gentoo`, `void`, `opensuse`, `artix`, `endeavouros`, `linuxmint`, `zorin`, `elementary`, `slackware`, `freebsd`, `redhat`, `rocky`, `garuda`, `parrot`.
  - Added `--logos` / `--list-logos` command.
- **Configuration Engine**:
  - Added `--save` / `-s` / `--persist` / `-p` flags to persist CLI parameters into `config.toml`.
  - Added `+` prefix syntax (`+theme`, `+logo`, `+layout`, `+modern`, `+compact`, `+nerd-fonts`).
  - Added `nourfetch set <key> <val>` and `nourfetch config <path|get|set|reset>` subcommands.

### Changed
- Refactored JSON schema to include structured GPU and VRAM fields.
- Improved memory and disk bar rendering alignment in modern card layout.

---

## [1.0.1] - 2026-08-15

### Added
- `--install` automated PATH deployment script for Windows, Linux, and macOS.
- Winget and Shell installer one-line scripts (`install.ps1`, `install.sh`).
- Theme auto-detection based on terminal background palette.

### Fixed
- Terminal width detection edge-cases on older Windows ConHost instances.
- Disk usage calculations for mounted network shares.

---

## [1.0.0] - 2026-08-14

### Added
- Initial release of **nourfetch**.
- Zero-dependency system architecture written in pure standard-library Rust.
- Core hardware detection: CPU, RAM, OS, Kernel, Uptime, Shell, Terminal, Resolution.
- Classic, Modern (Card), Compact, and JSON output layouts.
- 10 built-in color themes (nour, cyberpunk, catppuccin, dracula, nord, sunset, matrix, tokyonight, gruvbox, minimal).
- Native TOML configuration parser.
