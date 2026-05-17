# PWSKL - Per-Workspace Keyboard Layout for Niri Wayland compositor

## Dependencies: [Niri IPC crate](https://crates.io/crates/niri-ipc)

### What is it?
A Rust utility that maintains different keyboard layouts across workspaces.
It uses network-based IPC (branch with shell-based IPC interface is also available) to subscribe to the
Niri event-stream. This stream contains a current state of the workspaces, their indices, active layout, etc.
It parses set of events related to workspaces and keyboard layout into a state map, then, upon switching to
some workspace - restores it's last used keyboard layout (`niri msg switch-layout <idx>`).

### Use case
Multi-lingual setups. I was tired of switching between 3 layouts when coding and chatting with somebody.
Same functionality exists somewhere in the depths of GNOME compositor.

### How to build and install
`cargo build -r` ->  `target/release/pwskl` (binary)
`sudo cp target/release/pwskl /usr/bin/pwskl`
or if you don't want to put it there:
`cp target/release/pwskl ~/.local/bin/pwskl`
just ensure `~/.local/bin/` is in your `echo $PATH`

### Usage
Personally I have added `spawn-at-startup` to my config section, you can do whatever you want.
Probably better to make it into systemd service as it may panic upon connection failure, here's outline:
```systemd
[Unit]
Description=PWSKL - Per-workspace Keyboard Layout for Niri
PartOf=graphical-session.target
After=graphical-session.target

[Service]
Type=simple
ExecStart=/usr/bin/pwskl
Restart=on-failure
RestartSec=3

[Install]
WantedBy=graphical-session.target
```
And then something like:
```sh
systemctl --user daemon-reload
systemctl --user enable --now pwskl.service
```

#### NOTE: it is my first Rust "project" - I have no prior experience, so there's room for improvement - currently it "just works".

#### LICENSE: NO LICENSE.
