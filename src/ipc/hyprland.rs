use std::{
    env,
    io::{self, ErrorKind, Write},
    os::unix::net::UnixStream,
    path::PathBuf,
};

use super::IPCHandler;

pub struct HyprlandIPCHandler {}

impl HyprlandIPCHandler {
    pub fn new() -> HyprlandIPCHandler {
        HyprlandIPCHandler {}
    }
}

impl IPCHandler for HyprlandIPCHandler {
    fn execute(&self, command: &String) -> io::Result<()> {
        let mut path = PathBuf::from(
            env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/run/user/1000".to_string()),
        );
        path.push("hypr");

        if !path.exists() {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                "Path to the hyprland socket does not exist!",
            ));
        }
        path.push(env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap()); // We only reach this when the variable exists in the first place
        let sock1_path = path.join(".socket.sock");

        let mut sock1_sock = UnixStream::connect(&sock1_path)?;
        let cmd = &format!("/dispatch -- exec {command}");
        sock1_sock.write_all(cmd.as_bytes())?;
        sock1_sock.flush()
    }
}
