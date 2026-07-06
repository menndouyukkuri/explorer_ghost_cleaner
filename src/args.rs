// Copyright (c) 2026 menndouyukkuri
// SPDX-License-Identifier: MIT OR Apache-2.0


use super::ui;

use windows::Win32::UI::WindowsAndMessaging::{
	MB_OK, MB_ICONINFORMATION, MB_ICONQUESTION
};


pub fn process_args(args: Vec<String>) -> bool {
	if args.len() <= 1 {
		return false
	}

	match args[1].as_str() {
		"-v" | "-V" | "--version" => print_version(),
		"-h" | "--help" => print_help(args),
		_ => false,
	}
}


fn print_version() -> bool {
	let message = format!("   Version {}
   Copyright (c) 2026 {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));

	let _ = ui::show_message_box(
		None, message, MB_OK | MB_ICONINFORMATION
	);

	true
}


fn print_help(args: Vec<String>) -> bool {
	let program_name = if args[0].is_empty() {
		concat!(env!("CARGO_PKG_NAME"), ".exe")
	} else {
		&args[0]
	};

	let message = format!("   Usage: {} [OPTIONS]
   Options:
       -h, --help       Print this help message
       -v, --version    Print the version number", program_name);

	let _ = ui::show_message_box(
		None, message, MB_OK | MB_ICONQUESTION
	);

	true
}
