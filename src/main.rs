// Copyright (c) 2026 menndouyukkuri
// SPDX-License-Identifier: MIT OR Apache-2.0


#![windows_subsystem = "windows"]


use std::process::ExitCode;

use explorer_ghost_cleaner::{self, ui};


fn main() -> ExitCode {
	match explorer_ghost_cleaner::run_app() {
		Ok(()) => ExitCode::SUCCESS,
		Err(e) => {
			ui::show_error_dialog(e);
			ExitCode::FAILURE
		}
	}
}
