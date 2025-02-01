use std::{fs, path::Path};
use toml;
use serde::{Deserialize, Serialize};
use colored::*;
use dirs; // Import the dirs crate

#[derive(Serialize, Deserialize)]
struct Config {
    alignment: String, // "left" or "right"
    spacing: usize, // Number of spaces between elements
    show_cpu: bool,
    show_ram: bool,
    show_os: bool,
    show_battery: bool,
    show_disk: bool,
    show_network: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            alignment: "left".to_string(),
            spacing: 2,
            show_cpu: true,
            show_ram: true,
            show_os: true,
            show_battery: true,
            show_disk: true,
            show_network: true,
        }
    }
}

fn get_config_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    format!("{}/.config/CoreFetch/config.toml", home_dir.to_str().unwrap())
}

fn generate_config(config_path: &str) {
    let default_config = Config::default();
    let toml_string = toml::to_string_pretty(&default_config).unwrap();
    let path = Path::new(config_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(config_path, toml_string).unwrap();
    println!("{} {}", "Config file generated at:".yellow().bold(), config_path);
}

fn load_config(config_path: &str) -> Config {
    if Path::new(config_path).exists() {
        let content = fs::read_to_string(config_path).unwrap();
        toml::from_str(&content).unwrap()
    } else {
        Config::default()
    }
}

fn main() {
    let config_path = get_config_path();
    if !Path::new(&config_path).exists() {
        generate_config(&config_path);
    }
    let config = load_config(&config_path);
    println!("{}", "\n   ______                ______     __       __  ".truecolor(245, 224, 220).bold());
    println!("{}", "  / ____/___  ________  / ____/__  / /______/ /_ ".truecolor(242, 205, 205).bold());
    println!("{}", " / /   / __ \\/ ___/ _ \\/ /_  / _ \\/ __/ ___/ __ \\".truecolor(245, 194, 231).bold());
    println!("{}", "/ /___/ /_/ / /  /  __/ __/ /  __/ /_/ /__/ / / /".truecolor(203, 166, 247).bold());
    println!("{}", "\\____/\\____/_/   \\___/_/    \\___/\\__/\\___/_/ /_/".truecolor(243, 139, 168).bold());
    println!("{}", "\n".truecolor(235, 160, 172).bold());
    println!("Alignment: {}", config.alignment);
    println!("Spacing: {}", config.spacing);
    // Fetch and display system info here
}