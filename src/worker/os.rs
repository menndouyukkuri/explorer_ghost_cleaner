// Copyright (c) 2026 menndouyukkuri
// SPDX-License-Identifier: MIT OR Apache-2.0


use std::collections::HashSet;

use windows::core::BOOL;
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
	EnumWindows, GetShellWindow, GetWindowThreadProcessId, IsWindowVisible,
};


pub(super) fn get_shell_pid() -> Option<u32> {
	let hwnd = unsafe { GetShellWindow() };
	if hwnd.is_invalid() { return None; }
	let mut pid = 0;
	let _ = unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
	(pid != 0).then_some(pid)
}


pub(super) fn get_visible_pids() -> HashSet<u32> {
	let mut pids = HashSet::new();
	unsafe {
		let _ = EnumWindows(Some(enum_window_proc), LPARAM(&mut pids as *mut HashSet<u32> as isize));
	}
	pids
}


unsafe extern "system" fn enum_window_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
	let pids = unsafe { &mut *(lparam.0 as *mut HashSet<u32>) };

	if unsafe { IsWindowVisible(hwnd) }.as_bool() {
		let mut pid = 0;
		let _ = unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
		if pid != 0 { pids.insert(pid); };
	};

	BOOL(1)
}
