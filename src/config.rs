#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub theme: String,
    pub logo: String,
    pub layout: String,
    pub nerd_fonts: bool,
    pub color_blocks: bool,
    pub show_bars: bool,
    pub bar_style: String,
    pub bar_width: usize,
    pub enabled_modules: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "nour".to_string(),
            logo: "auto".to_string(),
            layout: "classic".to_string(),
            nerd_fonts: true,
            color_blocks: true,
            show_bars: true,
            bar_style: "smooth".to_string(),
            bar_width: 14,
            enabled_modules: vec![
                "title".to_string(),
                "separator".to_string(),
                "os".to_string(),
                "host".to_string(),
                "kernel".to_string(),
                "uptime".to_string(),
                "packages".to_string(),
                "shell".to_string(),
                "terminal".to_string(),
                "wm".to_string(),
                "cpu".to_string(),
                "gpu".to_string(),
                "memory".to_string(),
                "disk".to_string(),
                "battery".to_string(),
                "resolution".to_string(),
                "separator".to_string(),
                "colors".to_string(),
            ],
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let mut config = Self::default();
        if let Some(path) = Self::get_config_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    config.parse_simple_toml(&content);
                }
            }
        }
        config
    }

    pub fn get_config_path() -> Option<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            if let Ok(appdata) = env::var("APPDATA") {
                return Some(PathBuf::from(appdata).join("nourfetch").join("config.toml"));
            }
            if let Ok(userprofile) = env::var("USERPROFILE") {
                return Some(PathBuf::from(userprofile).join(".config").join("nourfetch").join("config.toml"));
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
                return Some(PathBuf::from(xdg_config).join("nourfetch").join("config.toml"));
            }
            if let Ok(home) = env::var("HOME") {
                return Some(PathBuf::from(home).join(".config").join("nourfetch").join("config.toml"));
            }
        }

        None
    }

    fn parse_simple_toml(&mut self, content: &str) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }

            if let Some((k, v)) = line.split_once('=') {
                let key = k.trim();
                let val = v.trim().trim_matches('"').trim_matches('\'');
                match key {
                    "theme" => self.theme = val.to_string(),
                    "logo" => self.logo = val.to_string(),
                    "layout" => self.layout = val.to_string(),
                    "nerd_fonts" => self.nerd_fonts = val.parse().unwrap_or(true),
                    "color_blocks" => self.color_blocks = val.parse().unwrap_or(true),
                    "show_bars" => self.show_bars = val.parse().unwrap_or(true),
                    "bar_style" => self.bar_style = val.to_string(),
                    "bar_width" => self.bar_width = val.parse().unwrap_or(14),
                    _ => {}
                }
            }
        }
    }

    pub fn generate_default_file() -> Result<PathBuf, String> {
        let path = Self::get_config_path().ok_or_else(|| "Could not determine config directory".to_string())?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        let template = r#"# nourfetch configuration

# Theme palette to use
# Options: "nour", "cyberpunk", "catppuccin", "dracula", "nord", "sunset", "matrix", "tokyonight", "gruvbox", "minimal"
theme = "nour"

# ASCII logo
# Options: "auto", "nour", "badge", "windows11", "windows10", "arch", "ubuntu", "fedora", "debian", "nixos", "alpine", "pop_os", "manjaro", "tux", "macos"
logo = "auto"

# Layout style
# Options: "classic", "modern", "compact"
layout = "classic"

nerd_fonts = true
color_blocks = true
show_bars = true

# Progress bar character style: "smooth", "block", "circle", "rounded", "ascii"
bar_style = "smooth"
bar_width = 14
"#;

        fs::write(&path, template).map_err(|e| format!("Failed to write config file: {}", e))?;
        Ok(path)
    }
}
