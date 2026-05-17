// vim: set noet tw=4 sw=4

use npwskl::*;
use npwskl::ipc::{NiriIPCClient};
use niri_ipc::Event;

fn event_loop(client: &mut ipc::NiriIPCClient, state: &mut PwsklState)
{
	println!("Listening to Niri events...");
	while let Some(event) = client.next_event() {
		match event {
			Event::KeyboardLayoutsChanged { keyboard_layouts } => { state.on_kb_layout_changed(&keyboard_layouts); }
			Event::KeyboardLayoutSwitched { idx } => { state.on_kb_layout_switched(idx); }
			Event::WorkspacesChanged { workspaces } => { state.on_workspaces_changed(&workspaces); }
			Event::WorkspaceActivated { id, focused } => { state.on_workspace_activated(id, focused, client);}
			_ => { },
		}
	}
	println!("Event loop ended.");
}

fn main()
{
	let mut state = PwsklState::new();
	let mut client = NiriIPCClient::new();

	event_loop(&mut client, &mut state);
}
