use super::IPCHandler;
use std::{
    env,
    io::{self, Read, Write},
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
    result.extend_from_slice(&(len as u32).to_ne_bytes()); // Length (4 bytes) in little-endian

    // Add message type as 4 bytes - run_command type has numeric type 0
    result.extend_from_slice(&(0u32).to_ne_bytes());

    // Add the message payload
    result.extend_from_slice(payload.as_bytes());

    result
}

impl IPCHandler for I3WMIPCHandler {
    fn execute(&self, command: &String) -> io::Result<()> {
        let path = PathBuf::from(env::var("I3SOCK").unwrap());

        let mut sock = UnixStream::connect(&path)?;

        let formatted_msg = format_ipc_command(&format!("exec {command}").to_string());
        sock.write_all(&formatted_msg)?;
        sock.flush()?;

        // sway somehow just launches when response is started to read...
        let mut header_buf = [0_u8; 14];
        sock.read_exact(&mut header_buf)?;

        // Just here for the future, when we want to react to the repsonse from the socket
        // let magic_data: [u8; 6] = header_buf[..6].try_into().unwrap();
        //let payload_len_buf: [u8; 4] = header_buf[2..6].try_into().unwrap();
        // let payload_len = u32::from_ne_bytes(payload_len_buf);
        // let reply_type_buf: [u8; 4] = header_buf[10..14].try_into().unwrap();
        // let reply_type = u32::from_ne_bytes(reply_type_buf);
        // let mut reply_payload = vec![0_u8; payload_len as usize];
        // sock.read_exact(&mut reply_payload)?;
        Ok(())
    }
}
