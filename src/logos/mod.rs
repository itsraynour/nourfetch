pub mod custom;
pub mod linux;
pub mod macos;
pub mod windows;

use crate::utils::ansi::Rgb;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Logo {
    pub name: &'static str,
    pub lines: Vec<&'static str>,
    pub primary_color: Rgb,
    pub secondary_color: Rgb,
    pub accent_color: Rgb,
}

impl Logo {
    pub fn resolve(key: &str) -> Self {
        let normalized = key.trim().to_lowercase();
        match normalized.as_str() {
            "windows" | "win" | "windows10" | "win10" => windows::windows_10(),
            "windows11" | "win11" => windows::windows_11(),
            "windows_modern" => windows::windows_modern(),
            "arch" | "archlinux" => linux::arch(),
            "ubuntu" => linux::ubuntu(),
            "fedora" => linux::fedora(),
            "debian" => linux::debian(),
            "nixos" | "nix" => linux::nixos(),
            "alpine" => linux::alpine(),
            "pop" | "pop_os" | "popos" => linux::pop_os(),
            "manjaro" => linux::manjaro(),
            "linux" | "tux" => linux::tux(),
            "macos" | "apple" | "darwin" | "osx" => macos::macos(),
            "nour" | "nourfetch" | "cyber" => custom::nour_signature(),
            "badge" => custom::nour_badge(),
            _ => {
                #[cfg(target_os = "windows")]
                {
                    windows::windows_11()
                }
                #[cfg(target_os = "linux")]
                {
                    linux::tux()
                }
                #[cfg(target_os = "macos")]
                {
                    macos::macos()
                }
                #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
                {
                    custom::generic()
                }
            }
        }
    }

    pub fn list_all() -> &'static [&'static str] {
        &[
            "windows11",
            "windows10",
            "windows_modern",
            "arch",
            "ubuntu",
            "fedora",
            "debian",
            "nixos",
            "alpine",
            "pop_os",
            "manjaro",
            "tux / linux",
            "macos",
            "nour / cyber",
            "badge",
        ]
    }
}
