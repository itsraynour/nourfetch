use std::env;

#[derive(Debug, Clone)]
pub enum ConfigCommand {
    Path,
    Get(Option<String>),
    Set(String, String),
    Reset,
}

#[derive(Debug, Clone, Default)]
pub struct CliArgs {
    pub help: bool,
    pub version: bool,
    pub install: bool,
    pub uninstall: bool,
    pub force: bool,
    pub theme: Option<String>,
    pub logo: Option<String>,
    pub layout: Option<String>,
    pub compact: bool,
    pub modern: bool,
    pub classic: bool,
    pub json: bool,
    pub no_color: bool,
    pub no_nerd_fonts: bool,
    pub nerd_fonts: Option<bool>,
    pub gen_config: bool,
    pub list_themes: bool,
    pub list_logos: bool,
    pub save: bool,
    pub config_cmd: Option<ConfigCommand>,
}

impl CliArgs {
    pub fn parse() -> Self {
        let mut args = Self::default();
        let raw_list: Vec<String> = env::args().skip(1).collect();
        let mut idx = 0;

        if !raw_list.is_empty() {
            let first = raw_list[0].to_lowercase();
            if first == "uninstall" {
                args.uninstall = true;
                for a in raw_list.iter().skip(1) {
                    if a == "-y" || a == "--yes" || a == "-f" || a == "--force" {
                        args.force = true;
                    }
                }
                return args;
            } else if first == "config" {
                if raw_list.len() == 1 {
                    args.config_cmd = Some(ConfigCommand::Get(None));
                    return args;
                }
                let sub = raw_list[1].to_lowercase();
                match sub.as_str() {
                    "path" => {
                        args.config_cmd = Some(ConfigCommand::Path);
                        return args;
                    }
                    "reset" => {
                        args.config_cmd = Some(ConfigCommand::Reset);
                        return args;
                    }
                    "get" => {
                        let key = raw_list.get(2).cloned();
                        args.config_cmd = Some(ConfigCommand::Get(key));
                        return args;
                    }
                    "set" => {
                        if raw_list.len() >= 4 {
                            args.config_cmd = Some(ConfigCommand::Set(
                                raw_list[2].clone(),
                                raw_list[3].clone(),
                            ));
                        } else {
                            eprintln!("Error: 'config set' requires <key> and <value>. Example: nourfetch config set theme cyberpunk");
                            std::process::exit(1);
                        }
                        return args;
                    }
                    _ => {}
                }
            } else if first == "set" {
                if raw_list.len() >= 3 {
                    args.config_cmd = Some(ConfigCommand::Set(
                        raw_list[1].clone(),
                        raw_list[2].clone(),
                    ));
                    return args;
                } else {
                    eprintln!("Error: 'set' requires <key> and <value>. Example: nourfetch set theme cyberpunk");
                    std::process::exit(1);
                }
            }
        }

        while idx < raw_list.len() {
            let arg = &raw_list[idx];

            match arg.as_str() {
                "-h" | "--help" => args.help = true,
                "-v" | "--version" => args.version = true,
                "--install" => args.install = true,
                "-u" | "--uninstall" => args.uninstall = true,
                "-y" | "--yes" | "-f" | "--force" => args.force = true,
                "-s" | "--save" | "-p" | "--persist" => args.save = true,

                "-t" | "--theme" => {
                    idx += 1;
                    if idx < raw_list.len() {
                        args.theme = Some(raw_list[idx].clone());
                    }
                }
                "+t" | "+theme" => {
                    idx += 1;
                    if idx < raw_list.len() {
                        args.theme = Some(raw_list[idx].clone());
                        args.save = true;
                    }
                }

                "-l" | "--logo" => {
                    idx += 1;
                    if idx < raw_list.len() {
                        args.logo = Some(raw_list[idx].clone());
                    }
                }
                "+l" | "+logo" => {
                    idx += 1;
                    if idx < raw_list.len() {
                        args.logo = Some(raw_list[idx].clone());
                        args.save = true;
                    }
                }

                "--layout" => {
                    idx += 1;
                    if idx < raw_list.len() {
                        args.layout = Some(raw_list[idx].clone());
                    }
                }
                "+layout" => {
                    idx += 1;
                    if idx < raw_list.len() {
                        args.layout = Some(raw_list[idx].clone());
                        args.save = true;
                    }
                }

                "--compact" | "--mini" => args.compact = true,
                "+compact" | "+mini" => {
                    args.compact = true;
                    args.save = true;
                }

                "--modern" | "--card" | "--box" => args.modern = true,
                "+modern" | "+card" | "+box" => {
                    args.modern = true;
                    args.save = true;
                }

                "--classic" => args.classic = true,
                "+classic" => {
                    args.classic = true;
                    args.save = true;
                }

                "--nerd-fonts" | "--icons" => args.nerd_fonts = Some(true),
                "+nerd-fonts" | "+icons" => {
                    args.nerd_fonts = Some(true);
                    args.save = true;
                }

                "--no-nerd-fonts" | "--no-icons" => {
                    args.no_nerd_fonts = true;
                    args.nerd_fonts = Some(false);
                }
                "+no-nerd-fonts" | "+no-icons" => {
                    args.no_nerd_fonts = true;
                    args.nerd_fonts = Some(false);
                    args.save = true;
                }

                "--json" => args.json = true,
                "--no-color" => args.no_color = true,
                "--gen-config" => args.gen_config = true,
                "--list-themes" | "--themes" => args.list_themes = true,
                "--list-logos" | "--logos" => args.list_logos = true,

                _ => {
                    if let Some(val) = arg.strip_prefix("--theme=") {
                        args.theme = Some(val.to_string());
                    } else if let Some(val) = arg.strip_prefix("+theme=").or_else(|| arg.strip_prefix("+t=")) {
                        args.theme = Some(val.to_string());
                        args.save = true;
                    } else if let Some(val) = arg.strip_prefix("--logo=") {
                        args.logo = Some(val.to_string());
                    } else if let Some(val) = arg.strip_prefix("+logo=").or_else(|| arg.strip_prefix("+l=")) {
                        args.logo = Some(val.to_string());
                        args.save = true;
                    } else if let Some(val) = arg.strip_prefix("--layout=") {
                        args.layout = Some(val.to_string());
                    } else if let Some(val) = arg.strip_prefix("+layout=") {
                        args.layout = Some(val.to_string());
                        args.save = true;
                    }
                }
            }
            idx += 1;
        }

        args
    }

