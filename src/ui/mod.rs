pub mod classic;
pub mod compact;
pub mod json;
pub mod modern;

use crate::utils::ansi::Rgb;

pub struct Icons {
    pub enabled: bool,
}

impl Icons {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn user(&self) -> &'static str {
        if self.enabled { " " } else { "" }
    }
    pub fn host(&self) -> &'static str {
        if self.enabled { "󰌢 " } else { "" }
    }
    pub fn os(&self) -> &'static str {
        if self.enabled { "󰣇 " } else { "" }
    }
    pub fn kernel(&self) -> &'static str {
        if self.enabled { "󰌽 " } else { "" }
    }
    pub fn uptime(&self) -> &'static str {
        if self.enabled { "󰅐 " } else { "" }
    }
    pub fn packages(&self) -> &'static str {
        if self.enabled { "󰏖 " } else { "" }
    }
    pub fn shell(&self) -> &'static str {
        if self.enabled { "󰞷 " } else { "" }
    }
    pub fn terminal(&self) -> &'static str {
        if self.enabled { "󰆍 " } else { "" }
    }
    pub fn wm(&self) -> &'static str {
        if self.enabled { "󰨇 " } else { "" }
    }
    pub fn cpu(&self) -> &'static str {
        if self.enabled { "󰍛 " } else { "" }
    }
    pub fn gpu(&self) -> &'static str {
        if self.enabled { "󰢮 " } else { "" }
    }
    pub fn memory(&self) -> &'static str {
        if self.enabled { "󰘚 " } else { "" }
    }
    pub fn disk(&self) -> &'static str {
        if self.enabled { "󰋊 " } else { "" }
    }
    pub fn battery(&self) -> &'static str {
        if self.enabled { "󰂀 " } else { "" }
    }
    pub fn display(&self) -> &'static str {
        if self.enabled { "󰍹 " } else { "" }
    }
}

pub fn render_color_palette(color_enabled: bool) -> String {
    if !color_enabled {
        return String::new();
    }
    let colors = [
        Rgb::new(0, 0, 0),
        Rgb::new(239, 68, 68),
        Rgb::new(34, 197, 94),
        Rgb::new(234, 179, 8),
        Rgb::new(59, 130, 246),
        Rgb::new(168, 85, 247),
        Rgb::new(6, 182, 212),
        Rgb::new(243, 244, 246),
    ];

    let mut row1 = String::new();
    let mut row2 = String::new();

    for &c in &colors {
        row1.push_str(&format!("{}███\x1b[0m", c.to_fg_ansi()));
        let bright = Rgb::new(
            c.r.saturating_add(60),
            c.g.saturating_add(60),
            c.b.saturating_add(60),
        );
        row2.push_str(&format!("{}███\x1b[0m", bright.to_fg_ansi()));
    }

    format!("{}\n{}", row1, row2)
}
