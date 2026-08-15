#![allow(dead_code)]

use std::env;
use std::process::Command;

use super::common::{detect_shell, detect_terminal};
use super::{BatteryInfo, CpuInfo, MemoryInfo, SystemInfo};

pub fn fetch_macos_info() -> SystemInfo {
    let mut info = SystemInfo::default();

    info.username = env::var("USER").unwrap_or_else(|_| "user".to_string());
    info.hostname = env::var("HOSTNAME").unwrap_or_else(|_| "MacBook".to_string());
    info.os_name = "macOS".to_string();
    info.os_arch = std::env::consts::ARCH.to_string();
    info.os_key = "macos".to_string();

    info.shell = detect_shell();
    info.terminal = detect_terminal();
    info.wm_de = "Aqua / Quartz Compositor".to_string();

    if let Ok(output) = Command::new("sw_vers").arg("-productVersion").output() {
        if output.status.success() {
            info.os_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }

    if let Ok(output) = Command::new("uname").arg("-r").output() {
        if output.status.success() {
            info.kernel = format!("Darwin {}", String::from_utf8_lossy(&output.stdout).trim());
        }
    }

    if let Ok(output) = Command::new("sysctl").arg("-n").arg("machdep.cpu.brand_string").output() {
        if output.status.success() {
            info.cpu.model = String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }
    if let Ok(output) = Command::new("sysctl").arg("-n").arg("hw.model").output() {
        if output.status.success() {
            info.host_model = String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }

    info
}
