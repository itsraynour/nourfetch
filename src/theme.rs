#![allow(dead_code)]

use crate::utils::ansi::Rgb;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub primary: Rgb,
    pub secondary: Rgb,
    pub accent: Rgb,
    pub label: Rgb,
    pub text: Rgb,
    pub border: Rgb,
    pub bar_fill: Rgb,
    pub bar_empty: Rgb,
    pub gradient_start: Rgb,
    pub gradient_end: Rgb,
}

impl Theme {
    pub fn cyberpunk() -> Self {
        Self {
            name: "cyberpunk",
            primary: Rgb::new(255, 0, 128),
            secondary: Rgb::new(0, 255, 238),
            accent: Rgb::new(255, 230, 0),
            label: Rgb::new(0, 240, 255),
            text: Rgb::new(245, 245, 245),
            border: Rgb::new(180, 0, 220),
            bar_fill: Rgb::new(255, 0, 128),
            bar_empty: Rgb::new(50, 20, 60),
            gradient_start: Rgb::new(255, 0, 128),
            gradient_end: Rgb::new(0, 240, 255),
        }
    }

    pub fn nour() -> Self {
        Self {
            name: "nour",
            primary: Rgb::new(99, 102, 241),
            secondary: Rgb::new(168, 85, 247),
            accent: Rgb::new(236, 72, 153),
            label: Rgb::new(56, 189, 248),
            text: Rgb::new(248, 250, 252),
            border: Rgb::new(129, 140, 248),
            bar_fill: Rgb::new(99, 102, 241),
            bar_empty: Rgb::new(30, 41, 59),
            gradient_start: Rgb::new(56, 189, 248),
            gradient_end: Rgb::new(168, 85, 247),
        }
    }

    pub fn catppuccin() -> Self {
        Self {
            name: "catppuccin",
            primary: Rgb::new(203, 166, 247),
            secondary: Rgb::new(137, 180, 250),
            accent: Rgb::new(245, 194, 231),
            label: Rgb::new(148, 226, 213),
            text: Rgb::new(205, 214, 244),
            border: Rgb::new(180, 190, 254),
            bar_fill: Rgb::new(203, 166, 247),
            bar_empty: Rgb::new(49, 50, 68),
            gradient_start: Rgb::new(137, 180, 250),
            gradient_end: Rgb::new(203, 166, 247),
        }
    }

    pub fn dracula() -> Self {
        Self {
            name: "dracula",
            primary: Rgb::new(189, 147, 249),
            secondary: Rgb::new(255, 121, 198),
            accent: Rgb::new(80, 250, 123),
            label: Rgb::new(139, 233, 253),
            text: Rgb::new(248, 248, 242),
            border: Rgb::new(98, 114, 164),
            bar_fill: Rgb::new(189, 147, 249),
            bar_empty: Rgb::new(40, 42, 54),
            gradient_start: Rgb::new(189, 147, 249),
            gradient_end: Rgb::new(255, 121, 198),
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "nord",
            primary: Rgb::new(136, 192, 208),
            secondary: Rgb::new(129, 161, 193),
            accent: Rgb::new(180, 142, 173),
            label: Rgb::new(143, 188, 187),
            text: Rgb::new(236, 239, 244),
            border: Rgb::new(76, 86, 106),
            bar_fill: Rgb::new(136, 192, 208),
            bar_empty: Rgb::new(46, 52, 64),
            gradient_start: Rgb::new(136, 192, 208),
            gradient_end: Rgb::new(180, 142, 173),
        }
    }

    pub fn sunset() -> Self {
        Self {
            name: "sunset",
            primary: Rgb::new(251, 146, 60),
            secondary: Rgb::new(244, 63, 94),
            accent: Rgb::new(250, 204, 21),
            label: Rgb::new(253, 186, 116),
            text: Rgb::new(255, 241, 242),
            border: Rgb::new(225, 29, 72),
            bar_fill: Rgb::new(251, 146, 60),
            bar_empty: Rgb::new(60, 20, 30),
            gradient_start: Rgb::new(250, 204, 21),
            gradient_end: Rgb::new(244, 63, 94),
        }
    }

    pub fn matrix() -> Self {
        Self {
            name: "matrix",
            primary: Rgb::new(0, 255, 102),
            secondary: Rgb::new(34, 197, 94),
            accent: Rgb::new(134, 239, 172),
            label: Rgb::new(74, 222, 128),
            text: Rgb::new(240, 253, 244),
            border: Rgb::new(22, 101, 52),
            bar_fill: Rgb::new(0, 255, 102),
            bar_empty: Rgb::new(10, 35, 18),
            gradient_start: Rgb::new(134, 239, 172),
            gradient_end: Rgb::new(0, 255, 102),
        }
    }

    pub fn tokyonight() -> Self {
        Self {
            name: "tokyonight",
            primary: Rgb::new(122, 162, 247),
            secondary: Rgb::new(187, 154, 247),
            accent: Rgb::new(125, 207, 255),
            label: Rgb::new(42, 195, 222),
            text: Rgb::new(192, 202, 245),
            border: Rgb::new(61, 89, 161),
            bar_fill: Rgb::new(122, 162, 247),
            bar_empty: Rgb::new(26, 27, 38),
            gradient_start: Rgb::new(125, 207, 255),
            gradient_end: Rgb::new(187, 154, 247),
        }
    }

    pub fn gruvbox() -> Self {
        Self {
            name: "gruvbox",
            primary: Rgb::new(254, 128, 25),
            secondary: Rgb::new(250, 189, 47),
            accent: Rgb::new(184, 187, 38),
            label: Rgb::new(142, 192, 124),
            text: Rgb::new(235, 219, 178),
            border: Rgb::new(102, 92, 84),
            bar_fill: Rgb::new(254, 128, 25),
            bar_empty: Rgb::new(40, 40, 40),
            gradient_start: Rgb::new(250, 189, 47),
            gradient_end: Rgb::new(254, 128, 25),
        }
    }

    pub fn minimal() -> Self {
        Self {
            name: "minimal",
            primary: Rgb::new(255, 255, 255),
            secondary: Rgb::new(200, 200, 200),
            accent: Rgb::new(160, 160, 160),
            label: Rgb::new(220, 220, 220),
            text: Rgb::new(240, 240, 240),
            border: Rgb::new(80, 80, 80),
            bar_fill: Rgb::new(255, 255, 255),
            bar_empty: Rgb::new(40, 40, 40),
            gradient_start: Rgb::new(255, 255, 255),
            gradient_end: Rgb::new(160, 160, 160),
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "cyberpunk" | "cyber" => Self::cyberpunk(),
            "catppuccin" | "cat" | "mocha" => Self::catppuccin(),
            "dracula" | "drac" => Self::dracula(),
            "nord" => Self::nord(),
            "sunset" | "warm" => Self::sunset(),
            "matrix" | "green" => Self::matrix(),
            "tokyonight" | "tokyo" => Self::tokyonight(),
            "gruvbox" | "gruv" => Self::gruvbox(),
            "minimal" | "mono" => Self::minimal(),
            _ => Self::nour(),
        }
    }

    pub fn list_all() -> &'static [&'static str] {
        &[
            "nour (default)",
            "cyberpunk",
            "catppuccin",
            "dracula",
            "nord",
            "sunset",
            "matrix",
            "tokyonight",
            "gruvbox",
            "minimal",
        ]
    }
}
