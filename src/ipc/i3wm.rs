use super::IPCHandler;
use std::{
    env,
    io::{self, Write},
    os::unix::net::UnixStream,
    path::PathBuf,
};

pub struct I3WMIPCHandler {}

impl I3WMIPCHandler {
    pub fn new() -> I3WMIPCHandler {
        I3WMIPCHandler {}
    }
}

fn format_ipc_command(payload: &String) -> Vec<u8> {
    let len = payload.len(); // Get the byte length of the string
    let mut result = Vec::new();

    // Prepend "i3-ipc" to the result
    result.extend_from_slice("i3-ipc".as_bytes());

    // Convert the length and zero to little-endian byte arrays
    result.extend_from_slice(&(len as u32).to_le_bytes()); // Length (4 bytes) in little-endian

    // Add message type as 4 bytes - run_command type has numeric type 0
    result.extend_from_slice(&(0u32).to_le_bytes());

    // Add the message payload
    result.extend_from_slice(payload.as_bytes());

    result
}

impl IPCHandler for I3WMIPCHandler {
    fn execute(&self, command: &String) -> io::Result<()> {
        let path = PathBuf::from(env::var("I3SOCK").unwrap());

        let mut sock = UnixStream::connect(&path)?;

        let formatted_msg = format_ipc_command(&format!("exec {}", command).to_string());
        sock.write_all(&formatted_msg)?;
        sock.flush()
    }
}
