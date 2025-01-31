use std::{env, fs, path::Path};
use toml;
use serde::{Deserialize, Serialize};
use colored::*;

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

fn config_path() -> String {
    let username = env::var("USERNAME").unwrap();
    format!("C:/Users/{}/.config/Corefetch/config.toml", username)
}

fn generate_config() {
    let default_config = Config::default();
    let toml_string = toml::to_string_pretty(&default_config).unwrap();
    let path = Path::new(&config_path());
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&config_path(), toml_string).unwrap();
    println!("{}", "Config file generated at C:/Users/<USERNAME>/.config/Corefetch/config.toml".green());
}

fn load_config() -> Config {
    let path = Path::new(&config_path());
    if path.exists() {
        let content = fs::read_to_string(path).unwrap();
        toml::from_str(&content).unwrap()
    } else {
        generate_config();
        Config::default()
    }
}

fn main() {
    let config = load_config();
    println!("{}", "\n    ██████╗ ██████╗ ███████╗███████╗████████╗███████╗ ██████╗██╗  ██╗".truecolor(245, 224, 220).bold());
    println!("{}", "    ██╔══██╗██╔══██╗██╔════╝██╔════╝╚══██╔══╝██╔════╝██╔════╝██║  ██║".truecolor(242, 205, 205).bold());
    println!("{}", "    ██████╔╝██████╔╝█████╗  ███████╗   ██║   █████╗  ██║     ███████║".truecolor(245, 194, 231).bold());
    println!("{}", "    ██╔═══╝ ██╔══██╗██╔══╝  ╚════██║   ██║   ██╔══╝  ██║     ██╔══██║".truecolor(203, 166, 247).bold());
    println!("{}", "    ██║     ██║  ██║███████╗███████║   ██║   ███████╗╚██████╗██║  ██║".truecolor(243, 139, 168).bold());
    println!("{}", "    ╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝   ╚═╝   ╚══════╝ ╚═════╝╚═╝  ╚═╝\n".truecolor(235, 160, 172).bold());
    println!("Alignment: {}", config.alignment);
    println!("Spacing: {}", config.spacing);
    // Fetch and display system info here
}