/* vim: set noet tw=4 sw=4: */
/**
  * @file: mod.rs - implementation for Niri IPC interface
 */
use niri_ipc::{socket::Socket, Request, Response, Event};

pub struct NiriIPCClient {
	write_socket: Socket,
	read_events: Box<dyn FnMut() -> std::io::Result<niri_ipc::Event>>,
}

impl NiriIPCClient
{
	pub fn new() -> Self
	{
		let mut read_socket: Socket = Socket::connect()
			.expect("Failed to connect to niri socket (read)");
		let reply = read_socket.send(Request::EventStream)
			.expect("Failed to request event stream");
		assert!(
			matches!(reply, Ok(Response::Handled)),
			"Unexpected reply to EventStream request"
		);

		let read_events = Box::new(read_socket.read_events());

		let write_socket = Socket::connect()
			.expect("Failed to connect to niri socket (write)");
		Self {
			write_socket,
			read_events,
		}
	}

	pub fn next_event(&mut self) -> Option<Event>
	{
		(self.read_events)().ok()
	}

	pub fn send_action(&mut self, action: niri_ipc::Action) -> bool
	{
		self.write_socket.send(Request::Action(action))
		.map(|r| r.is_ok())
		.unwrap_or(false)
	}

}

impl Default for NiriIPCClient {
	fn default() -> Self {
		Self::new()
	}
}
