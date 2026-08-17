#![allow(warnings)]
#![allow(clippy::all)]
#![allow(dead_code)]

use std::env;
use std::process::Command;

use super::common::{detect_shell, detect_terminal};
use super::{GpuInfo, SystemInfo};

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

    info.gpu = detect_macos_gpus();

    info
}

fn detect_macos_gpus() -> Vec<GpuInfo> {
    let mut gpus = Vec::new();
    if let Ok(output) = Command::new("system_profiler").arg("SPDisplaysDataType").output() {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            let mut current_gpu = GpuInfo::default();
            for line in text.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("Chipset Model:") {
                    if !current_gpu.name.is_empty() {
                        gpus.push(current_gpu);
                        current_gpu = GpuInfo::default();
                    }
                    current_gpu.name = trimmed.trim_start_matches("Chipset Model:").trim().to_string();
                    if current_gpu.name.contains("Apple") {
                        current_gpu.vendor = "Apple".to_string();
                    } else if current_gpu.name.contains("Intel") {
                        current_gpu.vendor = "Intel".to_string();
                    } else if current_gpu.name.contains("AMD") || current_gpu.name.contains("Radeon") {
                        current_gpu.vendor = "AMD".to_string();
                    } else if current_gpu.name.contains("NVIDIA") {
                        current_gpu.vendor = "NVIDIA".to_string();
                    }
                } else if trimmed.starts_with("VRAM (Total):") {
                    let vram_str = trimmed.trim_start_matches("VRAM (Total):").trim();
                    if let Some(mb_str) = vram_str.strip_suffix("MB").or_else(|| vram_str.strip_suffix(" MB")) {
                        if let Ok(mb) = mb_str.trim().parse::<u64>() {
                            current_gpu.total_vram_bytes = mb * 1024 * 1024;
                        }
                    } else if let Some(gb_str) = vram_str.strip_suffix("GB").or_else(|| vram_str.strip_suffix(" GB")) {
                        if let Ok(gb) = gb_str.trim().parse::<u64>() {
                            current_gpu.total_vram_bytes = gb * 1024 * 1024 * 1024;
                        }
                    }
                }
            }
            if !current_gpu.name.is_empty() {
                gpus.push(current_gpu);
            }
        }
    }
    gpus
}
