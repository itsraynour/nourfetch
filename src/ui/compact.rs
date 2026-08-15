use super::Icons;
use crate::config::Config;
use crate::sys::SystemInfo;
use crate::theme::Theme;
use crate::utils::ansi::{bold, fg_rgb};
use crate::utils::format::{format_bytes, format_uptime};

pub fn render(info: &SystemInfo, theme: &Theme, config: &Config, color_enabled: bool) {
    let icons = Icons::new(config.nerd_fonts);

    let sep = fg_rgb(" │ ", theme.border, color_enabled);

    let mut badges = Vec::new();

    let user_host = format!(
        "{}{}{}",
        fg_rgb(&info.username, theme.primary, color_enabled),
        fg_rgb("@", theme.accent, color_enabled),
        fg_rgb(&info.hostname, theme.secondary, color_enabled)
    );
    badges.push(format!("{}{}", icons.user(), bold(&user_host, color_enabled)));

    badges.push(format!(
        "{}{}",
        icons.os(),
        fg_rgb(&info.os_name, theme.label, color_enabled)
    ));

    if !info.kernel.is_empty() {
        badges.push(format!(
            "{}{}",
            icons.kernel(),
            fg_rgb(&info.kernel, theme.text, color_enabled)
        ));
    }

    if info.uptime_seconds > 0 {
        badges.push(format!(
            "{}{}",
            icons.uptime(),
            fg_rgb(&format_uptime(info.uptime_seconds), theme.accent, color_enabled)
        ));
    }

    if info.memory.total_bytes > 0 {
        let mem_str = format!(
            "{}/{} ({:.0}%)",
            format_bytes(info.memory.used_bytes),
            format_bytes(info.memory.total_bytes),
            info.memory.usage_percent
        );
        badges.push(format!(
            "{}{}",
            icons.memory(),
            fg_rgb(&mem_str, theme.label, color_enabled)
        ));
    }

    if !info.shell.is_empty() {
        badges.push(format!(
            "{}{}",
            icons.shell(),
            fg_rgb(&info.shell, theme.text, color_enabled)
        ));
    }

    println!();
    println!("{}", badges.join(&sep));
    println!();
}
