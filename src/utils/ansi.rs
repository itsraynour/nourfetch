#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim().trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self::new(r, g, b))
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let r = (self.r as f32 + (other.r as f32 - self.r as f32) * t).round() as u8;
        let g = (self.g as f32 + (other.g as f32 - self.g as f32) * t).round() as u8;
        let b = (self.b as f32 + (other.b as f32 - self.b as f32) * t).round() as u8;
        Self::new(r, g, b)
    }

    pub fn to_fg_ansi(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }

    pub fn to_bg_ansi(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
    }
}

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINE: &str = "\x1b[4m";

pub fn fg_rgb(text: &str, rgb: Rgb, enabled: bool) -> String {
    if !enabled {
        return text.to_string();
    }
    format!("{}{}{}", rgb.to_fg_ansi(), text, RESET)
}

pub fn bold(text: &str, enabled: bool) -> String {
    if !enabled {
        return text.to_string();
    }
    format!("{}{}{}", BOLD, text, RESET)
}

pub fn dim(text: &str, enabled: bool) -> String {
    if !enabled {
        return text.to_string();
    }
    format!("{}{}{}", DIM, text, RESET)
}

pub fn gradient_text(text: &str, start: Rgb, end: Rgb, enabled: bool) -> String {
    if !enabled {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    if len <= 1 {
        return fg_rgb(text, start, enabled);
    }

    let mut result = String::with_capacity(text.len() * 20);
    for (i, &ch) in chars.iter().enumerate() {
        let t = i as f32 / (len - 1) as f32;
        let color = start.lerp(&end, t);
        result.push_str(&color.to_fg_ansi());
        result.push(ch);
    }
    result.push_str(RESET);
    result
}

pub fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_escape = false;

    for ch in s.chars() {
        if ch == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if ch == 'm' || ch == 'H' || ch == 'J' || ch == 'K' {
                in_escape = false;
            }
        } else {
            out.push(ch);
        }
    }
    out
}

pub fn visual_len(s: &str) -> usize {
    strip_ansi(s).chars().count()
}

pub fn pad_right(s: &str, target_width: usize) -> String {
    let v_len = visual_len(s);
    if v_len >= target_width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(target_width - v_len))
    }
}
