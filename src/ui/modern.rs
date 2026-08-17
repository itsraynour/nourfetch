use super::Icons;
use crate::config::Config;
use crate::logos::Logo;
use crate::sys::SystemInfo;
use crate::theme::Theme;
use crate::utils::ansi::{bold, fg_rgb, visual_len};
use crate::utils::bar::{render_bar, BarStyle};
use crate::utils::format::{format_bytes, format_freq, format_uptime};

enum ModernRow {
    Section(String),
    Item {
        icon: &'static str,
        label: String,
        value: String,
    },
}

pub fn render(info: &SystemInfo, _logo: &Logo, theme: &Theme, config: &Config, color_enabled: bool) {
    let icons = Icons::new(config.nerd_fonts);
    let bar_style = BarStyle::from_str(&config.bar_style);

    let mut rows: Vec<ModernRow> = Vec::new();

    rows.push(ModernRow::Section("SYSTEM".to_string()));
    let os_val = if !info.os_version.is_empty() {
        format!("{} ({}) [{}]", info.os_name, info.os_version, info.os_arch)
    } else {
        format!("{} [{}]", info.os_name, info.os_arch)
    };
    rows.push(ModernRow::Item {
        icon: icons.os(),
        label: "OS".to_string(),
        value: os_val,
    });
    if !info.host_model.is_empty() {
        rows.push(ModernRow::Item {
            icon: icons.host(),
            label: "Host".to_string(),
            value: info.host_model.clone(),
        });
    }
    if !info.kernel.is_empty() {
        rows.push(ModernRow::Item {
            icon: icons.kernel(),
            label: "Kernel".to_string(),
            value: info.kernel.clone(),
        });
    }
    if info.uptime_seconds > 0 {
        rows.push(ModernRow::Item {
            icon: icons.uptime(),
            label: "Uptime".to_string(),
            value: format_uptime(info.uptime_seconds),
        });
    }
    if !info.shell.is_empty() {
        rows.push(ModernRow::Item {
            icon: icons.shell(),
            label: "Shell".to_string(),
            value: info.shell.clone(),
        });
    }
    if !info.terminal.is_empty() {
        rows.push(ModernRow::Item {
            icon: icons.terminal(),
            label: "Terminal".to_string(),
            value: info.terminal.clone(),
        });
    }
    if info.packages_count > 0 || !info.package_managers.is_empty() {
        let pkg_str = if !info.package_managers.is_empty() {
            info.package_managers.clone()
        } else {
            info.packages_count.to_string()
        };
        rows.push(ModernRow::Item {
            icon: icons.packages(),
            label: "Packages".to_string(),
            value: pkg_str,
        });
    }

    rows.push(ModernRow::Section("HARDWARE".to_string()));
    if !info.cpu.model.is_empty() {
        let mut cpu_val = info.cpu.model.clone();
        if info.cpu.threads > 0 {
            cpu_val = format!("{} ({}T)", cpu_val, info.cpu.threads);
        }
        if info.cpu.freq_mhz > 0 {
            cpu_val = format!("{} @ {}", cpu_val, format_freq(info.cpu.freq_mhz));
        }
        rows.push(ModernRow::Item {
            icon: icons.cpu(),
            label: "CPU".to_string(),
            value: cpu_val,
        });
    }
    for gpu in &info.gpu {
        let mut gpu_val = gpu.name.clone();
        if gpu.total_vram_bytes > 0 {
            let total_str = format_bytes(gpu.total_vram_bytes);
            if gpu.used_vram_bytes > 0 {
                let used_str = format_bytes(gpu.used_vram_bytes);
                gpu_val = format!("{} ({} / {})", gpu.name, used_str, total_str);
            } else {
                gpu_val = format!("{} ({})", gpu.name, total_str);
            }
        }
        rows.push(ModernRow::Item {
            icon: icons.gpu(),
            label: "GPU".to_string(),
            value: gpu_val,
        });
    }
    for disp in &info.displays {
        rows.push(ModernRow::Item {
            icon: icons.display(),
            label: "Display".to_string(),
            value: disp.resolution.clone(),
        });
    }

    rows.push(ModernRow::Section("RESOURCES".to_string()));
    if info.memory.total_bytes > 0 {
        let used_str = format_bytes(info.memory.used_bytes);
        let total_str = format_bytes(info.memory.total_bytes);
        let bar = render_bar(
            info.memory.usage_percent,
            10,
            bar_style,
            theme.bar_fill,
            theme.bar_empty,
            color_enabled,
        );
        let mem_val = format!("{} / {} {} ({:.1}%)", used_str, total_str, bar, info.memory.usage_percent);
        rows.push(ModernRow::Item {
            icon: icons.memory(),
            label: "RAM".to_string(),
            value: mem_val,
        });
    }

    for (idx, gpu) in info.gpu.iter().enumerate() {
        if gpu.total_vram_bytes > 0 && gpu.used_vram_bytes > 0 {
            let used_str = format_bytes(gpu.used_vram_bytes);
            let total_str = format_bytes(gpu.total_vram_bytes);
            let pct = gpu.usage_percent.unwrap_or_else(|| {
                (gpu.used_vram_bytes as f32 / gpu.total_vram_bytes as f32) * 100.0
            });
            let bar = render_bar(
                pct,
                10,
                bar_style,
                theme.bar_fill,
                theme.bar_empty,
                color_enabled,
            );
            let vram_label = if info.gpu.len() > 1 {
                format!("VRAM {}", idx + 1)
            } else {
                "VRAM".to_string()
            };
            let vram_val = format!("{} / {} {} ({:.1}%)", used_str, total_str, bar, pct);
            rows.push(ModernRow::Item {
                icon: icons.gpu(),
                label: vram_label,
                value: vram_val,
            });
        }
    }

    for disk in &info.disks {
        if disk.total_bytes > 0 {
            let used_str = format_bytes(disk.used_bytes);
            let total_str = format_bytes(disk.total_bytes);
            let bar = render_bar(
                disk.usage_percent,
                10,
                bar_style,
                theme.bar_fill,
                theme.bar_empty,
                color_enabled,
            );
            let disk_val = format!("{} / {} {} ({:.0}%)", used_str, total_str, bar, disk.usage_percent);
            rows.push(ModernRow::Item {
                icon: icons.disk(),
                label: format!("Disk ({})", disk.mount),
                value: disk_val,
            });
        }
    }

    if let Some(bat) = &info.battery {
        let bar = render_bar(
            bat.percentage as f32,
            10,
            bar_style,
            theme.accent,
            theme.bar_empty,
            color_enabled,
        );
        let bat_val = format!("{}% {} [{}]", bat.percentage, bar, bat.state);
        rows.push(ModernRow::Item {
            icon: icons.battery(),
            label: "Battery".to_string(),
            value: bat_val,
        });
    }

    let title_text = format!(" {}@{} ", info.username, info.hostname);
    let mut max_content_width = visual_len(&title_text) + 6;

    for row in &rows {
        match row {
            ModernRow::Section(title) => {
                let sec_len = visual_len(&format!(" ◈ {} ", title)) + 4;
                if sec_len > max_content_width {
                    max_content_width = sec_len;
                }
            }
            ModernRow::Item { icon, label, value } => {
                let row_len = visual_len(&format!(" {}{}: {}", icon, label, value));
                if row_len > max_content_width {
                    max_content_width = row_len;
                }
            }
        }
    }

    let width = max_content_width.max(64) + 2;

    let border_fg = |s: &str| fg_rgb(s, theme.border, color_enabled);
    let title_styled = bold(&fg_rgb(&title_text, theme.primary, color_enabled), color_enabled);

    let remaining_border = width.saturating_sub(visual_len(&title_text) + 3);
    let top_border = format!(
        "╭──{}{}{}╮",
        title_styled,
        border_fg("─"),
        border_fg(&"─".repeat(remaining_border))
    );
    println!();
    println!("{}", top_border);

    for row in rows {
        match row {
            ModernRow::Section(title) => {
                let header = format!(" ◈ {} ", title);
                let header_styled = bold(&fg_rgb(&header, theme.accent, color_enabled), color_enabled);
                let border_len = width.saturating_sub(visual_len(&header) + 2);
                let sep = border_fg(&"─".repeat(border_len));
                println!("{} {}{}{}", border_fg("├"), header_styled, sep, border_fg("┤"));
            }
            ModernRow::Item { icon, label, value } => {
                let label_str = format!(" {}{}", icon, label);
                let left = format!("{}:", bold(&fg_rgb(&label_str, theme.label, color_enabled), color_enabled));
                let right = fg_rgb(&value, theme.text, color_enabled);
                let content = format!("{} {}", left, right);
                let v_len = visual_len(&content);
                let pad = " ".repeat(width.saturating_sub(v_len + 1));
                println!("{} {}{}{}", border_fg("│"), content, pad, border_fg("│"));
            }
        }
    }

    let bot_border = format!("╰{}╯", border_fg(&"─".repeat(width)));
    println!("{}", bot_border);
    println!();
}
