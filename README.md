(English / [日本語](docs/README-ja.md))

# Explorer Ghost Cleaner

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows-blue)](#requirements)

A lightweight Windows tray application that automatically cleans up "ghost" `explorer.exe` processes.

When the "Launch folder windows in a separate process" option is enabled, Windows sometimes leaves idle Explorer processes in memory even after all windows are closed. This tool detects and terminates these idle processes to reclaim system resources.

## Features

* **🛡️ Smart & Safe Detection**\
Specifically targets **background** Explorer processes **without active windows**. It **never** interferes with your Desktop, Taskbar, or any active folder windows.
* **📊 Resource-Aware Cleaning**\
Only terminates processes that are truly idle. The application monitors CPU and Disk I/O to ensure no active tasks (like file copying) are interrupted.
* **🚀 Lightweight & Standalone**\
Written in Rust for maximum efficiency. It's a single executable with **zero dependencies**—no .NET or runtime installation required.
* **⚙️ Fully Customizable**\
Fine-tune the scan intervals and "idle" thresholds (CPU/Disk) via a simple `config.toml` file to match your system's needs.

## Requirements

* Windows 10 / 11

### Important

The application is designed for users who have enabled the following Windows setting:

1. Open **File Explorer Options**.
2. Go to the **View** tab.
3. Check **"Launch folder windows in a separate process"**.

(Note: If this setting is disabled, the application is not needed, as all windows will run within the system shell process.)

## Setup (x64)

1. **Download**: Get the latest zip file from the [releases page](https://github.com/menndouyukkuri/explorer_ghost_cleaner/releases/latest).
2. **Extract**: Unzip the archive to any folder of your choice.
3. **Run**: Double-click `explorer_ghost_cleaner.exe`. It will run in the background.
   * To close the app, right-click the tray icon and select **Exit**.
   * Does not require administrator privileges.

### Run at Startup

To have the application start automatically with Windows:

1. Right-click `explorer_ghost_cleaner.exe` and select **Create shortcut**.
2. Open File Explorer and paste `shell:startup` into the address bar.
3. Move the shortcut you created into that folder.

### Uninstall

To uninstall, simply delete the executable and configuration file.

If you created a shortcut in your `shell:startup` folder, delete that as well.

## Configuration

The application includes a `config.toml` file. You can modify it with any text editor to tune the behavior:

```toml
# Scan interval for idle processes (seconds)
interval_seconds = 300 # default: 300 (5min)

# Idle thresholds: Process is terminated if usage is below these values
# CPU usage (%)
cpu_threshold = 1.0 # default: 1.0
# Disk I/O bytes
disk_threshold_bytes = 102_400 # default: 102_400 (100KiB)
```

## Build

If you prefer to build from source, ensure you have Rust and Cargo (can be installed with [Rustup](https://rustup.rs/)) installed, then run:

```cmd
cargo build --release
```

## License

**Use at your own risk.**

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
