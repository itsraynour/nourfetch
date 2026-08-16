#![allow(warnings)]
#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod cli;
mod config;
mod logos;
mod sys;
mod theme;
mod ui;
mod utils;

use cli::CliArgs;
use config::Config;
use logos::Logo;
use sys::SystemInfo;
use theme::Theme;


#[cfg(windows)]
fn is_launched_from_explorer() -> bool {
    #[link(name = "kernel32")]
    extern "system" {
        fn GetConsoleProcessList(lpdwProcessList: *mut u32, dwProcessCount: u32) -> u32;
    }
    let mut process_list = [0u32; 2];
    let count = unsafe { GetConsoleProcessList(process_list.as_mut_ptr(), 2) };
    count <= 1
}

#[cfg(not(windows))]
fn is_launched_from_explorer() -> bool {
    false
}

fn perform_self_install(silent: bool) {
    let current_exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            if !silent {
                eprintln!("Error: Failed to locate executable: {}", e);
            }
            return;
        }
    };

    #[cfg(windows)]
    {
        if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
            let dest_dir = std::path::PathBuf::from(local_appdata).join("Microsoft").join("WindowsApps");
            let dest_file = dest_dir.join("nourfetch.exe");
            
            if current_exe != dest_file {
                match std::fs::copy(&current_exe, &dest_file) {
                    Ok(_) => {
                        if !silent {
                            println!("Installed nourfetch to {}", dest_file.display());
                            println!("Run 'nourfetch' from any command prompt or terminal.");
                        }
                    }
                    Err(e) => {
                        if !silent {
                            eprintln!("Error installing to {}: {}", dest_file.display(), e);
                        }
                    }
                }
            } else if !silent {
                println!("nourfetch is already installed in system PATH ({})", dest_file.display());
            }
        }
    }

    #[cfg(not(windows))]
    {
        let home = std::env::var("HOME").unwrap_or_default();
        let dest_dir = if !home.is_empty() {
            let d = std::path::PathBuf::from(home).join(".local").join("bin");
            let _ = std::fs::create_dir_all(&d);
            d
        } else {
            std::path::PathBuf::from("/usr/local/bin")
        };
        let dest_file = dest_dir.join("nourfetch");

        if current_exe != dest_file {
            match std::fs::copy(&current_exe, &dest_file) {
                Ok(_) => {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let _ = std::fs::set_permissions(&dest_file, std::fs::Permissions::from_mode(0o755));
                    }
                    if !silent {
                        println!("Installed nourfetch to {}", dest_file.display());
                        println!("Run 'nourfetch' in your terminal.");
                    }
                }
                Err(e) => {
                    if !silent {
                        eprintln!("Error installing to {}: {}", dest_file.display(), e);
                    }
                }
            }
        } else if !silent {
            println!("nourfetch is already installed in system PATH ({})", dest_file.display());
        }
    }
}

