#![allow(dead_code)]

use std::env;
use std::path::Path;
use super::SystemInfo;

pub fn detect_shell() -> String {
    if let Ok(shell_path) = env::var("SHELL") {
        if let Some(name) = Path::new(&shell_path).file_name().and_then(|n| n.to_str()) {
            return format_shell_name(name);
        }
    }

    if env::var("NU_VERSION").is_ok() {
        return "Nushell".to_string();
    }

    if env::var("FISH_VERSION").is_ok() {
        return "fish".to_string();
    }

    if env::var("ZSH_VERSION").is_ok() {
        return "zsh".to_string();
    }

    if env::var("BASH_VERSION").is_ok() {
        return "bash".to_string();
    }

    if env::var("PSModulePath").is_ok() {
        return "PowerShell".to_string();
    }

    if let Ok(comspec) = env::var("COMSPEC") {
        if let Some(name) = Path::new(&comspec).file_stem().and_then(|n| n.to_str()) {
            return format_shell_name(name);
        }
    }

    "sh".to_string()
}

fn format_shell_name(raw: &str) -> String {
    let lower = raw.to_lowercase();
    if lower.contains("pwsh") || lower.contains("powershell") {
        "PowerShell".to_string()
    } else if lower.contains("cmd") {
        "CMD".to_string()
    } else if lower.contains("nu") {
        "Nushell".to_string()
    } else if lower.contains("fish") {
        "fish".to_string()
    } else if lower.contains("zsh") {
        "zsh".to_string()
    } else if lower.contains("bash") {
        "bash".to_string()
    } else {
        raw.to_string()
    }
}

pub fn detect_terminal() -> String {
    if env::var("WT_SESSION").is_ok() {
        return "Windows Terminal".to_string();
    }

    if let Ok(term_prog) = env::var("TERM_PROGRAM") {
        let trimmed = term_prog.trim();
        if !trimmed.is_empty() {
            if trimmed.eq_ignore_ascii_case("vscode") {
                return "VS Code Terminal".to_string();
            } else if trimmed.eq_ignore_ascii_case("alacritty") {
                return "Alacritty".to_string();
            } else if trimmed.eq_ignore_ascii_case("wezterm") {
                return "WezTerm".to_string();
            } else if trimmed.eq_ignore_ascii_case("kitty") {
                return "Kitty".to_string();
            } else if trimmed.eq_ignore_ascii_case("iterm.app") {
                return "iTerm2".to_string();
            } else if trimmed.eq_ignore_ascii_case("apple_terminal") {
                return "Apple Terminal".to_string();
            }
            return trimmed.to_string();
        }
    }

    if env::var("ALACRITTY_LOG").is_ok() || env::var("ALACRITTY_WINDOW_ID").is_ok() {
        return "Alacritty".to_string();
    }

    if env::var("KITTY_WINDOW_ID").is_ok() {
        return "Kitty".to_string();
    }

    if env::var("WEZTERM_EXECUTABLE").is_ok() || env::var("WEZTERM_PANE").is_ok() {
        return "WezTerm".to_string();
    }

    if env::var("TERMINATOR_UUID").is_ok() {
        return "Terminator".to_string();
    }

    if let Ok(term) = env::var("TERM") {
        let lower = term.to_lowercase();
        if lower.contains("xterm") {
            return "xterm".to_string();
        } else if lower.contains("screen") {
            return "GNU Screen".to_string();
        } else if lower.contains("tmux") {
            return "tmux".to_string();
        } else if lower.contains("alacritty") {
            return "Alacritty".to_string();
        } else if lower.contains("kitty") {
            return "Kitty".to_string();
        } else if !term.is_empty() && term != "dumb" {
            return term;
        }
    }

    #[cfg(target_os = "windows")]
    {
        "Windows Console (ConHost)".to_string()
    }
    #[cfg(not(target_os = "windows"))]
    {
        "TTY".to_string()
    }
}

pub fn fetch_fallback_info() -> SystemInfo {
    let mut info = SystemInfo::default();
    info.username = env::var("USER").or_else(|_| env::var("USERNAME")).unwrap_or_else(|_| "user".to_string());
    info.hostname = env::var("HOSTNAME").or_else(|_| env::var("COMPUTERNAME")).unwrap_or_else(|_| "localhost".to_string());
    info.os_name = std::env::consts::OS.to_string();
    info.os_arch = std::env::consts::ARCH.to_string();
    info.shell = detect_shell();
    info.terminal = detect_terminal();
    info.os_key = "unknown".to_string();
    info
}
