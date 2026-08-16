#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::env;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;

use super::common::{detect_shell, detect_terminal};
use super::{BatteryInfo, CpuInfo, DiskInfo, DisplayInfo, GpuInfo, MemoryInfo, SystemInfo};

#[repr(C)]
#[derive(Clone, Copy)]
struct GUID {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

const IID_IDXGIFactory1: GUID = GUID {
    data1: 0x770aae78,
    data2: 0xf26f,
    data3: 0x4dba,
    data4: [0xa8, 0x29, 0x25, 0x3c, 0x83, 0xd1, 0xb3, 0x87],
};

const IID_IDXGIAdapter3: GUID = GUID {
    data1: 0x645967a4,
    data2: 0x5869,
    data3: 0x4371,
    data4: [0x93, 0xe3, 0x50, 0x17, 0x0c, 0x9d, 0xa4, 0xa1],
};

#[repr(C)]
struct DXGI_ADAPTER_DESC1 {
    Description: [u16; 128],
    VendorId: u32,
    DeviceId: u32,
    SubSysId: u32,
    Revision: u32,
    DedicatedVideoMemory: usize,
    DedicatedSystemMemory: usize,
    SharedSystemMemory: usize,
    AdapterLuid: u64,
    Flags: u32,
}

#[repr(C)]
struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    Budget: u64,
    CurrentUsage: u64,
    AvailableForReservation: u64,
    CurrentReservation: u64,
}

#[repr(C)]
struct SYSTEM_INFO {
    wProcessorArchitecture: u16,
    wReserved: u16,
    dwPageSize: u32,
    lpMinimumApplicationAddress: *mut std::ffi::c_void,
    lpMaximumApplicationAddress: *mut std::ffi::c_void,
    dwActiveProcessorMask: usize,
    dwNumberOfProcessors: u32,
    dwProcessorType: u32,
    dwAllocationGranularity: u32,
    wProcessorLevel: u16,
    wProcessorRevision: u16,
}

#[repr(C)]
struct MEMORYSTATUSEX {
    dwLength: u32,
    dwMemoryLoad: u32,
    ullTotalPhys: u64,
    ullAvailPhys: u64,
    ullTotalPageFile: u64,
    ullAvailPageFile: u64,
    ullTotalVirtual: u64,
    ullAvailVirtual: u64,
    ullAvailExtendedVirtual: u64,
}

#[repr(C)]
struct SYSTEM_POWER_STATUS {
    ACLineStatus: u8,
    BatteryFlag: u8,
    BatteryLifePercent: u8,
    SystemStatusFlag: u8,
    BatteryLifeTime: u32,
    BatteryFullLifeTime: u32,
}

type HKEY = *mut std::ffi::c_void;

const HKEY_LOCAL_MACHINE: HKEY = 0x80000002_u64 as HKEY;
const KEY_READ: u32 = 0x20019;
const RRF_RT_REG_SZ: u32 = 0x00000002;
const RRF_RT_REG_DWORD: u32 = 0x00000010;

#[link(name = "kernel32")]
extern "system" {
    fn GetComputerNameW(lpBuffer: *mut u16, nSize: *mut u32) -> i32;
    fn GetSystemInfo(lpSystemInfo: *mut SYSTEM_INFO);
    fn GlobalMemoryStatusEx(lpBuffer: *mut MEMORYSTATUSEX) -> i32;
    fn GetTickCount64() -> u64;
    fn GetLogicalDrives() -> u32;
    fn GetDiskFreeSpaceExW(
        lpDirectoryName: *const u16,
        lpFreeBytesAvailableToCaller: *mut u64,
        lpTotalNumberOfBytes: *mut u64,
        lpTotalNumberOfFreeBytes: *mut u64,
    ) -> i32;
    fn GetSystemPowerStatus(lpSystemPowerStatus: *mut SYSTEM_POWER_STATUS) -> i32;
    fn LoadLibraryW(lpLibFileName: *const u16) -> *mut std::ffi::c_void;
    fn FreeLibrary(hLibModule: *mut std::ffi::c_void) -> i32;
    fn GetProcAddress(hModule: *mut std::ffi::c_void, lpProcName: *const u8) -> *const std::ffi::c_void;
}

