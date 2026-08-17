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

fn perform_self_uninstall(force: bool) {
    if !force {
        use std::io::Write;
        print!("Are you sure you want to completely uninstall nourfetch and remove all configuration files? [y/N]: ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Aborted.");
            return;
        }
        let trimmed = input.trim();
        if !trimmed.eq_ignore_ascii_case("y") && !trimmed.eq_ignore_ascii_case("yes") {
            println!("Uninstallation cancelled.");
            return;
        }
    }

    println!("\nUninstalling nourfetch...\n");
    let mut removed_anything = false;

    let mut config_paths: Vec<std::path::PathBuf> = Vec::new();
    #[cfg(windows)]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            config_paths.push(std::path::PathBuf::from(appdata).join("nourfetch"));
        }
        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            config_paths.push(std::path::PathBuf::from(userprofile).join(".config").join("nourfetch"));
        }
    }

    #[cfg(not(windows))]
    {
        if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            config_paths.push(std::path::PathBuf::from(xdg_config).join("nourfetch"));
        }
        if let Ok(home) = std::env::var("HOME") {
            config_paths.push(std::path::PathBuf::from(home).join(".config").join("nourfetch"));
        }
    }

    for path in config_paths {
        if path.exists() {
            if path.is_dir() {
                match std::fs::remove_dir_all(&path) {
                    Ok(_) => {
                        println!("  Removed directory: {}", path.display());
                        removed_anything = true;
                    }
                    Err(e) => {
                        eprintln!("  Error: Failed to remove directory {}: {}", path.display(), e);
                    }
                }
            } else {
                match std::fs::remove_file(&path) {
                    Ok(_) => {
                        println!("  Removed file: {}", path.display());
                        removed_anything = true;
                    }
                    Err(e) => {
                        eprintln!("  Error: Failed to remove file {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    let current_exe = std::env::current_exe().ok();
    let mut bin_paths: Vec<std::path::PathBuf> = Vec::new();

    #[cfg(windows)]
    {
        if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
            bin_paths.push(std::path::PathBuf::from(local_appdata).join("Microsoft").join("WindowsApps").join("nourfetch.exe"));
        }
        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            bin_paths.push(std::path::PathBuf::from(userprofile).join(".cargo").join("bin").join("nourfetch.exe"));
        }
    }

    #[cfg(not(windows))]
    {
        if let Ok(home) = std::env::var("HOME") {
            let home_path = std::path::PathBuf::from(home);
            bin_paths.push(home_path.join(".local").join("bin").join("nourfetch"));
            bin_paths.push(home_path.join(".cargo").join("bin").join("nourfetch"));
        }
        bin_paths.push(std::path::PathBuf::from("/usr/local/bin").join("nourfetch"));
    }

    let mut current_exe_scheduled_on_windows = false;

    for path in bin_paths {
        if path.exists() {
            let is_current = current_exe.as_ref().map(|c| c == &path).unwrap_or(false);
            if is_current {
                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    let exe_str = path.display().to_string();
                    let _ = std::process::Command::new("cmd")
                        .args(&["/C", &format!("ping 127.0.0.1 -n 2 > nul & del /F /Q \"{}\"", exe_str)])
                        .creation_flags(0x08000000)
                        .spawn();
                    println!("  Removed executable: {}", path.display());
                    current_exe_scheduled_on_windows = true;
                    removed_anything = true;
                }
                #[cfg(not(windows))]
                {
                    match std::fs::remove_file(&path) {
                        Ok(_) => {
                            println!("  Removed executable: {}", path.display());
                            removed_anything = true;
                        }
                        Err(e) => {
                            eprintln!("  Error: Failed to remove binary {}: {}", path.display(), e);
                        }
                    }
                }
            } else {
                match std::fs::remove_file(&path) {
                    Ok(_) => {
                        println!("  Removed executable: {}", path.display());
                        removed_anything = true;
                    }
                    Err(e) => {
                        #[cfg(not(windows))]
                        if path.starts_with("/usr/local/bin") {
                            eprintln!("  Notice: Permission denied removing {}. Run: sudo rm {}", path.display(), path.display());
                            continue;
                        }
                        eprintln!("  Error: Failed to remove binary {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    if let Some(ref cur) = current_exe {
        if cur.exists() {
            #[cfg(windows)]
            {
                if !current_exe_scheduled_on_windows {
                    use std::os::windows::process::CommandExt;
                    let exe_str = cur.display().to_string();
                    let _ = std::process::Command::new("cmd")
                        .args(&["/C", &format!("ping 127.0.0.1 -n 2 > nul & del /F /Q \"{}\"", exe_str)])
                        .creation_flags(0x08000000)
                        .spawn();
                    println!("  Removed binary: {}", cur.display());
                    removed_anything = true;
                }
            }
            #[cfg(not(windows))]
            {
                if cur.file_name().and_then(|n| n.to_str()).map(|n| n.starts_with("nourfetch")).unwrap_or(false) {
                    if let Ok(_) = std::fs::remove_file(cur) {
                        println!("  Removed binary: {}", cur.display());
                        removed_anything = true;
                    }
                }
            }
        }
    }

    if removed_anything {
        println!("\nnourfetch has been completely uninstalled.");
    } else {
        println!("\nNo installed files or configurations found.");
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

    if args.uninstall {
        perform_self_uninstall(args.force);
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
                        println!("Configuration reset to defaults ({})", path.display());
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
                        println!("Saved '{} = {}' to {}", key, val, path.display());
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
                println!("Settings saved to {}", path.display());
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
