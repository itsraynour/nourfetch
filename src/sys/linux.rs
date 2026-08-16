#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use super::common::{detect_shell, detect_terminal};
use super::{BatteryInfo, CpuInfo, DiskInfo, DisplayInfo, MemoryInfo, SystemInfo};

pub fn fetch_linux_info() -> SystemInfo {
    let mut info = SystemInfo::default();

    info.username = env::var("USER").unwrap_or_else(|_| "user".to_string());
    info.hostname = fs::read_to_string("/etc/hostname")
        .map(|s| s.trim().to_string())
        .or_else(|_| env::var("HOSTNAME"))
        .unwrap_or_else(|_| "localhost".to_string());

    let (os_name, os_ver, os_id) = parse_os_release();
    info.os_name = os_name;
    info.os_version = os_ver;
    info.os_arch = std::env::consts::ARCH.to_string();
    info.os_key = os_id;

    if let Ok(version_str) = fs::read_to_string("/proc/version") {
        if let Some(first_word) = version_str.split("version ").nth(1) {
            if let Some(kernel_ver) = first_word.split_whitespace().next() {
                info.kernel = kernel_ver.to_string();
            }
        }
    }
    if info.kernel.is_empty() {
        if let Ok(sys_release) = fs::read_to_string("/proc/sys/kernel/osrelease") {
            info.kernel = sys_release.trim().to_string();
        }
    }

    info.host_model = detect_linux_host();

    if let Ok(uptime_str) = fs::read_to_string("/proc/uptime") {
        if let Some(first) = uptime_str.split_whitespace().next() {
            if let Ok(secs_f) = first.parse::<f64>() {
                info.uptime_seconds = secs_f.round() as u64;
            }
        }
    }

    info.shell = detect_shell();
    info.terminal = detect_terminal();
    info.wm_de = detect_linux_wm_de();

    info.cpu = parse_linux_cpu();
    info.gpu = detect_linux_gpus();
    info.memory = parse_linux_mem();
    info.disks = detect_linux_disks();
    info.battery = detect_linux_battery();
    info.displays = detect_linux_displays();

    let (count, managers) = count_linux_packages();
    info.packages_count = count;
    info.package_managers = managers;

    info
}

fn parse_os_release() -> (String, String, String) {
    let paths = ["/etc/os-release", "/usr/lib/os-release", "/etc/lsb-release"];
    for p in &paths {
        if let Ok(content) = fs::read_to_string(p) {
            let mut name = String::new();
            let mut version = String::new();
            let mut id = String::new();

            for line in content.lines() {
                let line = line.trim();
                if let Some((k, v)) = line.split_once('=') {
                    let val = v.trim_matches('"').trim_matches('\'').trim();
                    match k {
                        "PRETTY_NAME" if name.is_empty() => name = val.to_string(),
                        "NAME" if name.is_empty() => name = val.to_string(),
                        "VERSION_ID" | "VERSION" if version.is_empty() => version = val.to_string(),
                        "ID" => id = val.to_lowercase(),
                        _ => {}
                    }
                }
            }
            if !name.is_empty() {
                if id.is_empty() {
                    id = name.to_lowercase();
                }
                return (name, version, id);
            }
        }
    }
    ("Linux".to_string(), String::new(), "linux".to_string())
}

fn detect_linux_host() -> String {
    let vendor = fs::read_to_string("/sys/class/dmi/id/sys_vendor")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    let product = fs::read_to_string("/sys/class/dmi/id/product_name")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    let version = fs::read_to_string("/sys/class/dmi/id/product_version")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    if !product.is_empty() && !product.eq_ignore_ascii_case("system product name") {
        if !vendor.is_empty() && !product.contains(&vendor) {
            format!("{} {}", vendor, product)
        } else {
            product
        }
    } else if !version.is_empty() {
        format!("{} {}", vendor, version)
    } else if let Ok(model) = fs::read_to_string("/proc/device-tree/model") {
        model.trim_matches('\0').trim().to_string()
    } else {
        "PC / Generic Linux Host".to_string()
    }
}

fn detect_linux_wm_de() -> String {
    let de = env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .or_else(|_| env::var("XDG_SESSION_DESKTOP"))
        .unwrap_or_default();

    let session_type = env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| {
        if env::var("WAYLAND_DISPLAY").is_ok() {
            "wayland".to_string()
        } else if env::var("DISPLAY").is_ok() {
            "x11".to_string()
        } else {
            "tty".to_string()
        }
    });

    if !de.is_empty() {
        format!("{} ({})", de, session_type)
    } else {
        session_type
    }
}

fn parse_linux_cpu() -> CpuInfo {
    let mut cpu = CpuInfo::default();
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        let mut threads = 0;
        for line in content.lines() {
            if let Some((k, v)) = line.split_once(':') {
                let key = k.trim();
                let val = v.trim();
                if key == "model name" && cpu.model.is_empty() {
                    cpu.model = val.to_string();
                } else if key == "cpu MHz" && cpu.freq_mhz == 0 {
                    if let Ok(mhz_f) = val.parse::<f64>() {
                        cpu.freq_mhz = mhz_f.round() as u64;
                    }
                } else if key == "processor" {
                    threads += 1;
                }
            }
        }
        cpu.threads = threads;
        cpu.cores = threads;
    }
    if cpu.model.is_empty() {
        cpu.model = "Generic Linux Processor".to_string();
    }
    cpu
}

