pub mod common;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[derive(Debug, Clone, Default)]
pub struct CpuInfo {
    pub model: String,
    pub cores: usize,
    pub threads: usize,
    pub freq_mhz: u64,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Default)]
pub struct DiskInfo {
    pub mount: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub usage_percent: f32,
    pub fs_type: String,
}

#[derive(Debug, Clone, Default)]
pub struct BatteryInfo {
    pub percentage: u8,
    pub is_charging: bool,
    pub state: String,
}

#[derive(Debug, Clone, Default)]
pub struct DisplayInfo {
    pub resolution: String,
    pub refresh_rate: u32,
}

#[derive(Debug, Clone, Default)]
pub struct SystemInfo {
    pub username: String,
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub os_build: String,
    pub os_arch: String,
    pub host_model: String,
    pub kernel: String,
    pub uptime_seconds: u64,
    pub packages_count: usize,
    pub package_managers: String,
    pub shell: String,
    pub terminal: String,
    pub wm_de: String,
    pub cpu: CpuInfo,
    pub gpu: Vec<String>,
    pub memory: MemoryInfo,
    pub disks: Vec<DiskInfo>,
    pub battery: Option<BatteryInfo>,
    pub displays: Vec<DisplayInfo>,
    pub os_key: String,
}

impl SystemInfo {
    pub fn fetch() -> Self {
        #[cfg(target_os = "windows")]
        {
            windows::fetch_windows_info()
        }

        #[cfg(target_os = "linux")]
        {
            linux::fetch_linux_info()
        }

        #[cfg(target_os = "macos")]
        {
            macos::fetch_macos_info()
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            common::fetch_fallback_info()
        }
    }
}
