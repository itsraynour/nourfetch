use super::Logo;
use crate::utils::ansi::Rgb;

pub fn macos() -> Logo {
    Logo {
        name: "macOS",
        lines: vec![
            "                    'c.          ",
            "                 ,xNMM.          ",
            "               .OMMMMo           ",
            "               OMMM0,            ",
            "     .;loddo:' lbdlob:.          ",
            "   cKMMMMMMMMMMNWMMMMMMK:.       ",
            " .KMMMMMMMMMMMMMMMMMMMMMWd.      ",
            " XMMMMMMMMMMMMMMMMMMMMMMMX.      ",
            ";MMMMMMMMMMMMMMMMMMMMMMMM:       ",
            ":MMMMMMMMMMMMMMMMMMMMMMMM:       ",
            ".MMMMMMMMMMMMMMMMMMMMMMMX.       ",
            " kMMMMMMMMMMMMMMMMMMMMMMWd.      ",
            " .XMMMMMMMMMMMMMMMMMMMMMMk       ",
            "  .XMMMMMMMMMMMMMMMMMMMMK.       ",
            "    kMMMMMMMMMMMMMMMMMMd.        ",
            "     ;KMMMMMMMWXXWMMMMk.         ",
            "       .cooc,.    .,coo:.        ",
        ],
        primary_color: Rgb::new(200, 200, 200),
        secondary_color: Rgb::new(140, 140, 140),
        accent_color: Rgb::new(255, 255, 255),
    }
}
