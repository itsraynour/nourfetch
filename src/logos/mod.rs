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
            "windows" | "win" | "windows11" | "win11" => windows::windows_11(),
            "windows10" | "win10" => windows::windows_10(),
            "windows_modern" => windows::windows_modern(),
            "gnu" => linux::gnu(),
            "gnu_small" | "gnusmall" => linux::gnu_small(),
            "guix" | "gnu_guix" => linux::guix(),
            "arch" | "archlinux" => linux::arch(),
            "arch_small" | "archlinux_small" | "archsmall" => linux::arch_small(),
            "ubuntu" => linux::ubuntu(),
            "ubuntu_small" | "ubuntusmall" => linux::ubuntu_small(),
            "fedora" => linux::fedora(),
            "fedora_small" | "fedorasmall" => linux::fedora_small(),
            "fedora_old" | "fedoraold" => linux::fedora_old(),
            "debian" => linux::debian(),
            "debian_small" | "debiansmall" => linux::debian_small(),
            "nixos" | "nix" => linux::nixos(),
            "nixos_small" | "nixossmall" | "nix_small" => linux::nixos_small(),
            "nixos_old" | "nixosold" | "nix_old" => linux::nixos_old(),
            "alpine" => linux::alpine(),
            "alpine_small" | "alpinesmall" => linux::alpine_small(),
            "pop" | "pop_os" | "popos" => linux::pop_os(),
            "pop_os_small" | "popos_small" | "popossmall" | "pop_small" => linux::pop_os_small(),
            "manjaro" => linux::manjaro(),
            "manjaro_small" | "manjarosmall" => linux::manjaro_small(),
            "gentoo" => linux::gentoo(),
            "gentoo_small" | "gentoosmall" => linux::gentoo_small(),
            "kali" | "kali_linux" => linux::kali(),
            "void" | "void_linux" => linux::void(),
            "opensuse" | "suse" => linux::opensuse(),
            "opensuse_small" | "suse_small" | "susesmall" | "opensusessmall" => linux::opensuse_small(),
            "artix" => linux::artix(),
            "artix_small" | "artixsmall" => linux::artix_small(),
            "endeavouros" | "endeavour" => linux::endeavouros(),
            "mint" | "linuxmint" => linux::linuxmint(),
            "linuxmint_small" | "mint_small" | "mintsmall" => linux::linuxmint_small(),
            "linuxmint_old" | "mint_old" | "mintold" | "linuxmintold" => linux::linuxmint_old(),
            "zorin" | "zorin_os" => linux::zorin(),
            "zorin_small" | "zorin_os_small" | "zorinsmall" => linux::zorin_small(),
            "elementary" | "elementary_os" => linux::elementary(),
            "elementary_small" | "elementarysmall" => linux::elementary_small(),
            "slackware" => linux::slackware(),
            "slackware_small" | "slackwaresmall" | "slack_small" => linux::slackware_small(),
            "freebsd" | "bsd" => linux::freebsd(),
            "freebsd_small" | "freebsdsmall" | "bsd_small" => linux::freebsd_small(),
            "redhat" | "rhel" => linux::redhat(),
            "redhat_small" | "rhel_small" | "redhatsmall" | "rhelsmall" => linux::redhat_small(),
            "rocky" | "rocky_linux" => linux::rocky(),
            "rocky_small" | "rockysmall" => linux::rocky_small(),
            "garuda" => linux::garuda(),
            "parrot" | "parrot_os" => linux::parrot(),
            "parrot_small" | "parrotsmall" => linux::parrot_small(),
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
            "gnu",
            "gnu_small",
            "guix",
            "arch",
            "arch_small",
            "ubuntu",
            "ubuntu_small",
            "fedora",
            "fedora_small",
            "fedora_old",
            "debian",
            "debian_small",
            "nixos",
            "nixos_small",
            "nixos_old",
            "alpine",
            "alpine_small",
            "pop_os",
            "pop_os_small",
            "manjaro",
            "manjaro_small",
            "gentoo",
            "gentoo_small",
            "kali",
            "void",
            "opensuse",
            "opensuse_small",
            "artix",
            "artix_small",
            "endeavouros",
            "linuxmint",
            "linuxmint_small",
            "linuxmint_old",
            "zorin",
            "zorin_small",
            "elementary",
            "elementary_small",
            "slackware",
            "slackware_small",
            "freebsd",
            "freebsd_small",
            "redhat",
            "redhat_small",
            "rocky",
            "rocky_small",
            "garuda",
            "parrot",
            "parrot_small",
            "tux / linux",
            "macos",
            "nour / cyber",
            "badge",
        ]
    }
}
