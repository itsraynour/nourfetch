use super::ansi::{fg_rgb, Rgb};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarStyle {
    Smooth,  // █ ░
    Block,   // ■ □
    Circle,  // ● ○
    Rounded, // ━ ╺ ─
    Ascii,   // # -
}

impl BarStyle {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "block" => Self::Block,
            "circle" => Self::Circle,
            "rounded" => Self::Rounded,
            "ascii" => Self::Ascii,
            _ => Self::Smooth,
        }
    }
}

pub fn render_bar(
    percentage: f32,
    width: usize,
    style: BarStyle,
    filled_color: Rgb,
    empty_color: Rgb,
    color_enabled: bool,
) -> String {
    let pct = percentage.clamp(0.0, 100.0);
    let filled_count = ((pct / 100.0) * (width as f32)).round() as usize;
    let empty_count = width.saturating_sub(filled_count);

    let (fill_char, empty_char) = match style {
        BarStyle::Smooth => ('█', '░'),
        BarStyle::Block => ('■', '□'),
        BarStyle::Circle => ('●', '○'),
        BarStyle::Rounded => ('━', '─'),
        BarStyle::Ascii => ('#', '-'),
    };

    let fill_str: String = std::iter::repeat(fill_char).take(filled_count).collect();
    let empty_str: String = std::iter::repeat(empty_char).take(empty_count).collect();

    let colored_fill = fg_rgb(&fill_str, filled_color, color_enabled);
    let colored_empty = fg_rgb(&empty_str, empty_color, color_enabled);

    format!("{}{}", colored_fill, colored_empty)
}
