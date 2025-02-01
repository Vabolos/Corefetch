use std::{fs, path::Path};
use toml;
use serde::{Deserialize, Serialize};
use colored::*;
use dirs;
use std::fs::File;
use std::io::Write;
use sysinfo::{Components, Disks, Networks, System, SystemExt, ProcessorExt, DiskExt, NetworkExt}; // Import sysinfo crate

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

fn write_config_to_file() {
    // Create an instance of the configuration
    let config = Config {
        alignment: "left".to_string(),
        spacing: 2,
        show_cpu: true,
        show_ram: true,
        show_os: true,
        show_battery: false,
        show_disk: true,
        show_network: true,
    };

    // Serialize the configuration to TOML format
    let toml_string = toml::to_string(&config).expect("Failed to serialize config");

    // Write the TOML string to the config.toml file
    let mut file = File::create("config.toml").expect("Failed to create config.toml");
    file.write_all(toml_string.as_bytes()).expect("Failed to write to config.toml");
}

fn load_config(config_path: &str) -> Config {
    if Path::new(config_path).exists() {
        let content = fs::read_to_string(config_path).unwrap();
        toml::from_str(&content).unwrap()
    } else {
        Config::default()
    }
}

fn display_system_info(config: &Config) {
    let mut system = System::new_all();
    system.refresh_all();

    if config.show_cpu {
        let cpu_usage = system.global_processor_info().cpu_usage();
        println!("CPU Usage: {:.2}%", cpu_usage);
    }

    if config.show_ram {
        let total_memory = system.total_memory();
        let used_memory = system.used_memory();
        println!("Total Memory: {} bytes", total_memory);
        println!("Used Memory : {} bytes", used_memory);
    }

    if config.show_os {
        let os_name = system.name().unwrap_or_else(|| "Unknown".to_string());
        let os_kernel_version = system.kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let os_version = system.os_version().unwrap_or_else(|| "Unknown".to_string());
        let host_name = system.host_name().unwrap_or_else(|| "Unknown".to_string());

        println!("System Name:             {}", os_name);
        println!("System Kernel Version:   {}", os_kernel_version);
        println!("System OS Version:       {}", os_version);
        println!("System Host Name:        {}", host_name);
    }

    if config.show_battery {
        // sysinfo does not support battery information directly, you may need another crate like `battery`
        println!("Battery: Information not available");
    }

    if config.show_disk {
        for disk in system.disks() {
            println!(
                "Disk: {} - Total: {} bytes, Available: {} bytes",
                disk.name().to_string_lossy(),
                disk.total_space(),
                disk.available_space()
            );
        }
    }

    if config.show_network {
        for (interface_name, data) in system.networks() {
            println!(
                "Network: {} - Received: {} bytes, Transmitted: {} bytes",
                interface_name,
                data.received(),
                data.transmitted()
            );
        }
    }
}

fn main() {
    let config_path = get_config_path();
    if !Path::new(&config_path).exists() {
        generate_config(&config_path);
        write_config_to_file();
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

    display_system_info(&config);
}