// vim: set noet tw=4 sw=4

use std::collections::HashMap;

pub mod ipc;
use ipc::*;
use niri_ipc::{Action, LayoutSwitchTarget};

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

	pub fn on_kb_layout_changed(&mut self, keyboard_layouts: &niri_ipc::KeyboardLayouts)
	{
		self.curr_layout_idx = keyboard_layouts.current_idx;
		if let Some(ws_id) = self.curr_ws_idx {
			self.ws_layouts.insert(ws_id, self.curr_layout_idx);
		}
	}

	pub fn on_kb_layout_switched(&mut self, idx: u8)
	{
		self.curr_layout_idx = idx;
		if let Some(ws_id) = self.curr_ws_idx {
			self.ws_layouts.insert(ws_id, self.curr_layout_idx);
		}
	}

	pub fn on_workspaces_changed(&mut self, workspaces: &[niri_ipc::Workspace])
	{
		if let Some(focused) = workspaces.iter().find(|w| w.is_focused) {
			self.curr_ws_idx = Some(focused.id);
		}
	}

	pub fn on_workspace_activated(&mut self, id: u64, focused: bool, client: &mut NiriIPCClient)
	{
		if !focused {
			return;
		}

		match self.ws_layouts.get(&id) {
			Some(&saved) if saved != self.curr_layout_idx => {
				if switch_layout(saved, client) {
					self.curr_layout_idx = saved;
				} else {
					eprintln!("  →  IPC 'switch-layout' failed");
				}
			},
			None => {
				self.ws_layouts.insert(id, self.curr_layout_idx);
			},
			_ => {},
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

pub fn switch_layout(idx: u8, client: &mut NiriIPCClient) -> bool
{
	client.send_action(Action::SwitchLayout {
		layout: LayoutSwitchTarget::Index(idx),
	})
}
