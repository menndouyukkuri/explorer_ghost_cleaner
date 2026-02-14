// Copyright (c) 2026 menndouyukkuri
// SPDX-License-Identifier: MIT OR Apache-2.0


use sysinfo::{System, RefreshKind, ProcessRefreshKind, UpdateKind, ProcessesToUpdate};


pub(super) trait ProcSystemTrait {
	fn refresh_proc(&mut self);
}

impl ProcSystemTrait for System {
	fn refresh_proc(&mut self) {
		let _ = self.refresh_processes_specifics(
			ProcessesToUpdate::All,
			true,
			ProcessRefreshKind::nothing()
				.with_exe(UpdateKind::OnlyIfNotSet)
				.with_cpu()
				.with_disk_usage()
		);
	}
}


pub(super) fn new_proc_system() -> System {
	System::new_with_specifics(
		RefreshKind::nothing().with_processes(
			ProcessRefreshKind::nothing()
		)
	)
}
