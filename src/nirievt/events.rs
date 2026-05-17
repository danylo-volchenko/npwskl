 /* vim: set noet tw=4 sw=4: */
 /**
  * @file: events.rs - definitions of monitored events and their json representation
  */
use serde::Deserialize;

/// ── Keyboard ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct KeyboardLayouts {
	pub current_idx: u8,
	#[serde(skip)] pub names: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct KeyboardLayoutsChanged {
	pub keyboard_layouts: KeyboardLayouts,
}

/// ── Workspace ───────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub id: u64,
    pub is_focused: bool,
}

#[derive(Debug, Deserialize)]
pub struct WorkspacesChanged {
    pub workspaces: Vec<Workspace>,
}

/// ── Top-level event enum ────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum NiriEvent {
	KeyboardLayoutsChanged(KeyboardLayoutsChanged),
	KeyboardLayoutSwitched { idx: u8 },
	WorkspacesChanged(WorkspacesChanged),
	WorkspaceActivated {id: u64, focused: bool},
}

 /// --- Impl ------------------------------------------------------------------
impl NiriEvent
{
	/// Keys to handle —  used to filter the stream before full parse
	pub const KNOWN: &'static [&'static str] = &[
		"KeyboardLayoutsChanged",
		"KeyboardLayoutSwitched",
		"WorkspacesChanged",
		"WorkspaceActivated",
	];
}