fn main() {
    let args = CliArgs::parse();

    if args.help {
        CliArgs::print_help();
        return;
    }

    if args.version {
        println!("nourfetch v{} (Rust - Zero Dependencies)", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.install {
        perform_self_install(false);
        return;
    }

    if args.list_themes {
        println!("Available Themes in nourfetch:");
        for t in Theme::list_all() {
            println!("  • {}", t);
        }
        return;
    }

    if args.list_logos {
        println!("Available ASCII Logos in nourfetch:");
        for l in Logo::list_all() {
            println!("  • {}", l);
        }
        return;
    }

    if args.gen_config {
        match Config::generate_default_file() {
            Ok(path) => {
                println!("Configuration file generated at: {}", path.display());
            }
            Err(e) => {
                eprintln!("Error: Failed to generate configuration: {}", e);
            }
        }
        return;
    }

    if let Some(cmd) = args.config_cmd {
        match cmd {
            cli::ConfigCommand::Path => {
                if let Some(path) = Config::get_config_path() {
                    println!("Configuration file path: {}", path.display());
                } else {
                    eprintln!("Error: Unable to resolve configuration path.");
                }
                return;
            }
            cli::ConfigCommand::Reset => {
                match Config::reset() {
                    Ok(path) => {
                        println!("✔ Configuration has been reset to defaults ({})", path.display());
                    }
                    Err(e) => {
                        eprintln!("Error resetting configuration: {}", e);
                    }
                }
                return;
            }
            cli::ConfigCommand::Get(key_opt) => {
                let config = Config::load();
                if let Some(key) = key_opt {
                    match key.to_lowercase().as_str() {
                        "theme" => println!("theme = \"{}\"", config.theme),
                        "logo" => println!("logo = \"{}\"", config.logo),
                        "layout" => println!("layout = \"{}\"", config.layout),
                        "nerd_fonts" | "icons" => println!("nerd_fonts = {}", config.nerd_fonts),
                        "color_blocks" => println!("color_blocks = {}", config.color_blocks),
                        "show_bars" => println!("show_bars = {}", config.show_bars),
                        "bar_style" => println!("bar_style = \"{}\"", config.bar_style),
                        "bar_width" => println!("bar_width = {}", config.bar_width),
                        other => eprintln!("Unknown configuration key: '{}'", other),
                    }
                } else {
                    let path_str = Config::get_config_path().map(|p| p.display().to_string()).unwrap_or_default();
                    println!("Current configuration ({}):", path_str);
                    println!("  theme = \"{}\"", config.theme);
                    println!("  logo = \"{}\"", config.logo);
                    println!("  layout = \"{}\"", config.layout);
                    println!("  nerd_fonts = {}", config.nerd_fonts);
                    println!("  color_blocks = {}", config.color_blocks);
                    println!("  show_bars = {}", config.show_bars);
                    println!("  bar_style = \"{}\"", config.bar_style);
                    println!("  bar_width = {}", config.bar_width);
                }
                return;
            }
            cli::ConfigCommand::Set(key, val) => {
                let mut config = Config::load();
                let key_lower = key.to_lowercase();
                match key_lower.as_str() {
                    "theme" => config.theme = val.clone(),
                    "logo" => config.logo = val.clone(),
                    "layout" => config.layout = val.clone(),
                    "nerd_fonts" | "icons" => {
                        config.nerd_fonts = val.parse().unwrap_or_else(|_| !val.eq_ignore_ascii_case("false") && !val.eq_ignore_ascii_case("0") && !val.eq_ignore_ascii_case("no"));
                    }
                    "color_blocks" => {
                        config.color_blocks = val.parse().unwrap_or(true);
                    }
                    "show_bars" => {
                        config.show_bars = val.parse().unwrap_or(true);
                    }
                    "bar_style" => config.bar_style = val.clone(),
                    "bar_width" => config.bar_width = val.parse().unwrap_or(14),
                    other => {
                        eprintln!("Error: Unknown configuration property '{}'", other);
                        return;
                    }
                }
                match config.save() {
                    Ok(path) => {
                        println!("✔ Successfully saved '{} = {}' permanently to {}", key, val, path.display());
                    }
                    Err(e) => {
                        eprintln!("Error saving configuration: {}", e);
                    }
                }
                return;
            }
        }
    }

    let from_explorer = is_launched_from_explorer();
    if from_explorer {
        perform_self_install(true);
    }

    let mut config = Config::load();
    let mut changed = false;

    if let Some(theme_name) = args.theme {
        config.theme = theme_name;
        changed = true;
    }
    if let Some(logo_name) = args.logo {
        config.logo = logo_name;
        changed = true;
    }
    if let Some(layout_name) = args.layout {
        config.layout = layout_name;
        changed = true;
    }
    if args.compact {
        config.layout = "compact".to_string();
        changed = true;
    }
    if args.modern {
        config.layout = "modern".to_string();
        changed = true;
    }
    if args.classic {
        config.layout = "classic".to_string();
        changed = true;
    }
    if let Some(nf) = args.nerd_fonts {
        config.nerd_fonts = nf;
        changed = true;
    } else if args.no_nerd_fonts {
        config.nerd_fonts = false;
        changed = true;
    }

    if args.save && changed {
        match config.save() {
            Ok(path) => {
                println!("✔ Settings saved permanently to {}", path.display());
            }
            Err(e) => {
                eprintln!("Warning: Failed to save permanent configuration: {}", e);
            }
        }
    }

    let color_enabled = !args.no_color;
    let info = SystemInfo::fetch();

    if args.json {
        ui::json::render_json(&info);
        return;
    }

    let theme = Theme::from_name(&config.theme);
    let logo_key = if config.logo != "auto" {
        config.logo.as_str()
    } else {
        info.os_key.as_str()
    };
    let logo = Logo::resolve(logo_key);

    match config.layout.to_lowercase().as_str() {
        "modern" | "card" | "box" => {
            ui::modern::render(&info, &logo, &theme, &config, color_enabled);
        }
        "compact" | "mini" => {
            ui::compact::render(&info, &theme, &config, color_enabled);
        }
        _ => {
            ui::classic::render(&info, &logo, &theme, &config, color_enabled);
        }
    }

    if from_explorer {
        println!();
        println!("  Press Enter to exit...");
        let mut buffer = String::new();
        let _ = std::io::stdin().read_line(&mut buffer);
    }
}
