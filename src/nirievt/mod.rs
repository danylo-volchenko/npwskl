/* vim: set noet tw=4 sw=4: */
/**
  * @file: mod.rs - implementation for Niri events handler
 */
mod events;
use std::{
	io::{
		BufRead,
		BufReader,
	},
	process::{
		Child,
		ChildStdout,
		Command,
		Stdio,
	},
};

pub use events::{
	NiriEvent,
	KeyboardLayoutsChanged,
    WorkspacesChanged,
};

pub struct NiriEvtReader {
	proc: Child,
	reader: BufReader<ChildStdout>,
}

impl NiriEvtReader
{
	pub fn new() -> Self
	{
		let mut child = Command::new("niri")
			.args(["msg", "-j", "event-stream"])
			.stdout(Stdio::piped())
			.spawn()
			.expect("Failed to start niri event-stream");

		let stdout = child.stdout.take().expect("Failed to capture stdout");
		Self {
			proc: child,
			reader: BufReader::new(stdout),
		}
	}

	pub fn next_event(&mut self) -> Option<NiriEvent>
	{
		loop {
			let mut line = String::new();
			match self.reader.read_line(&mut line) {
				Ok(0) => return None,
				Ok(_) => {}
				Err(e) => {
					eprintln!("Read error: {e}");
					return None;
				}
			}

			let trimmed = line.trim();
			let Ok(raw) = serde_json::from_str::<serde_json::Value>(trimmed) else {
				eprintln!("Invalid JSON: {trimmed}");
				continue;
			};

			let is_known = raw.as_object().and_then(|o| o.keys().next())
				.map(|k| NiriEvent::KNOWN.contains(&k.as_str())).unwrap_or(false);

			if !is_known {
				continue;
			}

			match serde_json::from_value::<NiriEvent>(raw) {
				Ok(evt) => return Some(evt),
				Err(e) => eprintln!("Parse error: {e}"),
			}
		}
	}

	pub fn shutdown(&mut self)
	{
		let _ = self.proc.kill();
		let _ = self.proc.wait();
	}

}

impl Default for NiriEvtReader {
	fn default() -> Self {
		Self::new()
	}
}

impl Drop for NiriEvtReader
{
	fn drop(&mut self) {
		println!("Shutting down event reader");
		self.shutdown();
	}
}

impl Iterator for NiriEvtReader
{
	type Item = NiriEvent;
	fn next(&mut self) -> Option<NiriEvent> {
		self.next_event()
	}
}

pub fn call_ipc(ipc: &str, args: &str) -> bool
{
	Command::new("niri").args(["msg", "action", ipc, args]).status().map(|s| s.success())
        .unwrap_or(false)
}
