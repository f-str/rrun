use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;

pub fn read_from_file(path: &str) -> io::Result<String> {
    std::fs::read_to_string(Path::new(expand_path(path.to_string()).as_str()))
}

pub fn write_to_file(path: &str, content: &str) -> io::Result<()> {
    let binding = expand_path(path.to_string());
    let path = Path::new(binding.as_str());
    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap())?;
    }
    match File::create(path) {
        Ok(mut file) => match file.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub fn append_to_file(path: &str, content: &String) -> io::Result<()> {
    let binding = expand_path(path.to_string());
    let path = Path::new(binding.as_str());
    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::File::create(path)?;
    }
    match OpenOptions::new().append(true).open(path) {
        Ok(mut file) => match file.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub fn does_file_exist(path: &str) -> bool {
    Path::new(&expand_path(path.to_string())).exists()
}

pub fn expand_path(path: String) -> String {
    if path.starts_with('~') {
        let home = std::env::var("HOME").unwrap();
        return path.replacen('~', home.as_str(), 1);
    }
    path
}

pub fn check_if_program_is_installed(program: &str) -> bool {
    let output = std::process::Command::new("which")
        .arg(program)
        .output()
        .expect("Failed to execute command");
    output.status.success()
}
