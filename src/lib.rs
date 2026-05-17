// vim: set noet tw=4 sw=4

use std::collections::HashMap;

pub mod nirievt;
use nirievt::{
	KeyboardLayoutsChanged,
	WorkspacesChanged,
};

pub struct PwsklState {
	ws_layouts: HashMap<u64, u8>,
	curr_layout_idx: u8,
	curr_ws_idx: Option<u64>,
}

impl PwsklState
{
	pub fn new() -> Self
	{
		Self {
			ws_layouts: HashMap::new(),
			curr_layout_idx: 0,
			curr_ws_idx: None,
		}
	}

	pub fn on_kb_layout_changed(&mut self, payload: &KeyboardLayoutsChanged)
	{
		self.curr_layout_idx = payload.keyboard_layouts.current_idx;
		println!("Layout changed → idx {}", self.curr_layout_idx);
		if let Some(ws_id) = self.curr_ws_idx {
			self.ws_layouts.insert(ws_id, self.curr_layout_idx);
		}
	}

	pub fn on_kb_layout_switched(&mut self, idx: u8)
	{
		self.curr_layout_idx = idx;
		println!("[LayoutSwitched] idx={idx}, curr_ws={:?}", self.curr_ws_idx);
		if let Some(ws_id) = self.curr_ws_idx {
			self.ws_layouts.insert(ws_id, self.curr_layout_idx);
		}
	}

	pub fn on_workspaces_changed(&mut self, payload: &WorkspacesChanged)
	{
		if let Some(focused) = payload.workspaces.iter().find(|w| w.is_focused) {
			self.curr_ws_idx = Some(focused.id);
			println!("[WorkspacesChanged] initial focused ws={}", focused.id);
		}
	}

	pub fn on_workspace_activated(&mut self, id: u64, focused: bool)
	{
		println!("[WorkspaceActivated] id={id}, focused={focused}");
		println!(
			"  curr_layout_idx={}, ws_layouts={:?}",
			self.curr_layout_idx, self.ws_layouts
		);

		if !focused {
			return;
		}

		match self.ws_layouts.get(&id) {
			Some(&saved) if saved != self.curr_layout_idx => {
				println!(
					"  → RESTORING layout {saved} (currently {})",
					self.curr_layout_idx
				);
				if nirievt::call_ipc("switch-layout", &saved.to_string()) {
					self.curr_layout_idx = saved;
				} else {
					eprintln!("  →  IPC 'switch-layout' failed");
				}
			}
			Some(&saved) => {
				println!("  → layout already correct ({saved}), nothing to do");
			}
			None => {
				self.ws_layouts.insert(id, self.curr_layout_idx);
				println!(
					"  → first visit, inheriting layout {}",
					self.curr_layout_idx
				);
			}
		}

		self.curr_ws_idx = Some(id);
	}
}

impl Default for PwsklState
{
	fn default() -> Self
	{
		Self::new()
	}
}
