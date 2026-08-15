use std::env;

#[derive(Debug, Clone, Default)]
pub struct CliArgs {
    pub help: bool,
    pub version: bool,
    pub install: bool,
    pub theme: Option<String>,
    pub logo: Option<String>,
    pub layout: Option<String>,
    pub compact: bool,
    pub modern: bool,
    pub json: bool,
    pub no_color: bool,
    pub no_nerd_fonts: bool,
    pub gen_config: bool,
    pub list_themes: bool,
    pub list_logos: bool,
}

impl CliArgs {
    pub fn parse() -> Self {
        let mut args = Self::default();
        let mut raw_args = env::args().skip(1);

        while let Some(arg) = raw_args.next() {
            match arg.as_str() {
                "-h" | "--help" => args.help = true,
                "-v" | "--version" => args.version = true,
                "--install" => args.install = true,
                "-t" | "--theme" => {
                    if let Some(val) = raw_args.next() {
                        args.theme = Some(val);
                    }
                }
                "-l" | "--logo" => {
                    if let Some(val) = raw_args.next() {
                        args.logo = Some(val);
                    }
                }
                "--layout" => {
                    if let Some(val) = raw_args.next() {
                        args.layout = Some(val);
                    }
                }
                "--compact" => args.compact = true,
                "--modern" | "--card" => args.modern = true,
                "--json" => args.json = true,
                "--no-color" => args.no_color = true,
                "--no-nerd-fonts" | "--no-icons" => args.no_nerd_fonts = true,
                "--gen-config" => args.gen_config = true,
                "--list-themes" => args.list_themes = true,
                "--list-logos" => args.list_logos = true,
                _ => {
                    if arg.starts_with("--theme=") {
                        args.theme = Some(arg.trim_start_matches("--theme=").to_string());
                    } else if arg.starts_with("--logo=") {
                        args.logo = Some(arg.trim_start_matches("--logo=").to_string());
                    } else if arg.starts_with("--layout=") {
                        args.layout = Some(arg.trim_start_matches("--layout=").to_string());
                    }
                }
            }
        }
        args
    }

    pub fn print_help() {
        println!(
            r#"nourfetch 1.0.0
A fast, lightweight, zero-dependency system information tool.

USAGE:
    nourfetch [OPTIONS]

OPTIONS:
    -h, --help                 Print help information
    -v, --version              Print version information
        --install              Install nourfetch to system PATH
    -t, --theme <NAME>         Select color theme (nour, cyberpunk, dracula, nord, etc.)
    -l, --logo <NAME>          Select ASCII logo (windows11, arch, ubuntu, nour, etc.)
        --layout <LAYOUT>      Select layout style (classic, modern, compact)
        --compact              Render minimal inline summary
        --modern               Render modern boxed card layout
        --json                 Output system specs in JSON format
        --no-color             Disable ANSI color output
        --no-nerd-fonts        Disable Nerd Font icons
        --gen-config           Generate default config.toml file
        --list-themes          List available color themes
        --list-logos           List available ASCII logos

EXAMPLES:
    nourfetch                         # Run with default settings
    nourfetch --install               # Install to system PATH
    nourfetch --theme cyberpunk       # Run with Cyberpunk theme
    nourfetch --modern                # Run with card layout
    nourfetch --compact               # Run in compact mode
    nourfetch --logo arch             # Display Arch Linux logo
    nourfetch --json                  # Output JSON format
    nourfetch --gen-config            # Generate config.toml
"#
        );
    }
}