#[link(name = "advapi32")]
extern "system" {
    fn RegOpenKeyExW(
        hKey: HKEY,
        lpSubKey: *const u16,
        ulOptions: u32,
        samDesired: u32,
        phkResult: *mut HKEY,
    ) -> i32;
    fn RegQueryValueExW(
        hKey: HKEY,
        lpValueName: *const u16,
        lpReserved: *mut u32,
        lpType: *mut u32,
        lpData: *mut u8,
        lpcbData: *mut u32,
    ) -> i32;
    fn RegCloseKey(hKey: HKEY) -> i32;
}

#[link(name = "user32")]
extern "system" {
    fn GetSystemMetrics(nIndex: i32) -> i32;
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn read_reg_string(hkey: HKEY, subkey: &str, value_name: &str) -> Option<String> {
    unsafe {
        let mut key: HKEY = ptr::null_mut();
        let subkey_w = to_wide(subkey);
        if RegOpenKeyExW(hkey, subkey_w.as_ptr(), 0, KEY_READ, &mut key) != 0 {
            return None;
        }

        let value_w = to_wide(value_name);
        let mut buf_len: u32 = 512;
        let mut buf: Vec<u16> = vec![0; (buf_len / 2) as usize];
        let mut val_type: u32 = 0;

        let status = RegQueryValueExW(
            key,
            value_w.as_ptr(),
            ptr::null_mut(),
            &mut val_type,
            buf.as_mut_ptr() as *mut u8,
            &mut buf_len,
        );

        RegCloseKey(key);

        if status == 0 {
            let actual_len = (buf_len / 2) as usize;
            if actual_len > 0 {
                let trimmed_len = if buf[actual_len - 1] == 0 {
                    actual_len - 1
                } else {
                    actual_len
                };
                return String::from_utf16(&buf[..trimmed_len]).ok();
            }
        }
        None
    }
}

fn read_reg_dword(hkey: HKEY, subkey: &str, value_name: &str) -> Option<u32> {
    unsafe {
        let mut key: HKEY = ptr::null_mut();
        let subkey_w = to_wide(subkey);
        if RegOpenKeyExW(hkey, subkey_w.as_ptr(), 0, KEY_READ, &mut key) != 0 {
            return None;
        }

        let value_w = to_wide(value_name);
        let mut val: u32 = 0;
        let mut val_len: u32 = std::mem::size_of::<u32>() as u32;
        let mut val_type: u32 = 0;

        let status = RegQueryValueExW(
            key,
            value_w.as_ptr(),
            ptr::null_mut(),
            &mut val_type,
            &mut val as *mut u32 as *mut u8,
            &mut val_len,
        );

        RegCloseKey(key);

        if status == 0 {
            Some(val)
        } else {
            None
        }
    }
}

fn read_reg_qword(hkey: HKEY, subkey: &str, value_name: &str) -> Option<u64> {
    unsafe {
        let mut key: HKEY = ptr::null_mut();
        let subkey_w = to_wide(subkey);
        if RegOpenKeyExW(hkey, subkey_w.as_ptr(), 0, KEY_READ, &mut key) != 0 {
            return None;
        }

        let value_w = to_wide(value_name);
        let mut val: u64 = 0;
        let mut val_len: u32 = std::mem::size_of::<u64>() as u32;
        let mut val_type: u32 = 0;

        let status = RegQueryValueExW(
            key,
            value_w.as_ptr(),
            ptr::null_mut(),
            &mut val_type,
            &mut val as *mut u64 as *mut u8,
            &mut val_len,
        );

        RegCloseKey(key);

        if status == 0 {
            Some(val)
        } else {
            None
        }
    }
}

pub fn fetch_windows_info() -> SystemInfo {
    let mut info = SystemInfo::default();

    info.username = env::var("USERNAME").unwrap_or_else(|_| "user".to_string());
    unsafe {
        let mut buf = [0u16; 256];
        let mut size = buf.len() as u32;
        if GetComputerNameW(buf.as_mut_ptr(), &mut size) != 0 {
            info.hostname = OsString::from_wide(&buf[..size as usize])
                .to_string_lossy()
                .to_string();
        } else {
            info.hostname = env::var("COMPUTERNAME").unwrap_or_else(|_| "DESKTOP".to_string());
        }
    }

    let product_name = read_reg_string(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
        "ProductName",
    )
    .unwrap_or_else(|| "Windows 11".to_string());

    let display_version = read_reg_string(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
        "DisplayVersion",
    )
    .or_else(|| {
        read_reg_string(
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
            "ReleaseId",
        )
    })
    .unwrap_or_default();

    let current_build = read_reg_string(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
        "CurrentBuild",
    )
    .unwrap_or_default();

    let ubr = read_reg_dword(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
        "UBR",
    );

    let build_num: u32 = current_build.parse().unwrap_or(0);
    let is_win11 = build_num >= 22000;

    let fixed_os_name = if is_win11 && product_name.contains("Windows 10") {
        product_name.replace("Windows 10", "Windows 11")
    } else {
        product_name
    };

    info.os_name = fixed_os_name;
    info.os_version = display_version;
    info.os_build = if let Some(rev) = ubr {
        format!("{}.{}", current_build, rev)
    } else {
        current_build
    };
    info.os_arch = std::env::consts::ARCH.to_string();
    info.kernel = format!("Windows NT {}", info.os_build);
    info.os_key = if is_win11 { "windows11" } else { "windows" }.to_string();

    let manufacturer = read_reg_string(
        HKEY_LOCAL_MACHINE,
        r"HARDWARE\DESCRIPTION\System\BIOS",
        "SystemManufacturer",
    )
    .unwrap_or_default();

    let product_model = read_reg_string(
        HKEY_LOCAL_MACHINE,
        r"HARDWARE\DESCRIPTION\System\BIOS",
        "SystemProductName",
    )
    .unwrap_or_default();

    if !product_model.is_empty() && !product_model.eq_ignore_ascii_case("System Product Name") {
        if !manufacturer.is_empty() && !product_model.contains(&manufacturer) {
            info.host_model = format!("{} {}", manufacturer.trim(), product_model.trim());
        } else {
            info.host_model = product_model.trim().to_string();
        }
    } else if !manufacturer.is_empty() {
        info.host_model = manufacturer.trim().to_string();
    } else {
        info.host_model = "PC (Desktop / Laptop)".to_string();
    }

    unsafe {
        let uptime_ms = GetTickCount64();
        info.uptime_seconds = uptime_ms / 1000;
    }

    info.shell = detect_shell();
    info.terminal = detect_terminal();
    info.wm_de = "Desktop Window Manager (DWM)".to_string();

    let mut cpu = CpuInfo::default();
    cpu.model = read_reg_string(
        HKEY_LOCAL_MACHINE,
        r"HARDWARE\DESCRIPTION\System\CentralProcessor\0",
        "ProcessorNameString",
    )
    .map(|s| s.trim().to_string())
    .unwrap_or_else(|| "Unknown Processor".to_string());

    if let Some(mhz) = read_reg_dword(
        HKEY_LOCAL_MACHINE,
        r"HARDWARE\DESCRIPTION\System\CentralProcessor\0",
        "~MHz",
    ) {
        cpu.freq_mhz = mhz as u64;
    }

    unsafe {
        let mut sys_info: SYSTEM_INFO = std::mem::zeroed();
        GetSystemInfo(&mut sys_info);
        cpu.threads = sys_info.dwNumberOfProcessors as usize;
        cpu.cores = cpu.threads;
    }
    info.cpu = cpu;

    info.gpu = detect_windows_gpus();

    unsafe {
        let mut mem_status: MEMORYSTATUSEX = std::mem::zeroed();
        mem_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        if GlobalMemoryStatusEx(&mut mem_status) != 0 {
            let total = mem_status.ullTotalPhys;
            let free = mem_status.ullAvailPhys;
            let used = total.saturating_sub(free);
            let pct = if total > 0 {
                (used as f32 / total as f32) * 100.0
            } else {
                0.0
            };
            info.memory = MemoryInfo {
                total_bytes: total,
                used_bytes: used,
                free_bytes: free,
                usage_percent: pct,
            };
        }
    }

    info.disks = detect_windows_disks();

    unsafe {
        let mut power: SYSTEM_POWER_STATUS = std::mem::zeroed();
        if GetSystemPowerStatus(&mut power) != 0 && power.BatteryFlag != 128 && power.BatteryFlag != 255 {
            let is_charging = (power.BatteryFlag & 8) != 0 || power.ACLineStatus == 1;
            let pct = if power.BatteryLifePercent <= 100 {
                power.BatteryLifePercent
            } else {
                100
            };
            info.battery = Some(BatteryInfo {
                percentage: pct,
                is_charging,
                state: if is_charging {
                    "Charging".to_string()
                } else {
                    "Discharging".to_string()
                },
            });
        }
    }

    unsafe {
        let w = GetSystemMetrics(0);
        let h = GetSystemMetrics(1);
        if w > 0 && h > 0 {
            info.displays.push(DisplayInfo {
                resolution: format!("{}x{}", w, h),
                refresh_rate: 60,
            });
        }
    }

    let (pkg_count, pkg_names) = count_windows_packages();
    info.packages_count = pkg_count;
    info.package_managers = pkg_names;

    info
}

fn detect_windows_gpus() -> Vec<GpuInfo> {
    let mut gpus: Vec<GpuInfo> = Vec::new();

    
    unsafe {
        let dxgi_lib = to_wide("dxgi.dll");
        let hmodule = LoadLibraryW(dxgi_lib.as_ptr());
        if !hmodule.is_null() {
            let create_factory_name = b"CreateDXGIFactory1\0";
            let create_factory_ptr = GetProcAddress(hmodule, create_factory_name.as_ptr());
            if !create_factory_ptr.is_null() {
                type CreateDXGIFactory1Fn =
                    unsafe extern "system" fn(*const GUID, *mut *mut std::ffi::c_void) -> i32;
                let create_factory: CreateDXGIFactory1Fn = std::mem::transmute(create_factory_ptr);

                let mut factory_ptr: *mut std::ffi::c_void = ptr::null_mut();
                if create_factory(&IID_IDXGIFactory1, &mut factory_ptr) == 0
                    && !factory_ptr.is_null()
                {
                    let factory_vtable = *(factory_ptr as *mut *mut usize);
                    type EnumAdapters1Fn = unsafe extern "system" fn(
                        *mut std::ffi::c_void,
                        u32,
                        *mut *mut std::ffi::c_void,
                    ) -> i32;
                    let enum_adapters1: EnumAdapters1Fn =
                        std::mem::transmute(*factory_vtable.add(12));

                    type ReleaseFn = unsafe extern "system" fn(*mut std::ffi::c_void) -> u32;
                    let release_factory: ReleaseFn =
                        std::mem::transmute(*factory_vtable.add(2));

                    let mut i = 0u32;
                    loop {
                        let mut adapter_ptr: *mut std::ffi::c_void = ptr::null_mut();
                        if enum_adapters1(factory_ptr, i, &mut adapter_ptr) != 0
                            || adapter_ptr.is_null()
                        {
                            break;
                        }

                        let adapter_vtable = *(adapter_ptr as *mut *mut usize);
                        type QueryInterfaceFn = unsafe extern "system" fn(
                            *mut std::ffi::c_void,
                            *const GUID,
                            *mut *mut std::ffi::c_void,
                        ) -> i32;
                        let query_interface: QueryInterfaceFn =
                            std::mem::transmute(*adapter_vtable.add(0));
                        let release_adapter: ReleaseFn =
                            std::mem::transmute(*adapter_vtable.add(2));

                        type GetDesc1Fn = unsafe extern "system" fn(
                            *mut std::ffi::c_void,
                            *mut DXGI_ADAPTER_DESC1,
                        ) -> i32;
                        let get_desc1: GetDesc1Fn =
                            std::mem::transmute(*adapter_vtable.add(11));

                        let mut desc: DXGI_ADAPTER_DESC1 = std::mem::zeroed();
                        if get_desc1(adapter_ptr, &mut desc) == 0 {
                            let is_software = (desc.Flags & 2) != 0;
                            if !is_software {
                                let name_len = desc
                                    .Description
                                    .iter()
                                    .position(|&c| c == 0)
                                    .unwrap_or(desc.Description.len());
                                let name = String::from_utf16_lossy(&desc.Description[..name_len])
                                    .trim()
                                    .to_string();

                                if !name.is_empty()
                                    && !name.contains("Microsoft Basic Render")
                                    && !name.contains("Remote Desktop")
                                {
                                    let mut gpu_info = GpuInfo::default();
                                    gpu_info.name = name;
                                    gpu_info.total_vram_bytes = desc.DedicatedVideoMemory as u64;
                                    gpu_info.vendor = match desc.VendorId {
                                        0x10DE => "NVIDIA".to_string(),
                                        0x1002 => "AMD".to_string(),
                                        0x8086 => "Intel".to_string(),
                                        0x1414 => "Microsoft".to_string(),
                                        _ => String::new(),
                                    };

                                    let mut adapter3_ptr: *mut std::ffi::c_void = ptr::null_mut();
                                    if query_interface(
                                        adapter_ptr,
                                        &IID_IDXGIAdapter3,
                                        &mut adapter3_ptr,
                                    ) == 0
                                        && !adapter3_ptr.is_null()
                                    {
                                        let adapter3_vtable = *(adapter3_ptr as *mut *mut usize);
                                        type QueryVideoMemoryInfoFn = unsafe extern "system" fn(
                                            *mut std::ffi::c_void,
                                            u32,
                                            u32,
                                            *mut DXGI_QUERY_VIDEO_MEMORY_INFO,
                                        )
                                            -> i32;
                                        let query_vram: QueryVideoMemoryInfoFn =
                                            std::mem::transmute(*adapter3_vtable.add(14));
                                        let release_adapter3: ReleaseFn =
                                            std::mem::transmute(*adapter3_vtable.add(2));

                                        let mut vram_info: DXGI_QUERY_VIDEO_MEMORY_INFO =
                                            std::mem::zeroed();
                                        if query_vram(adapter3_ptr, 0, 0, &mut vram_info) == 0 {
                                            gpu_info.used_vram_bytes = vram_info.CurrentUsage;
                                            if gpu_info.total_vram_bytes == 0 && vram_info.Budget > 0 {
                                                gpu_info.total_vram_bytes = vram_info.Budget;
                                            }
                                            if gpu_info.total_vram_bytes > 0 {
                                                gpu_info.free_vram_bytes = gpu_info
                                                    .total_vram_bytes
                                                    .saturating_sub(gpu_info.used_vram_bytes);
                                                let pct = (gpu_info.used_vram_bytes as f32
                                                    / gpu_info.total_vram_bytes as f32)
                                                    * 100.0;
                                                gpu_info.usage_percent = Some(pct.min(100.0));
                                            }
                                        }
                                        release_adapter3(adapter3_ptr);
                                    }

                                    gpus.push(gpu_info);
                                }
                            }
                        }

                        release_adapter(adapter_ptr);
                        i += 1;
                    }
                    release_factory(factory_ptr);
                }
            }
            FreeLibrary(hmodule);
        }
    }

    for i in 0..16 {
        let subkey = format!(
            r"SYSTEM\CurrentControlSet\Control\Class\{{4d36e968-e325-11ce-bfc1-08002be10318}}\{:04}",
            i
        );
        if let Some(desc) = read_reg_string(HKEY_LOCAL_MACHINE, &subkey, "DriverDesc") {
            let desc_trimmed = desc.trim().to_string();
            if !desc_trimmed.is_empty()
                && !desc_trimmed.contains("Microsoft Remote Display")
                && !desc_trimmed.contains("Microsoft Basic Render")
            {
                let driver_ver = read_reg_string(HKEY_LOCAL_MACHINE, &subkey, "DriverVersion")
                    .unwrap_or_default();
                let vram_qw = read_reg_qword(
                    HKEY_LOCAL_MACHINE,
                    &subkey,
                    "HardwareInformation.qwMemorySize",
                )
                .or_else(|| {
                    read_reg_dword(
                        HKEY_LOCAL_MACHINE,
                        &subkey,
                        "HardwareInformation.MemorySize",
                    )
                    .map(|d| d as u64)
                });

                if let Some(existing) = gpus.iter_mut().find(|g| {
                    g.name == desc_trimmed
                        || desc_trimmed.contains(&g.name)
                        || g.name.contains(&desc_trimmed)
                }) {
                    if existing.driver_version.is_empty() && !driver_ver.is_empty() {
                        existing.driver_version = driver_ver;
                    }
                    if existing.total_vram_bytes == 0 {
                        if let Some(vram) = vram_qw {
                            existing.total_vram_bytes = vram;
                        }
                    }
                } else if gpus.is_empty() {
                    let mut gpu = GpuInfo::default();
                    gpu.name = desc_trimmed;
                    gpu.driver_version = driver_ver;
                    if let Some(vram) = vram_qw {
                        gpu.total_vram_bytes = vram;
                    }
                    gpus.push(gpu);
                }
            }
        }
    }

    gpus
}

fn detect_windows_disks() -> Vec<DiskInfo> {
    let mut disks = Vec::new();
    unsafe {
        let drives_mask = GetLogicalDrives();
        for i in 0..26 {
            if (drives_mask & (1 << i)) != 0 {
                let drive_letter = (b'A' + i) as char;
                let root_path = format!("{}:\\", drive_letter);
                let root_w = to_wide(&root_path);

                let mut free_avail: u64 = 0;
                let mut total: u64 = 0;
                let mut total_free: u64 = 0;

                if GetDiskFreeSpaceExW(
                    root_w.as_ptr(),
                    &mut free_avail,
                    &mut total,
                    &mut total_free,
                ) != 0
                {
                    if total > 0 {
                        let used = total.saturating_sub(total_free);
                        let pct = (used as f32 / total as f32) * 100.0;
                        disks.push(DiskInfo {
                            mount: format!("{}:", drive_letter),
                            total_bytes: total,
                            used_bytes: used,
                            free_bytes: total_free,
                            usage_percent: pct,
                            fs_type: "NTFS".to_string(),
                        });
                    }
                }
            }
        }
    }
    disks
}

fn count_windows_packages() -> (usize, String) {
    let mut count = 0;
    let mut managers = Vec::new();

    if let Ok(userprofile) = env::var("USERPROFILE") {
        let scoop_apps = PathBuf::from(&userprofile).join("scoop").join("apps");
        if scoop_apps.exists() {
            if let Ok(entries) = std::fs::read_dir(&scoop_apps) {
                let scoop_count = entries.filter_map(Result::ok).count();
                if scoop_count > 0 {
                    count += scoop_count;
                    managers.push(format!("scoop ({})", scoop_count));
                }
            }
        }
    }

    let choco_lib = PathBuf::from(r"C:\ProgramData\chocolatey\lib");
    if choco_lib.exists() {
        if let Ok(entries) = std::fs::read_dir(&choco_lib) {
            let choco_count = entries.filter_map(Result::ok).count();
            if choco_count > 0 {
                count += choco_count;
                managers.push(format!("choco ({})", choco_count));
            }
        }
    }

    if let Ok(userprofile) = env::var("USERPROFILE") {
        let cargo_bin = PathBuf::from(&userprofile).join(".cargo").join("bin");
        if cargo_bin.exists() {
            if let Ok(entries) = std::fs::read_dir(&cargo_bin) {
                let cargo_count = entries.filter_map(Result::ok).count();
                if cargo_count > 0 {
                    count += cargo_count;
                    managers.push(format!("cargo ({})", cargo_count));
                }
            }
        }
    }

    if let Ok(localappdata) = env::var("LOCALAPPDATA") {
        let winget_dir = PathBuf::from(&localappdata)
            .join("Microsoft")
            .join("WinGet")
            .join("Packages");
        if winget_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&winget_dir) {
                let winget_count = entries.filter_map(Result::ok).count();
                if winget_count > 0 {
                    count += winget_count;
                    managers.push(format!("winget ({})", winget_count));
                }
            }
        }
    }

    (count, managers.join(", "))
}
