#![allow(dead_code)]

use super::Logo;
use crate::utils::ansi::Rgb;

pub fn nour_signature() -> Logo {
    Logo {
        name: "nourfetch Cyber",
        lines: vec![
            "  _  _  _____  _   _  ____   ",
            " | \\| |/ _ \\ || | | |  _ \\  ",
            " | .` | (_) | || |_| | |_) | ",
            " |_|\\_|\\___/ \\__/\\___/|_| \\_\\",
            "      N O U R F E T C H      ",
        ],
        primary_color: Rgb::new(99, 102, 241),
        secondary_color: Rgb::new(236, 72, 153),
        accent_color: Rgb::new(56, 189, 248),
    }
}

pub fn nour_badge() -> Logo {
    Logo {
        name: "nourfetch Badge",
        lines: vec![
            "  ╭─────────────╮  ",
            "  │  ◆ NOUR ◆   │  ",
            "  │   FETCH     │  ",
            "  ╰─────────────╯  ",
        ],
        primary_color: Rgb::new(168, 85, 247),
        secondary_color: Rgb::new(99, 102, 241),
        accent_color: Rgb::new(245, 158, 11),
    }
}

pub fn generic() -> Logo {
    Logo {
        name: "Generic System",
        lines: vec![
            "    .--------.    ",
            "   / .------. \\   ",
            "  | /  NOUR  \\ |  ",
            "  | \\  FETCH / |  ",
            "   \\ '------' /   ",
            "    '--------'    ",
        ],
        primary_color: Rgb::new(100, 150, 255),
        secondary_color: Rgb::new(150, 200, 255),
        accent_color: Rgb::new(255, 255, 255),
    }
}