    pub fn print_help() {
        println!(
            "nourfetch {}\nA fast, zero-dependency system information tool with GPU & VRAM hardware detection.\n",
            env!("CARGO_PKG_VERSION")
        );
        println!(
            r#"USAGE:
    nourfetch [OPTIONS]
    nourfetch set <KEY> <VALUE>
    nourfetch config <path|get|set|reset>
    nourfetch uninstall [-y]

ONE-TIME RUN OPTIONS (Temporary):
    -h, --help                 Print help information
    -v, --version              Print version information
        --install              Install nourfetch to system PATH
    -u, --uninstall            Completely uninstall nourfetch and clean all files
    -y, --yes, --force         Skip confirmation prompt during uninstall
    -t, --theme <NAME>         Select color theme for this run
    -l, --logo <NAME>          Select ASCII logo for this run
        --layout <LAYOUT>      Select layout style (classic, modern, compact)
        --modern               Render modern boxed card layout
        --compact              Render minimal inline summary
        --classic              Render classic side-by-side layout
        --no-nerd-fonts        Disable Nerd Font icons for this run
        --no-color             Disable ANSI color output
        --json                 Output complete system & GPU specs in JSON
        --list-themes, --themes  List all available color themes
        --list-logos, --logos    List all available ASCII logos (GNU, Distros, Windows, Mac)
        --gen-config           Generate/restore default config.toml file

PERMANENT SETTINGS (Saved to config.toml):
    -s, --save                 Save all provided flags permanently into config
    +theme <NAME>, +t <NAME>   Set theme permanently (e.g. nourfetch +theme cyberpunk)
    +logo <NAME>, +l <NAME>    Set logo permanently (e.g. nourfetch +logo gnu)
    +layout <LAYOUT>           Set layout permanently (e.g. nourfetch +layout modern)
    +modern, +compact          Set layout mode permanently
    +nerd-fonts, +no-nerd-fonts Set icon preferences permanently

SUBCOMMANDS:
    set <KEY> <VAL>            Set config value (e.g. nourfetch set theme dracula)
    config path                Show config file path
    config get [KEY]           Print current saved configuration
    config set <KEY> <VAL>     Set and persist a config property
    config reset               Reset config back to default settings
    uninstall [-y|--force]     Completely remove nourfetch and its configs from the system

EXAMPLES:
    nourfetch                            # Run with current saved settings
    nourfetch --theme cyberpunk          # Temporary run with cyberpunk theme
    nourfetch --logo gnu                 # Temporary run with GNU logo
    nourfetch --theme cyberpunk --save   # Save cyberpunk as default theme
    nourfetch +theme cyberpunk           # Set default theme permanently
    nourfetch +logo gnu                  # Set default logo permanently
    nourfetch +modern                    # Set modern boxed layout permanently
    nourfetch set theme nord             # Update config value directly
    nourfetch config path                # Show config file location
    nourfetch uninstall                  # Completely uninstall nourfetch
"#
        );
    }
}
