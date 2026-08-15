use super::Logo;
use crate::utils::ansi::Rgb;

pub fn arch() -> Logo {
    Logo {
        name: "Arch Linux",
        lines: vec![
            "      /\\      ",
            "     /  \\     ",
            "    /\\   \\    ",
            "   /      \\   ",
            "  /   ,,   \\  ",
            " /   |  |  -\\ ",
            "/_-''    ''-_\\",
        ],
        primary_color: Rgb::new(23, 147, 209),
        secondary_color: Rgb::new(18, 114, 163),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn ubuntu() -> Logo {
    Logo {
        name: "Ubuntu",
        lines: vec![
            "         _    ",
            "     ---(_)   ",
            " _/  ---  \\   ",
            "(_) |   |     ",
            "  \\  --- _/   ",
            "     ---(_)   ",
        ],
        primary_color: Rgb::new(233, 84, 32),
        secondary_color: Rgb::new(119, 41, 83),
        accent_color: Rgb::new(240, 240, 240),
    }
}

pub fn fedora() -> Logo {
    Logo {
        name: "Fedora",
        lines: vec![
            "        ,'''''.       ",
            "       /   ,-. `\\     ",
            "      |  /   \\   |    ",
            "    ,-'--.    |  |    ",
            "   / .--. \\   |  |    ",
            "   | |  | |   |  |    ",
            "   \\ `--' /  /___|    ",
            "    `---''            ",
        ],
        primary_color: Rgb::new(81, 162, 218),
        secondary_color: Rgb::new(41, 65, 114),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn debian() -> Logo {
    Logo {
        name: "Debian",
        lines: vec![
            "     .---.     ",
            "    /     \\    ",
            "   | () () |   ",
            "    \\  ^  /    ",
            "     |||||     ",
            "     |||||     ",
        ],
        primary_color: Rgb::new(215, 7, 81),
        secondary_color: Rgb::new(160, 0, 50),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn nixos() -> Logo {
    Logo {
        name: "NixOS",
        lines: vec![
            "  \\\\  \\\\ //  ",
            " ==\\\\__\\\\/   ",
            "   //   \\\\== ",
            "  //     \\\\  ",
        ],
        primary_color: Rgb::new(126, 186, 228),
        secondary_color: Rgb::new(82, 119, 195),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn alpine() -> Logo {
    Logo {
        name: "Alpine Linux",
        lines: vec![
            "   /\\       ",
            "  // \\      ",
            " //   \\     ",
            "///    \\ /\\ ",
            "//      // \\",
            "/      //   \\",
        ],
        primary_color: Rgb::new(13, 89, 142),
        secondary_color: Rgb::new(34, 153, 221),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn pop_os() -> Logo {
    Logo {
        name: "Pop!_OS",
        lines: vec![
            "  ______                _   ",
            " |  ___ \\              | |  ",
            " | | _/ /___  ____   __| |  ",
            " |  __/ / _ \\| '_ \\ / _` |  ",
            " |_|    \\___/| .__/ \\__,_|  ",
            "             |_|            ",
        ],
        primary_color: Rgb::new(72, 185, 199),
        secondary_color: Rgb::new(246, 174, 52),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn manjaro() -> Logo {
    Logo {
        name: "Manjaro",
        lines: vec![
            "███████ ███████ ███████",
            "███████ ███████ ███████",
            "███████         ███████",
            "███████ ███████ ███████",
            "███████ ███████ ███████",
            "███████ ███████ ███████",
            "███████ ███████ ███████",
        ],
        primary_color: Rgb::new(53, 191, 92),
        secondary_color: Rgb::new(35, 130, 62),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn tux() -> Logo {
    Logo {
        name: "Linux (Tux)",
        lines: vec![
            "    .--.    ",
            "   |o_o |   ",
            "   |:_/ |   ",
            "  //   \\ \\  ",
            " (|     | ) ",
            "/'\\_   _/`\\ ",
            "\\___)=(___/ ",
        ],
        primary_color: Rgb::new(250, 204, 21),
        secondary_color: Rgb::new(245, 245, 245),
        accent_color: Rgb::new(30, 30, 30),
    }
}
