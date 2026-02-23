// Copyright (c) 2026 menndouyukkuri
// SPDX-License-Identifier: MIT OR Apache-2.0


use std::collections::HashSet;

use sysinfo::System;

use crate::Config;


mod os;


#[derive(Hash, Eq, PartialEq)]
struct ProcessKey {
	pid: u32,
	start_time: u64,
}


pub struct Inspector {
	prev_hidden: HashSet<ProcessKey>,
}

impl Inspector {
	pub(super) fn new() -> Self {
		Self { prev_hidden: HashSet::new() }  // Previously hidden processes
	}


	pub(super) fn scan_and_clean(&mut self, sys: &mut System, config: &Config) {
		// New set of hidden processes
		let mut new_hidden: HashSet<ProcessKey> = HashSet::new();

		// Get excluded processes
		let shell_pid = os::get_shell_pid().unwrap_or(0);
		let visible_pids = os::get_visible_pids();

		// Scan processes loop
		for (pid, process) in sys.processes() {
			// Convert Pid to u32
			let pid_u32 = pid.as_u32();

			// If it's an excluded process, skip it
			if is_excluded_process(process, pid_u32, shell_pid, &visible_pids) {
				continue;
			}

			// Evaluate the process
			evaluate_process(process, pid_u32, config, &self.prev_hidden, &mut new_hidden);
		}

		// Update previously hidden processes
		self.prev_hidden = new_hidden;
	}
}


fn evaluate_process(
	process: &sysinfo::Process,
	pid_u32: u32,
	config: &Config,
	prev_hidden: &HashSet<ProcessKey>,
	new_hidden: &mut HashSet<ProcessKey>
) {
	// Create key for the process
	let key = ProcessKey {
		pid: pid_u32,
		start_time: process.start_time(),
	};

	// If it was already a hidden process in the previous check
	// Only processes that stay hidden for two consecutive checks are eligible for termination
	if prev_hidden.contains(&key) {
		if is_idle(process, config) {  // Check if the process is idle
			let _ = process.kill();  // Skip it if it fails to kill
		}
	} else {  // If it's a new process
		new_hidden.insert(key);  // Add it to the new set
	}
}


fn is_excluded_process(process: &sysinfo::Process, pid_u32: u32, shell_pid: u32, visible_pids: &HashSet<u32>) -> bool {
	// If it's not an explorer.exe process
	!process.name().eq_ignore_ascii_case("explorer.exe")

	||

	// If it's the shell process or a visible process
	pid_u32 == shell_pid || visible_pids.contains(&pid_u32)
}


fn is_idle(process: &sysinfo::Process, config: &Config) -> bool {
	process.cpu_usage()
		<= config.cpu_threshold
	&&
	process.disk_usage().read_bytes
		<= config.disk_threshold_bytes
	&&
	process.disk_usage().written_bytes
		<= config.disk_threshold_bytes
}
