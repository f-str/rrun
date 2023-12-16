use crate::util::{read_from_file, write_to_file};
use std::io;

const STORAGE_FILE: &str = "~/.rrun/tmp";

pub fn read_from_persistent() -> io::Result<String> {
    read_from_file(STORAGE_FILE)
}

pub fn write_to_persistent(content: &str) -> io::Result<()> {
    write_to_file(STORAGE_FILE, content)
}
