// Copyright (c) 2026 menndouyukkuri
// SPDX-License-Identifier: MIT OR Apache-2.0


use std::any::Any;

pub(super) fn join_handle_error(err: Box<dyn Any + Send>) -> String {
	if let Some(s) = err.downcast_ref::<&str>() {
		s.to_string()
	} else if let Some(s) = err.downcast_ref::<String>() {
		s.clone()
	} else {
		"Unknown type".to_string()
	}
}
