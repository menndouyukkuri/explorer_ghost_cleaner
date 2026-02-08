// Copyright (c) 2026 menndouyukkuri
// SPDX-License-Identifier: MIT OR Apache-2.0


use windows::core::{HSTRING, w};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
	MessageBoxW, MESSAGEBOX_STYLE, MESSAGEBOX_RESULT,
	MB_OK, MB_ICONERROR, MB_TOPMOST
};


pub fn show_error_dialog<E: std::fmt::Display>(err: E) {
	let message = format!("Application Error:\n{}", err);

	let _ = show_message_box(
		None, message, MB_OK | MB_ICONERROR | MB_TOPMOST
	);
}


pub(super) fn show_message_box<S: Into<HSTRING>>(
	hwnd: Option<HWND>, message: S, style: MESSAGEBOX_STYLE
) -> MESSAGEBOX_RESULT {
	unsafe {
		MessageBoxW(
			hwnd,
			&message.into(),
			w!("Explorer Ghost Cleaner"),
			style
		)
	}
}
