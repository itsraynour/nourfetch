use super::Logo;
use crate::utils::ansi::Rgb;

pub fn arch() -> Logo {
    Logo {
        name: "Arch Linux",
        lines: vec![
            "                   -`",
            "                  .o+`",
            "                 `ooo/",
            "                `+oooo:",
            "               `+oooooo:",
            "               -+oooooo+:",
            "             `/:-:++oooo+:",
            "            `/++++/+++++++:",
            "           `/++++++++++++++:",
            "          `/+++oooooooooooo/`",
            "         ./ooosssso++osssssso+`",
            "        .oossssso-````/ossssss+`",
            "       -osssssso.      :ssssssso.",
            "      :osssssss/        osssso+++.",
            "     /ossssssss/        +ssssooo/-",
            "   `/ossssso+/:-        -:/+osssso+-",
            "  `+sso+:-`                 `.-/+oso:",
            " `++:.                           `-/+/",
            " .`                                 `/",
        ],
        primary_color: Rgb::new(23, 147, 209),
        secondary_color: Rgb::new(18, 114, 163),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn arch_small() -> Logo {
    Logo {
        name: "Arch (Small)",
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
            "            .-/+oossssoo+\\-.",
            "        ´:+ssssssssssssssssss+:`",
            "      -+ssssssssssssssssssyyssss+-",
            "    .ossssssssssssssssssdMMMNysssso.",
            "   /ssssssssssshdmmNNmmyNMMMMhssssss\\",
            "  +ssssssssshmydMMMMMMMNddddyssssssss+",
            " /sssssssshNMMMyhhyyyyhmNMMMNhssssssss\\",
            ".ssssssssdMMMNhsssssssssshNMMMdssssssss.",
            "+sssshhhyNMMNyssssssssssssyNMMMysssssss+",
            "ossyNMMMNyMMhsssssssssssssshmmmhssssssso",
            "ossyNMMMNyMMhsssssssssssssshmmmhssssssso",
            "+sssshhhyNMMNyssssssssssssyNMMMysssssss+",
            ".ssssssssdMMMNhsssssssssshNMMMdssssssss.",
            " \\sssssssshNMMMyhhyyyyhdNMMMNhssssssss/",
            "  +sssssssssdmydMMMMMMMMddddyssssssss+",
            "   \\ssssssssssshdmNNNNmyNMMMMhssssss/",
            "    .ossssssssssssssssssdMMMNysssso.",
            "      -+sssssssssssssssssyyyssss+-",
            "        `:+ssssssssssssssssss+:`",
            "            .-\\+oossssoo+/-.",
        ],
        primary_color: Rgb::new(233, 84, 32),
        secondary_color: Rgb::new(119, 41, 83),
        accent_color: Rgb::new(240, 240, 240),
    }
}

pub fn ubuntu_small() -> Logo {
    Logo {
        name: "Ubuntu (Small)",
        lines: vec![
            "         _",
            "     ---(_)",
            " _/  ---  \\",
            "(_) |   |",
            "  \\  --- _/",
            "     ---(_)",
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
            "             .',;::::;,'.",
            "         .';:cccccccccccc:;,.",
            "      .;cccccccccccccccccccccc;.",
            "    .:cccccccccccccccccccccccccc:.",
            "  .;ccccccccccccc;.:dddl:.;ccccccc;.",
            " .:ccccccccccccc;OWMKOOXMWd;ccccccc:.",
            ".:ccccccccccccc;KMMc;cc;xMMc;ccccccc:.",
            ",cccccccccccccc;MMM.;cc;;WW:;cccccccc,",
            ":cccccccccccccc;MMM.;cccccccccccccccc:",
            ":ccccccc;oxOOOo;MMM0OOk.;cccccccccccc:",
            "cccccc;0MMKxdd:;MMMkddc.;cccccccccccc;",
            "ccccc;XM0';cccc;MMM.;cccccccccccccccc'",
            "ccccc;MMo;ccccc;MMW.;ccccccccccccccc;",
            "ccccc;0MNc.ccc.xMMd;ccccccccccccccc;",
            "cccccc;dNMWXXXWM0:;cccccccccccccc:,",
            "cccccccc;.:odl:.;cccccccccccccc:;,.",
            ":cccccccccccccccccccccccccccc:'.",
            ".:cccccccccccccccccccccc:;,..",
            "  '::cccccccccccccc::;,.",
        ],
        primary_color: Rgb::new(81, 162, 218),
        secondary_color: Rgb::new(41, 65, 114),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn fedora_small() -> Logo {
    Logo {
        name: "Fedora (Small)",
        lines: vec![
            "        ,'''''.",
            "       |   ,.  |",
            "       |  |  '_'",
            "  ,....|  |..",
            ".'  ,_;|   ..'",
            "|  |   |  |",
            "|  ',_,'  |",
            " '.     ,'",
            "   '''''",
        ],
        primary_color: Rgb::new(81, 162, 218),
        secondary_color: Rgb::new(41, 65, 114),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn fedora_old() -> Logo {
    Logo {
        name: "Fedora (Classic)",
        lines: vec![
            "          /:-------------:\\",
            "       :-------------------::",
            "     :-----------/shhOHbmp---:\\",
            "   /-----------omMMMNNNMMD  ---:",
            "  :-----------sMMMMNMNMP.    ---:",
            " :-----------:MMMdP-------    ---\\",
            ",------------:MMMd--------    ---:",
            ":------------:MMMd-------    .---:",
            ":----    oNMMMMMMMMMNho     .----:",
            ":--     .+shhhMMMmhhy++   .------/",
            ":-    -------:MMMd--------------:",
            ":-   --------/MMMd-------------;",
            ":-    ------/hMMMy------------:",
            ":-- :dMNdhhdNMMNo------------;",
            ":---:sdNMMMMNds:------------:",
            ":------:://:-------------::",
            ":---------------------://",
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
            "       _,met$$$$$gg.",
            "    ,g$$$$$$$$$$$$$$$P.",
            "  ,g$$P\"        \"\"\"Y$$\".",
            " ,$$P'              `$$$.",
            "',$$P       ,ggs.     `$$b:",
            "`d$$'     ,$P\"'   .    $$$",
            " $$P      d$'     ,    $$P",
            " $$:      $$.   -    ,d$$'",
            " $$;      Y$b._   _,d$P'",
            " Y$$.    `.`\"Y$$$$P\"'",
            " `$$b      \"-.__",
            "  `Y$$",
            "   `Y$$.",
            "     `$$b.",
            "       `Y$$b.",
            "          `\"Y$b._",
            "              `\"\"\"",
        ],
        primary_color: Rgb::new(215, 7, 81),
        secondary_color: Rgb::new(160, 0, 50),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn debian_small() -> Logo {
    Logo {
        name: "Debian (Small)",
        lines: vec![
            "  _____",
            " /  __ \\",
            "|  /    |",
            "|  \\___-",
            "-_",
            "  --_",
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
            "          ▗▄▄▄       ▗▄▄▄▄    ▄▄▄▖",
            "          ▜███▙       ▜███▙  ▟███▛",
            "           ▜███▙       ▜███▙▟███▛",
            "            ▜███▙       ▜██████▛",
            "     ▟█████████████████▙ ▜████▛     ▟▙",
            "    ▟███████████████████▙ ▜███▙    ▟██▙",
            "           ▄▄▄▄▖           ▜███▙  ▟███▛",
            "          ▟███▛             ▜██▛ ▟███▛",
            "         ▟███▛               ▜▛ ▟███▛",
            "▟███████████▛                  ▟██████████▙",
            "▜██████████▛                  ▟███████████▛",
            "      ▟███▛ ▟▙               ▟███▛",
            "     ▟███▛ ▟██▙             ▟███▛",
            "    ▟███▛  ▜███▙           ▝▀▀▀▀",
            "    ▜██▛    ▜███▙ ▜██████████████████▛",
            "     ▜▛     ▟████▙ ▜████████████████▛",
            "           ▟██████▙       ▜███▙",
            "          ▟███▛▜███▙       ▜███▙",
            "         ▟███▛  ▜███▙       ▜███▙",
            "         ▝▀▀▀    ▀▀▀▀▘       ▀▀▀▘",
        ],
        primary_color: Rgb::new(126, 186, 228),
        secondary_color: Rgb::new(82, 119, 195),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn nixos_small() -> Logo {
    Logo {
        name: "NixOS (Small)",
        lines: vec![
            "    \\\\  \\\\ //",
            " ==\\\\__\\\\/ //",
            "   //   \\\\//",
            "==//     //==",
            " //\\\\___//",
            "// /\\\\  \\\\==",
            "  // \\\\  \\\\",
        ],
        primary_color: Rgb::new(126, 186, 228),
        secondary_color: Rgb::new(82, 119, 195),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn nixos_old() -> Logo {
    Logo {
        name: "NixOS (Classic)",
        lines: vec![
            "          ::::.    ':::::     ::::'",
            "          ':::::    ':::::.  ::::'",
            "            :::::     '::::.:::::",
            "      .......:::::..... ::::::::",
            "     ::::::::::::::::::. ::::::    ::::.",
            "    ::::::::::::::::::::: :::::.  .::::'",
            "           .....           ::::' :::::'",
            "          :::::            '::' :::::'",
            " ........:::::               ' :::::::::::.",
            ":::::::::::::                 :::::::::::::",
            " ::::::::::: ..              :::::",
            "     .::::: .:::            :::::",
            "    .:::::  :::::          '''''    .....",
            "    :::::   ':::::.  ......:::::::::::::'",
            "     :::     ::::::. ':::::::::::::::::'",
            "            .:::::::: '::::::::::",
            "           .::::''::::.     '::::.",
            "          .::::'   ::::.     '::::.",
            "         .::::      ::::      '::::.",
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
            "       .hddddddddddddddddddddddh.",
            "      :dddddddddddddddddddddddddd:",
            "     /dddddddddddddddddddddddddddd/",
            "    +dddddddddddddddddddddddddddddd+",
            "  `sdddddddddddddddddddddddddddddddds`",
            " `ydddddddddddd++hdddddddddddddddddddy`",
            ".hddddddddddd+`  `+ddddh:-sdddddddddddh.",
            "hdddddddddd+`      `+y:    .sddddddddddh",
            "ddddddddh+`   `//`   `.`     -sddddddddd",
            "ddddddh+`   `/hddh/`   `:s-    -sddddddd",
            "ddddh+`   `/+/dddddh/`   `+s-    -sddddd",
            "ddd+`   `/o` :dddddddh/`   `oy-    .yddd",
            "hdddyo+ohddyosdddddddddho+oydddy++ohdddh",
            ".hddddddddddddddddddddddddddddddddddddh.",
            " `yddddddddddddddddddddddddddddddddddy`",
            "  `sdddddddddddddddddddddddddddddddds`",
            "    +dddddddddddddddddddddddddddddd+",
            "     /dddddddddddddddddddddddddddd/",
            "      :dddddddddddddddddddddddddd:",
            "       .hddddddddddddddddddddddh.",
        ],
        primary_color: Rgb::new(13, 89, 142),
        secondary_color: Rgb::new(34, 153, 221),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn alpine_small() -> Logo {
    Logo {
        name: "Alpine (Small)",
        lines: vec![
            "   /\\ /\\",
            "  // \\  \\",
            " //   \\  \\",
            "///    \\  \\",
            "//      \\  \\",
            "         \\",
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
            "             /////////////",
            "         /////////////////////",
            "      ///////*767////////////////",
            "    //////7676767676*//////////////",
            "   /////76767//7676767//////////////",
            "  /////767676///*76767///////////////",
            " ///////767676///76767.///7676*///////",
            "/////////767676//76767///767676////////",
            "//////////76767676767////76767/////////",
            "///////////76767676//////7676//////////",
            "////////////,7676,///////767///////////",
            "/////////////*7676///////76////////////",
            "///////////////7676////////////////////",
            " ///////////////7676///767////////////",
            "  //////////////////////'////////////",
            "   //////.7676767676767676767,//////",
            "    /////767676767676767676767/////",
            "      ///////////////////////////",
            "         /////////////////////",
            "             /////////////",
        ],
        primary_color: Rgb::new(72, 185, 199),
        secondary_color: Rgb::new(246, 174, 52),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn pop_os_small() -> Logo {
    Logo {
        name: "Pop!_OS (Small)",
        lines: vec![
            "______",
            "\\   _ \\        __",
            " \\ \\ \\ \\      / /",
            "  \\ \\_\\ \\    / /",
            "   \\  ___\\  /_/",
            "    \\ \\    _",
            "   __\\_\\__(_)_",
            "  (___________)`",
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
            "██████████████████  ████████",
            "██████████████████  ████████",
            "██████████████████  ████████",
            "██████████████████  ████████",
            "████████            ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
            "████████  ████████  ████████",
        ],
        primary_color: Rgb::new(53, 191, 92),
        secondary_color: Rgb::new(35, 130, 62),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn manjaro_small() -> Logo {
    Logo {
        name: "Manjaro (Small)",
        lines: vec![
            "||||||||| ||||",
            "||||||||| ||||",
            "||||      ||||",
            "|||| |||| ||||",
            "|||| |||| ||||",
            "|||| |||| ||||",
            "|||| |||| ||||",
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

pub fn gnu() -> Logo {
    Logo {
        name: "GNU",
        lines: vec![
            "    ,           ,  ",
            "   /             \\ ",
            "  ((__-^^-,-^^-__))",
            "    `-_---'  `---_-'",
            "     `--|o`   'o|--'",
            "        \\  `  /    ",
            "         ): :(     ",
            "         :o_o:     ",
            "          \"-\"      ",
        ],
        primary_color: Rgb::new(179, 142, 93),
        secondary_color: Rgb::new(218, 179, 126),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn gnu_small() -> Logo {
    Logo {
        name: "GNU (Small)",
        lines: vec![
            "  ,-'\"\"`-.  ",
            " / (o)(o) \\ ",
            " |  .--.  | ",
            "  \\ `..' /  ",
            "   `----'   ",
        ],
        primary_color: Rgb::new(179, 142, 93),
        secondary_color: Rgb::new(220, 190, 140),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn guix() -> Logo {
    Logo {
        name: "GNU Guix",
        lines: vec![
            " |.__         ___.| ",
            "  \\__`--..---'__/  ",
            "    \\___/ \\___/     ",
            "     | `---' |      ",
            "     |       |      ",
            "     `-------'      ",
        ],
        primary_color: Rgb::new(255, 213, 0),
        secondary_color: Rgb::new(51, 51, 51),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn gentoo() -> Logo {
    Logo {
        name: "Gentoo Linux",
        lines: vec![
            "         -/oyddmdhs+:.",
            "     -odNMMMMMMMMNNmhy+-`",
            "   -yNMMMMMMMMMMMNNNmmdhy+-",
            " `omMMMMMMMMMMMMNmdmmmmddhhy/`",
            " omMMMMMMMMMMNhhyyyohmdddhhhdo`",
            ".ydMMMMMMMMMMdhs++so/smdddhhhhdm+`",
            " oyhdmNMMMMMMMNdyooydmddddhhhhyhNd.",
            "  :oyhhdNNMMMMMMMNNNmmdddhhhhhyymMh",
            "    .:+sydNMMMMMNNNmmmdddhhhhhhmMmy",
            "       /mMMMMMMNNNmmmdddhhhhhmMNhs:",
            "    `oNMMMMMMMNNNmmmddddhhdmMNhs+`",
            "  `sNMMMMMMMMNNNmmmdddddmNMmhs/.",
            " /NMMMMMMMMNNNNmmmdddmNMNdso:`",
            "+MMMMMMMNNNNNmmmmdmNMNdso/-",
            "yMMNNNNNNNmmmmmNNMmhs+/-`",
            "/hMMNNNNNNNNMNdhs++/-`",
            "`/ohdmmddhys+++/:.`",
            "  `-//////:--.",
        ],
        primary_color: Rgb::new(204, 187, 240),
        secondary_color: Rgb::new(140, 110, 210),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn gentoo_small() -> Logo {
    Logo {
        name: "Gentoo (Small)",
        lines: vec![
            "   .-----.   ",
            " ./  -   -\\  ",
            "/ /       /\\ ",
            "| |      / / ",
            "\\ \\_____/ /  ",
            " `-------'   ",
        ],
        primary_color: Rgb::new(204, 187, 240),
        secondary_color: Rgb::new(90, 60, 160),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn kali() -> Logo {
    Logo {
        name: "Kali Linux",
        lines: vec![
            "  ..............  ",
            "  ...        ...  ",
            "  ..  .:::.   ..  ",
            "   . ::::::   .   ",
            "    ':::::'  '    ",
            "      `:'         ",
        ],
        primary_color: Rgb::new(85, 127, 255),
        secondary_color: Rgb::new(0, 0, 0),
        accent_color: Rgb::new(42, 192, 255),
    }
}

pub fn void() -> Logo {
    Logo {
        name: "Void Linux",
        lines: vec![
            "    _______    ",
            "   /   /   \\   ",
            "  /   /     \\  ",
            " /   /   __  \\ ",
            " \\   \\__/ /  / ",
            "  \\      /  /  ",
            "   \\____/__/   ",
        ],
        primary_color: Rgb::new(71, 128, 97),
        secondary_color: Rgb::new(40, 75, 55),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn opensuse() -> Logo {
    Logo {
        name: "openSUSE",
        lines: vec![
            "           .;ldkO0000Okdl;.",
            "       .;d00xl:^''''''^:ok00d;.",
            "     .d00l'                'o00d.",
            "   .d0Kd'  Okxol:;,.          :O0d.",
            "  .OKKKK0kOKKKKKKKKKKOxo:,      lKO.",
            " ,0KKKKKKKKKKKKKKKK0P^,,,^dx:    ;00,",
            ".OKKKKKKKKKKKKKKKKk'.oOPPb.'0k.   cKO.",
            ":KKKKKKKKKKKKKKKK: kKx..dd lKd   'OK:",
            "dKKKKKKKKKKOx0KKKd ^0KKKO' kKKc   dKd",
            "dKKKKKKKKKK;.;oOKx,..^..;kKKK0.  dKd",
            ":KKKKKKKKKK0o;...^cdxxOK0O/^^'  .0K:",
            " kKKKKKKKKKKKKK0x;,,......,;od  lKk",
            " '0KKKKKKKKKKKKKKKKKKKK00KKOo^  c00'",
            "  'kKKKOxddxkOO00000Okxoc;''   .dKk'",
            "    l0Ko.                    .c00l'",
            "     'l0Kk:.              .;xK0l'",
            "        'lkK0xl:;,,,,;:ldO0kl'",
            "            '^:ldxkkkkxdl:^'",
        ],
        primary_color: Rgb::new(115, 186, 37),
        secondary_color: Rgb::new(48, 140, 20),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn opensuse_small() -> Logo {
    Logo {
        name: "openSUSE (Small)",
        lines: vec![
            "  _______",
            "__|   __ \\",
            "     / .\\ \\",
            "     \\__/ |",
            "   _______|",
            "   \\_______",
            "__________/",
        ],
        primary_color: Rgb::new(115, 186, 37),
        secondary_color: Rgb::new(48, 140, 20),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn artix() -> Logo {
    Logo {
        name: "Artix Linux",
        lines: vec![
            "                   '",
            "                  'o'",
            "                 'ooo'",
            "                'ooxoo'",
            "               'ooxxxoo'",
            "              'oookkxxoo'",
            "             'oiioxkkxxoo'",
            "            ':;:iiiioxxxoo'",
            "               `'.;::ioxxoo'",
            "          '-.      `':;jiooo'",
            "         'oooio-..     `'i:io'",
            "        'ooooxxxxoio:,.   `'-;'",
            "       'ooooxxxxxkkxoooIi:-.  `'",
            "      'ooooxxxxxkkkkxoiiiiiji'",
            "     'ooooxxxxxkxxoiiii:'`     .i'",
            "    'ooooxxxxxoi:::'`       .;ioxo'",
            "   'ooooxooi::'`         .:iiixkxxo'",
            "  'ooooi:'`                `'';ioxxo'",
            " 'i:'`                          '':io'",
            "'`                                   `'",
        ],
        primary_color: Rgb::new(24, 169, 219),
        secondary_color: Rgb::new(16, 98, 138),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn artix_small() -> Logo {
    Logo {
        name: "Artix (Small)",
        lines: vec![
            "      /\\",
            "     /  \\",
            "    /`'.,\\",
            "   /     ',",
            "  /      ,`\\",
            " /   ,.'`.  \\",
            "/.,'`     `'.\\",
        ],
        primary_color: Rgb::new(24, 169, 219),
        secondary_color: Rgb::new(16, 98, 138),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn endeavouros() -> Logo {
    Logo {
        name: "EndeavourOS",
        lines: vec![
            "      /\\      ",
            "    //  \\\\    ",
            "   // /\\ \\\\   ",
            "  // /  \\ \\\\  ",
            " // /    \\ \\\\ ",
            "//_/      \\_\\\\",
        ],
        primary_color: Rgb::new(127, 63, 191),
        secondary_color: Rgb::new(255, 102, 102),
        accent_color: Rgb::new(0, 204, 255),
    }
}

pub fn linuxmint() -> Logo {
    Logo {
        name: "Linux Mint",
        lines: vec![
            "             ...-:::::-...",
            "          .-MMMMMMMMMMMMMMM-.",
            "      .-MMMM`..-:::::::-..`MMMM-.",
            "    .:MMMM.:MMMMMMMMMMMMMMM:.MMMM:.",
            "   -MMM-M---MMMMMMMMMMMMMMMMMMM.MMM-",
            " `:MMM:MM`  :MMMM:....::-...-MMMM:MMM:`",
            " :MMM:MMM`  :MM:`  ``    ``  `:MMM:MMM:",
            ".MMM.MMMM`  :MM.  -MM.  .MM-  `MMMM.MMM.",
            ":MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM-MMM:",
            ":MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM:MMM:",
            ":MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM-MMM:",
            ".MMM.MMMM`  :MM:--:MM:--:MM:  `MMMM.MMM.",
            " :MMM:MMM-  `-MMMMMMMMMMMM-`  -MMM-MMM:",
            "  :MMM:MMM:`                `:MMM:MMM:",
            "   .MMM.MMMM:--------------:MMMM.MMM.",
            "     '-MMMM.-MMMMMMMMMMMMMMM-.MMMM-'",
            "       '.-MMMM``--:::::--``MMMM-.'",
            "            '-MMMMMMMMMMMMM-'",
            "               ``-:::::-``",
        ],
        primary_color: Rgb::new(135, 207, 62),
        secondary_color: Rgb::new(100, 175, 45),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn linuxmint_small() -> Logo {
    Logo {
        name: "Linux Mint (Small)",
        lines: vec![
            " ___________",
            "|_          \\",
            "  | | _____ |",
            "  | | | | | |",
            "  | | | | | |",
            "  | \\_____/ |",
            "  \\_________/",
        ],
        primary_color: Rgb::new(135, 207, 62),
        secondary_color: Rgb::new(100, 175, 45),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn linuxmint_old() -> Logo {
    Logo {
        name: "Linux Mint (Classic)",
        lines: vec![
            "MMMMMMMMMMMMMMMMMMMMMMMMMmds+.",
            "MMm----::-://////////////oymNMd+`",
            "MMd      /++                -sNMd:",
            "MMNso/`  dMM    `.::-. .-::.` .hMN:",
            "ddddMMh  dMM   :hNMNMNhNMNMNh: `NMm",
            "    NMm  dMM  .NMN/-+MMM+-/NMN` dMM",
            "    NMm  dMM  -MMm  `MMM   dMM. dMM",
            "    NMm  dMM  -MMm  `MMM   dMM. dMM",
            "    NMm  dMM  .mmd  `mmm   yMM. dMM",
            "    NMm  dMM`  ..`   ...   ydm. dMM",
            "    hMM- +MMd/-------...-:sdds  dMM",
            "    -NMm- :hNMNNNmdddddddddy/`  dMM",
            "     -dMNs-``-::::-------.``    dMM",
            "      `/dMNmy+/:-------------:/yMMM",
            "         ./ydNMMMMMMMMMMMMMMMMMMMMM",
            "            .MMMMMMMMMMMMMMMMMMM",
        ],
        primary_color: Rgb::new(135, 207, 62),
        secondary_color: Rgb::new(100, 175, 45),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn zorin() -> Logo {
    Logo {
        name: "Zorin OS",
        lines: vec![
            "        `osssssssssssssssssssso`",
            "       .osssssssssssssssssssssso.",
            "      .+oooooooooooooooooooooooo+.",
            "",
            "",
            "  `::::::::::::::::::::::.         .:`",
            " `+ssssssssssssssssss+:.`     `.:+ssso`",
            ".ossssssssssssssso/.       `-+ossssssso.",
            "ssssssssssssso/-`      `-/osssssssssssss",
            ".ossssssso/-`      .-/ossssssssssssssso.",
            " `+sss+:.      `.:+ssssssssssssssssss+`",
            "  `:.         .::::::::::::::::::::::`",
            "",
            "",
            "      .+oooooooooooooooooooooooo+.",
            "       -osssssssssssssssssssssso-",
            "        `osssssssssssssssssssso`",
        ],
        primary_color: Rgb::new(20, 166, 240),
        secondary_color: Rgb::new(0, 200, 255),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn zorin_small() -> Logo {
    Logo {
        name: "Zorin (Small)",
        lines: vec![
            "  ________  ",
            " / ______/  ",
            " | |   / /  ",
            " | |  / /   ",
            " | | / /    ",
            " | |/ /___  ",
            " /_______/  ",
        ],
        primary_color: Rgb::new(20, 166, 240),
        secondary_color: Rgb::new(12, 110, 160),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn elementary() -> Logo {
    Logo {
        name: "elementary OS",
        lines: vec![
            "         eeeeeeeeeeeeeeeee",
            "      eeeeeeeeeeeeeeeeeeeeeee",
            "    eeeee  eeeeeeeeeeee   eeeee",
            "  eeee   eeeee       eee     eeee",
            " eeee   eeee          eee     eeee",
            "eee    eee            eee       eee",
            "eee   eee            eee        eee",
            "ee    eee           eeee       eeee",
            "ee    eee         eeeee      eeeeee",
            "ee    eee       eeeee      eeeee ee",
            "eee   eeee   eeeeee      eeeee  eee",
            "eee    eeeeeeeeee     eeeeee    eee",
            " eeeeeeeeeeeeeeeeeeeeeeee    eeeee",
            "  eeeeeeee eeeeeeeeeeee      eeee",
            "    eeeee                 eeeee",
            "      eeeeeee         eeeeeee",
            "         eeeeeeeeeeeeeeeee",
        ],
        primary_color: Rgb::new(64, 165, 229),
        secondary_color: Rgb::new(100, 180, 240),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn elementary_small() -> Logo {
    Logo {
        name: "elementary (Small)",
        lines: vec![
            "  _______",
            " / ____  \\",
            "/  |  /  /\\",
            "|__\\ /  / |",
            "\\   /__/  /",
            " \\_______/",
        ],
        primary_color: Rgb::new(64, 165, 229),
        secondary_color: Rgb::new(100, 180, 240),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn slackware() -> Logo {
    Logo {
        name: "Slackware",
        lines: vec![
            "                  :::::::",
            "            :::::::::::::::::::",
            "         :::::::::::::::::::::::::",
            "       ::::::::cllcccccllllllll::::::",
            "    :::::::::lc               dc:::::::",
            "   ::::::::cl   clllccllll    oc:::::::::",
            "  :::::::::o   lc::::::::co   oc::::::::::",
            " ::::::::::o    cccclc:::::clcc::::::::::::",
            " :::::::::::lc        cclccclc:::::::::::::",
            "::::::::::::::lcclcc          lc::::::::::::",
            "::::::::::cclcc:::::lccclc     oc:::::::::::",
            "::::::::::o    l::::::::::l    lc:::::::::::",
            " :::::cll:o     clcllcccll     o:::::::::::",
            " :::::occ:o                  clc:::::::::::",
            "  ::::ocl:ccslclccclclccclclc:::::::::::::",
            "   :::oclcccccccccccccllllllllllllll:::::",
            "    ::lcc1lcccccccccccccccccccccccco::::",
            "      ::::::::::::::::::::::::::::::::",
            "        ::::::::::::::::::::::::::::",
            "           ::::::::::::::::::::::",
            "                ::::::::::::",
        ],
        primary_color: Rgb::new(98, 102, 178),
        secondary_color: Rgb::new(64, 68, 148),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn slackware_small() -> Logo {
    Logo {
        name: "Slackware (Small)",
        lines: vec![
            "   ________",
            "  /  ______|",
            "  | |______",
            "  \\______  \\",
            "   ______| |",
            "| |________/",
            "|____________",
        ],
        primary_color: Rgb::new(98, 102, 178),
        secondary_color: Rgb::new(64, 68, 148),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn freebsd() -> Logo {
    Logo {
        name: "FreeBSD",
        lines: vec![
            "   ```                        `",
            "  ` `.....---.......--.```   -/",
            "  +o   .--`         /y:`      +.",
            "   yo`:.            :o      `+-",
            "    y/               -/`   -o/",
            "   .-                  ::/sy+:.",
            "   /                     `--  /",
            "  `:                          :`",
            "  `:                          :`",
            "   /                          /",
            "   .-                        -.",
            "    --                      -.",
            "     `:`                  `:`",
            "       .--             `--.",
            "          .---.....----.",
        ],
        primary_color: Rgb::new(171, 18, 0),
        secondary_color: Rgb::new(220, 50, 40),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn freebsd_small() -> Logo {
    Logo {
        name: "FreeBSD (Small)",
        lines: vec![
            "/\\,-'''''-,/\\",
            "\\_)       (_/",
            "|           |",
            "|           |",
            " ;         ;",
            "  '-_____-'",
        ],
        primary_color: Rgb::new(171, 18, 0),
        secondary_color: Rgb::new(220, 50, 40),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn redhat() -> Logo {
    Logo {
        name: "Red Hat",
        lines: vec![
            "           .MMM..:MMMMMMM",
            "          MMMMMMMMMMMMMMMMMM",
            "          MMMMMMMMMMMMMMMMMMMM.",
            "         MMMMMMMMMMMMMMMMMMMMMM",
            "        ,MMMMMMMMMMMMMMMMMMMMMM:",
            "        MMMMMMMMMMMMMMMMMMMMMMMM",
            "  .MMMM'  MMMMMMMMMMMMMMMMMMMMMM",
            " MMMMMM    `MMMMMMMMMMMMMMMMMMMM.",
            "MMMMMMMM      MMMMMMMMMMMMMMMMMM .",
            "MMMMMMMMM.       `MMMMMMMMMMMMM' MM.",
            "MMMMMMMMMMM.                     MMMM",
            "`MMMMMMMMMMMMM.                 ,MMMMM.",
            " `MMMMMMMMMMMMMMMMM.          ,MMMMMMMM.",
            "    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
            "      MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM:",
            "         MMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
            "            `MMMMMMMMMMMMMMMMMMMMMMMM:",
            "                ``MMMMMMMMMMMMMMMMM'",
        ],
        primary_color: Rgb::new(238, 0, 0),
        secondary_color: Rgb::new(204, 0, 0),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn redhat_small() -> Logo {
    Logo {
        name: "Red Hat (Small)",
        lines: vec![
            "    .---.    ",
            "   /     \\   ",
            " _/_______\\_ ",
            "(___________)",
            "  \\_______/  ",
        ],
        primary_color: Rgb::new(238, 0, 0),
        secondary_color: Rgb::new(20, 20, 20),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn rocky() -> Logo {
    Logo {
        name: "Rocky Linux",
        lines: vec![
            "          __wgliliiligw_,",
            "       _williiiiiiliilililw,",
            "     _%iiiiiilililiiiiiiiiiii_",
            "   .Qliiiililiiiiiiililililiilm.",
            "  _iiiiiliiiiiililiiiiiiiiiiliil,",
            " .lililiiilililiiiilililililiiiii,",
            "_liiiiiiliiiiiiiliiiiiF{iiiiiilili,",
            "jliililiiilililiiili@`  ~ililiiiiiL",
            "iiiliiiiliiiiiiili>`      ~liililii",
            "liliiiliiilililii`         -9liiiil",
            "iiiiiliiliiiiii~             \"4lili",
            "4ililiiiiilil~|      -w,       )4lf",
            "-liiiiililiF'       _liig,       )'",
            " )iiiliii@`       _QIililig,",
            "  )iiii>`       .Qliliiiililw",
            "   )<>~       .mliiiiiliiiiiil,",
            "            _gllilililiililii~",
            "           giliiiiiiiiiiiiT`",
            "          -^~$ililili@~~'",
        ],
        primary_color: Rgb::new(16, 185, 129),
        secondary_color: Rgb::new(15, 118, 110),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn rocky_small() -> Logo {
    Logo {
        name: "Rocky (Small)",
        lines: vec![
            "    `-/+++++++++/-.`",
            " `-+++++++++++++++++-`",
            ".+++++++++++++++++++++.",
            "-+++++++++++++++++++++++.",
            "+++++++++++++++/-/+++++++",
            "+++++++++++++/.   ./+++++",
            "+++++++++++:.       ./+++",
            "+++++++++:`   `:/:`   .:/",
            "-++++++:`   .:+++++:`",
            " .+++-`   ./+++++++++:`",
            "  `-`   ./+++++++++++-",
            "       -+++++++++:-.`",
        ],
        primary_color: Rgb::new(16, 185, 129),
        secondary_color: Rgb::new(15, 118, 110),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn garuda() -> Logo {
    Logo {
        name: "Garuda Linux",
        lines: vec![
            "     /\\     ",
            "   _/  \\_   ",
            "  / /\\/\\ \\  ",
            " / /    \\ \\ ",
            "/_/      \\_\\",
        ],
        primary_color: Rgb::new(255, 107, 0),
        secondary_color: Rgb::new(255, 46, 99),
        accent_color: Rgb::new(0, 229, 255),
    }
}

pub fn parrot() -> Logo {
    Logo {
        name: "Parrot OS",
        lines: vec![
            "  `:oho/-`",
            "`mMMMMMMMMMMMNmmdhy-",
            " dMMMMMMMMMMMMMMMMMMs`",
            " +MMsohNMMMMMMMMMMMMMm/",
            " .My   .+dMMMMMMMMMMMMMh.",
            "  +       :NMMMMMMMMMMMMNo",
            "           `yMMMMMMMMMMMMMm:",
            "             /NMMMMMMMMMMMMMy`",
            "              .hMMMMMMMMMMMMMN+",
            "                  ``-NMMMMMMMMMd-",
            "                     /MMMMMMMMMMMs`",
            "                      mMMMMMMMsyNMN/",
            "                      +MMMMMMMo  :sNh.",
            "                      `NMMMMMMm     -o/",
            "                       oMMMMMMM.",
            "                       `NMMMMMM+",
            "                        +MMd/NMh",
            "                         mMm -mN`",
            "                         /MM  `h:",
            "                          dM`   .",
            "                          :M-",
            "                           d:",
            "                           -+",
            "                            -",
        ],
        primary_color: Rgb::new(0, 229, 255),
        secondary_color: Rgb::new(0, 150, 255),
        accent_color: Rgb::new(255, 255, 255),
    }
}

pub fn parrot_small() -> Logo {
    Logo {
        name: "Parrot (Small)",
        lines: vec![
            "   .---.   ",
            "  / .-. \\  ",
            " | |   | | ",
            " | `---' | ",
            "  \\_____/  ",
            "   //   \\\\ ",
        ],
        primary_color: Rgb::new(0, 229, 255),
        secondary_color: Rgb::new(0, 150, 255),
        accent_color: Rgb::new(255, 255, 255),
    }
}