fn parse_linux_mem() -> MemoryInfo {
    let mut total = 0u64;
    let mut avail = 0u64;
    let mut free = 0u64;
    let mut buffers = 0u64;
    let mut cached = 0u64;

    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            let mut parts = line.split_whitespace();
            if let (Some(key), Some(val_str)) = (parts.next(), parts.next()) {
                let kb: u64 = val_str.parse().unwrap_or(0);
                let bytes = kb * 1024;
                match key {
                    "MemTotal:" => total = bytes,
                    "MemAvailable:" => avail = bytes,
                    "MemFree:" => free = bytes,
                    "Buffers:" => buffers = bytes,
                    "Cached:" => cached = bytes,
                    _ => {}
                }
            }
        }
    }

    let actual_used = if avail > 0 {
        total.saturating_sub(avail)
    } else {
        total.saturating_sub(free + buffers + cached)
    };

    let pct = if total > 0 {
        (actual_used as f32 / total as f32) * 100.0
    } else {
        0.0
    };

    MemoryInfo {
        total_bytes: total,
        used_bytes: actual_used,
        free_bytes: if avail > 0 { avail } else { free },
        usage_percent: pct,
    }
}

fn detect_linux_gpus() -> Vec<String> {
    let mut gpus = Vec::new();
    let drm_path = Path::new("/sys/class/drm");
    if drm_path.exists() {
        if let Ok(entries) = fs::read_dir(drm_path) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if file_name.starts_with("card") && !file_name.contains('-') {
                    let device_vendor = path.join("device/vendor");
                    if device_vendor.exists() {
                        let name = format!("GPU ({})", file_name);
                        if !gpus.contains(&name) {
                            gpus.push(name);
                        }
                    }
                }
            }
        }
    }
    gpus
}

fn detect_linux_disks() -> Vec<DiskInfo> {
    let mut disks = Vec::new();
    if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let dev = parts[0];
                let mount = parts[1];
                let fs_type = parts[2];

                if (dev.starts_with("/dev/sd")
                    || dev.starts_with("/dev/nvme")
                    || dev.starts_with("/dev/vd")
                    || dev.starts_with("/dev/mapper")
                    || mount == "/")
                    && !mount.starts_with("/boot")
                    && !mount.starts_with("/snap")
                {
                    disks.push(DiskInfo {
                        mount: mount.to_string(),
                        total_bytes: 0,
                        used_bytes: 0,
                        free_bytes: 0,
                        usage_percent: 0.0,
                        fs_type: fs_type.to_string(),
                    });
                }
            }
        }
    }
    disks
}

fn detect_linux_battery() -> Option<BatteryInfo> {
    let ps_dir = Path::new("/sys/class/power_supply");
    if let Ok(entries) = fs::read_dir(ps_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with("BAT") {
                let cap_file = path.join("capacity");
                let status_file = path.join("status");

                let pct: u8 = fs::read_to_string(cap_file)
                    .ok()
                    .and_then(|s| s.trim().parse().ok())
                    .unwrap_or(0);

                let status = fs::read_to_string(status_file)
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|_| "Unknown".to_string());

                let is_charging = status.eq_ignore_ascii_case("charging");

                return Some(BatteryInfo {
                    percentage: pct,
                    is_charging,
                    state: status,
                });
            }
        }
    }
    None
}

fn detect_linux_displays() -> Vec<DisplayInfo> {
    Vec::new()
}

fn count_linux_packages() -> (usize, String) {
    let mut count = 0;
    let mut managers = Vec::new();

    let pacman_dir = Path::new("/var/lib/pacman/local");
    if pacman_dir.exists() {
        if let Ok(entries) = fs::read_dir(pacman_dir) {
            let c = entries.filter_map(Result::ok).count().saturating_sub(1);
            if c > 0 {
                count += c;
                managers.push(format!("pacman ({})", c));
            }
        }
    }

    let dpkg_status = Path::new("/var/lib/dpkg/status");
    if dpkg_status.exists() {
        if let Ok(content) = fs::read_to_string(dpkg_status) {
            let c = content.lines().filter(|l| l.starts_with("Package: ")).count();
            if c > 0 {
                count += c;
                managers.push(format!("dpkg ({})", c));
            }
        }
    }

    let nix_system = Path::new("/run/current-system/sw/bin");
    let nix_user = std::env::var("HOME").ok().map(|h| PathBuf::from(h).join(".nix-profile/bin"));
    let mut nix_count = 0;

    if nix_system.exists() {
        if let Ok(entries) = fs::read_dir(nix_system) {
            nix_count += entries.filter_map(Result::ok).count();
        }
    }

    if let Some(user_path) = nix_user {
        if user_path.exists() && user_path != nix_system {
            if let Ok(entries) = fs::read_dir(user_path) {
                nix_count += entries.filter_map(Result::ok).count();
            }
        }
    }

    if nix_count > 0 {
        count += nix_count;
        managers.push(format!("nix ({})", nix_count));
    }

    let flatpak_dir = Path::new("/var/lib/flatpak/app");
    if flatpak_dir.exists() {
        if let Ok(entries) = fs::read_dir(flatpak_dir) {
            let c = entries.filter_map(Result::ok).count();
            if c > 0 {
                count += c;
                managers.push(format!("flatpak ({})", c));
            }
        }
    }

    let snap_dir = Path::new("/var/lib/snapd/snaps");
    if snap_dir.exists() {
        if let Ok(entries) = fs::read_dir(snap_dir) {
            let c = entries.filter_map(Result::ok).count();
            if c > 0 {
                count += c;
                managers.push(format!("snap ({})", c));
            }
        }
    }

    let rpm_dir = Path::new("/var/lib/rpm");
    if rpm_dir.exists() && managers.is_empty() {
        managers.push("rpm".to_string());
    }

    (count, managers.join(", "))
}
