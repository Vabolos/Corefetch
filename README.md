# CoreFetch 🚀

![CoreFetch](https://img.shields.io/badge/CoreFetch-v1.0-blue)

CoreFetch is a fun and customizable CLI tool to fetch and display system information in a visually appealing way. Powered by Rust, CoreFetch allows you to see details about your CPU, RAM, OS, battery, disk, and network in an attractive ASCII art format. 🎨

## Features ✨

- Display system information with beautiful ASCII art 🎉
- Customizable alignment and spacing 🛠️
- Optional display of CPU, RAM, OS, battery, disk, and network information 💻🔋📶
- Configurable through a simple TOML file 📜

## Installation 🛠️

To install CoreFetch, you'll need to have Rust installed on your system. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

1. Clone the repository:

   ```sh
   git clone https://github.com/vabolos/corefetch.git
   cd corefetch
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. Move the Executable to a System Path:
   ```sh
   copy target\\release\\corefetch.exe C:\\Users\\YourUsername\\.cargo\\bin\\
   ```

> [!INFO]
> Each time you build/compile the code for a brand new executable, you will have to run the command above to copy it to the system path!

## Usage 📖

To use CoreFetch, simply run `corefetch` in your terminal. CoreFetch will automatically load your configuration from `~/.config/CoreFetch/config.toml` if it exists. If the configuration file is not found, it will use the default settings.

### Configuration ⚙️

_This part is still a WIP!_

You can customize CoreFetch by modifying the `config.toml` file located at `~/.config/CoreFetch/config.toml`. Here is an example configuration:

```toml
# Example config.toml
alignment = "left"  # Options: "left" or "right"
spacing = 2         # Number of spaces between elements
show_cpu = true     # Show CPU information
show_ram = true     # Show RAM information
show_os = true      # Show OS information
show_battery = true # Show battery information
show_disk = true    # Show disk information
show_network = true # Show network information
```

## Contributing 🤝

We welcome contributions! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

1. Fork the repository 🍴
2. Create your feature branch (`git checkout -b feature/new-feature`) 🌟
3. Commit your changes (`git commit -m 'Add some feature'`) 💾
4. Push to the branch (`git push origin feature/new-feature`) 🚀
5. Open a pull request 📬

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements 🙏

- [Rust Programming Language](https://www.rust-lang.org/)
- [Catppuccin Theme](https://github.com/catppuccin)
- [colored crate](https://crates.io/crates/colored)
- [clap crate](https://crates.io/crates/clap)

---

Enjoy using CoreFetch and have fun customizing your system information display! 🎉

