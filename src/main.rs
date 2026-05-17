// vim: set noet tw=4 sw=4

use npwskl::*;
use npwskl::nirievt::{NiriEvent, NiriEvtReader};

fn event_loop(reader: &mut nirievt::NiriEvtReader, state: &mut PwsklState)
{
	println!("Listening to Niri events...");
	for event in reader {
		match event {
			NiriEvent::KeyboardLayoutsChanged(payload) => state.on_kb_layout_changed(&payload),
			NiriEvent::KeyboardLayoutSwitched { idx } => state.on_kb_layout_switched(idx),
			NiriEvent::WorkspacesChanged(payload) => state.on_workspaces_changed(&payload),
			NiriEvent::WorkspaceActivated { id, focused } => state.on_workspace_activated(id, focused),
		}
	}
	println!("Event loop ended.");
}

fn main()
{
	let mut state = PwsklState::new();
	let mut reader = NiriEvtReader::new();

	event_loop(&mut reader, &mut state);
}
