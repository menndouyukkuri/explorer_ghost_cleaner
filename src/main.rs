// Copyright (c) 2026 menndouyukkuri
// SPDX-License-Identifier: MIT OR Apache-2.0


#![windows_subsystem = "windows"]


use std::env;
use std::process::ExitCode;

use explorer_ghost_cleaner::{self, args, ui};


fn main() -> ExitCode {
	let args: Vec<String> = env::args_os()
		.map(|os_str| os_str.to_string_lossy().into_owned())
		.collect();

	if args::process_args(args) {
		return ExitCode::SUCCESS;
	}

	match explorer_ghost_cleaner::run_app() {
		Ok(()) => ExitCode::SUCCESS,
		Err(e) => {
			ui::show_error_dialog(e);
			ExitCode::FAILURE
		}
	}
}
