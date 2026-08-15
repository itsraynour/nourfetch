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
        println!("nourfetch v1.0.0 (Rust - Zero Dependencies)");
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

    let from_explorer = is_launched_from_explorer();
    if from_explorer {
        perform_self_install(true);
    }

    let mut config = Config::load();

    if let Some(theme_name) = args.theme {
        config.theme = theme_name;
    }
    if let Some(logo_name) = args.logo {
        config.logo = logo_name;
    }
    if let Some(layout_name) = args.layout {
        config.layout = layout_name;
    }
    if args.compact {
        config.layout = "compact".to_string();
    }
    if args.modern {
        config.layout = "modern".to_string();
    }
    if args.no_nerd_fonts {
        config.nerd_fonts = false;
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
