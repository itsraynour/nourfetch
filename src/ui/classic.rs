use super::{render_color_palette, Icons};
use crate::config::Config;
use crate::logos::Logo;
use crate::sys::SystemInfo;
use crate::theme::Theme;
use crate::utils::ansi::{bold, fg_rgb, gradient_text, visual_len};
use crate::utils::bar::{render_bar, BarStyle};
use crate::utils::format::{format_bytes, format_freq, format_uptime};

pub fn render(info: &SystemInfo, logo: &Logo, theme: &Theme, config: &Config, color_enabled: bool) {
    let icons = Icons::new(config.nerd_fonts);
    let bar_style = BarStyle::from_str(&config.bar_style);

    let mut info_lines = Vec::new();

    let user_styled = fg_rgb(&info.username, theme.primary, color_enabled);
    let at_styled = fg_rgb("@", theme.accent, color_enabled);
    let host_styled = fg_rgb(&info.hostname, theme.secondary, color_enabled);
    let title_full = format!("{}{}{}{}", icons.user(), user_styled, at_styled, host_styled);
    info_lines.push(title_full);

    let sep_len = info.username.len() + 1 + info.hostname.len() + if config.nerd_fonts { 2 } else { 0 };
    let sep_char = "─";
    let sep_str: String = sep_char.repeat(sep_len.max(16));
    info_lines.push(gradient_text(&sep_str, theme.gradient_start, theme.gradient_end, color_enabled));

    let format_kv = |icon: &str, label: &str, value: &str| -> String {
        let label_colored = fg_rgb(&format!("{}{}", icon, label), theme.label, color_enabled);
        let val_colored = fg_rgb(value, theme.text, color_enabled);
        format!("{}: {}", bold(&label_colored, color_enabled), val_colored)
    };

    let os_val = if !info.os_version.is_empty() {
        format!("{} ({}) [{}]", info.os_name, info.os_version, info.os_arch)
    } else {
        format!("{} [{}]", info.os_name, info.os_arch)
    };
    info_lines.push(format_kv(icons.os(), "OS", &os_val));

    if !info.host_model.is_empty() {
        info_lines.push(format_kv(icons.host(), "Host", &info.host_model));
    }

    if !info.kernel.is_empty() {
        info_lines.push(format_kv(icons.kernel(), "Kernel", &info.kernel));
    }

    if info.uptime_seconds > 0 {
        info_lines.push(format_kv(icons.uptime(), "Uptime", &format_uptime(info.uptime_seconds)));
    }

    if info.packages_count > 0 || !info.package_managers.is_empty() {
        let pkg_val = if !info.package_managers.is_empty() {
            info.package_managers.clone()
        } else {
            info.packages_count.to_string()
        };
        info_lines.push(format_kv(icons.packages(), "Packages", &pkg_val));
    }

    if !info.shell.is_empty() {
        info_lines.push(format_kv(icons.shell(), "Shell", &info.shell));
    }

    if !info.terminal.is_empty() {
        info_lines.push(format_kv(icons.terminal(), "Terminal", &info.terminal));
    }

    if !info.wm_de.is_empty() {
        info_lines.push(format_kv(icons.wm(), "WM/DE", &info.wm_de));
    }

    if !info.cpu.model.is_empty() {
        let mut cpu_val = info.cpu.model.clone();
        if info.cpu.threads > 0 {
            cpu_val = format!("{} ({} threads)", cpu_val, info.cpu.threads);
        }
        if info.cpu.freq_mhz > 0 {
            cpu_val = format!("{} @ {}", cpu_val, format_freq(info.cpu.freq_mhz));
        }
        info_lines.push(format_kv(icons.cpu(), "CPU", &cpu_val));
    }

    for gpu in &info.gpu {
        let mut gpu_val = gpu.name.clone();
        if gpu.total_vram_bytes > 0 {
            let total_str = format_bytes(gpu.total_vram_bytes);
            if gpu.used_vram_bytes > 0 {
                let used_str = format_bytes(gpu.used_vram_bytes);
                if let Some(pct) = gpu.usage_percent {
                    if config.show_bars {
                        let bar = render_bar(
                            pct,
                            config.bar_width,
                            bar_style,
                            theme.bar_fill,
                            theme.bar_empty,
                            color_enabled,
                        );
                        gpu_val = format!("{} [{} / {} {} ({:.1}%)]", gpu.name, used_str, total_str, bar, pct);
                    } else {
                        gpu_val = format!("{} [{} / {} ({:.1}%)]", gpu.name, used_str, total_str, pct);
                    }
                } else {
                    gpu_val = format!("{} [{} / {}]", gpu.name, used_str, total_str);
                }
            } else {
                gpu_val = format!("{} [{}]", gpu.name, total_str);
            }
        }
        info_lines.push(format_kv(icons.gpu(), "GPU", &gpu_val));
    }

    if info.memory.total_bytes > 0 {
        let used_str = format_bytes(info.memory.used_bytes);
        let total_str = format_bytes(info.memory.total_bytes);
        let mem_val = if config.show_bars {
            let bar = render_bar(
                info.memory.usage_percent,
                config.bar_width,
                bar_style,
                theme.bar_fill,
                theme.bar_empty,
                color_enabled,
            );
            format!("{} / {} {} ({:.1}%)", used_str, total_str, bar, info.memory.usage_percent)
        } else {
            format!("{} / {} ({:.1}%)", used_str, total_str, info.memory.usage_percent)
        };
        info_lines.push(format_kv(icons.memory(), "Memory", &mem_val));
    }

    for disk in &info.disks {
        if disk.total_bytes > 0 {
            let used_str = format_bytes(disk.used_bytes);
            let total_str = format_bytes(disk.total_bytes);
            let disk_val = if config.show_bars {
                let bar = render_bar(
                    disk.usage_percent,
                    config.bar_width,
                    bar_style,
                    theme.bar_fill,
                    theme.bar_empty,
                    color_enabled,
                );
                format!("{} / {} {} ({:.0}%)", used_str, total_str, bar, disk.usage_percent)
            } else {
                format!("{} / {} ({:.0}%)", used_str, total_str, disk.usage_percent)
            };
            let label = format!("Disk ({})", disk.mount);
            info_lines.push(format_kv(icons.disk(), &label, &disk_val));
        }
    }

    if let Some(bat) = &info.battery {
        let bat_val = if config.show_bars {
            let bar = render_bar(
                bat.percentage as f32,
                config.bar_width,
                bar_style,
                theme.accent,
                theme.bar_empty,
                color_enabled,
            );
            format!("{}% {} [{}]", bat.percentage, bar, bat.state)
        } else {
            format!("{}% [{}]", bat.percentage, bat.state)
        };
        info_lines.push(format_kv(icons.battery(), "Battery", &bat_val));
    }

    for disp in &info.displays {
        info_lines.push(format_kv(icons.display(), "Display", &disp.resolution));
    }

    if config.color_blocks {
        info_lines.push(String::new());
        for line in render_color_palette(color_enabled).lines() {
            info_lines.push(line.to_string());
        }
    }

    let mut colored_logo_lines = Vec::new();
    let logo_width = logo.lines.iter().map(|l| visual_len(l)).max().unwrap_or(0);

    for (i, line) in logo.lines.iter().enumerate() {
        let t = if logo.lines.len() > 1 {
            i as f32 / (logo.lines.len() - 1) as f32
        } else {
            0.0
        };
        let color = logo.primary_color.lerp(&logo.secondary_color, t);
        let styled = fg_rgb(line, color, color_enabled);
        colored_logo_lines.push((styled, visual_len(line)));
    }

    let max_lines = colored_logo_lines.len().max(info_lines.len());
    let margin = 4;

    println!();
    for i in 0..max_lines {
        let (left_str, left_len) = if i < colored_logo_lines.len() {
            (&colored_logo_lines[i].0[..], colored_logo_lines[i].1)
        } else {
            ("", 0)
        };

        let right_str = if i < info_lines.len() {
            &info_lines[i][..]
        } else {
            ""
        };

        let padding = " ".repeat((logo_width.saturating_sub(left_len)) + margin);
        println!("{}{}{}", left_str, padding, right_str);
    }
    println!();
}
