// Copyright (c) 2026 menndouyukkuri
// SPDX-License-Identifier: MIT OR Apache-2.0


use std::fs;
use std::thread;
use std::sync::mpsc;
use std::error::Error;
use std::time::Duration;

use serde::Deserialize;
use tray_item::{TrayItem, IconSource};


pub mod ui;

mod format;
mod system_ext;
mod worker;


#[derive(Deserialize)]
struct Config {
	interval_seconds: u64,
	cpu_threshold: f32,
	disk_threshold_bytes: u64,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			interval_seconds: 300,
			cpu_threshold: 1.0,
			disk_threshold_bytes: 102_400,
		}
	}
}


pub fn run_app() -> Result<(), Box<dyn Error>> {
	// Load config
	let config = load_config();

	// Channels
	let (main_exit_send, main_exit_recv) = mpsc::channel();
	let (worker_exit_send, worker_exit_recv) = mpsc::channel();

	// Worker thread (terminates on Ok(()) or channel disconnection)
	let worker_handle = thread::spawn(move || {
		run_worker(config, worker_exit_recv);
		let _ = main_exit_send.send(());  // Even if a panic occurs, the main thread can terminate because 'main_exit_send' is dropped
	});

	// Tray icon
	let mut tray = TrayItem::new("Explorer Ghost Cleaner", IconSource::Resource("#1"))
		.map_err(|e| format!("Failed to create tray icon: {}", e))?;

	tray.add_menu_item("Exit", move || {
		let _ = worker_exit_send.send(());
	}).map_err(|e| format!("Failed to add menu item: {}", e))?;

	// Wait for main exit signal
	let _ = main_exit_recv.recv();  // Ignore Err(_) here; errors are caught by worker_handle

	// Wait for the worker thread to finish
	worker_handle.join().map_err(|e| format!(
		"Worker thread panicked: {}", format::join_handle_error(e)
	))?;

	Ok(())
}


fn load_config() -> Config {
	fs::read_to_string("config.toml")
		.ok()
		.and_then(|s| toml::from_str(&s).ok())
		.unwrap_or_default()
}


fn run_worker(config: Config, exit_recv: mpsc::Receiver<()>) {
	use system_ext::ProcSystemTrait;
	use worker::Inspector;

	// Initialize system
	let mut sys = system_ext::new_proc_system();

	// Initialize inspector
	let mut inspector = Inspector::new();

	// Main loop
	loop {
		// Refresh system information
		sys.refresh_proc();

		// Scan and clean processes
		inspector.scan_and_clean(&mut sys, &config);

		// Wait for the next interval or exit signal
		match exit_recv.recv_timeout(Duration::from_secs(config.interval_seconds)) {
			// If an exit signal is received or the channel is disconnected, break the loop
			Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

			// If a timeout occurs, continue to the next iteration
			Err(mpsc::RecvTimeoutError::Timeout) => continue
		};
	}
}
