use std::io;

use crate::util::{does_file_exist, read_from_file, write_to_file};

const CONFIG_FILE: &str = "~/.config/rrun/config";

// Maybe add fzf colors, ltm location, etc.
pub struct Config {
    pub terminal: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            terminal: "kitty".to_string(),
        }
    }

    pub fn load() -> io::Result<Config> {
        if !does_file_exist(CONFIG_FILE) {
            Config::generate_default()?;
        }
        let content = read_from_file(CONFIG_FILE)?;
        let lines = content.lines();
        let mut config = Config::new();
        for line in lines {
            let mut parts = line.split("=");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            if key == "terminal" {
                config.terminal = value.to_string()
            }
        }
        Ok(config)
    }

    pub fn store(&self) -> io::Result<()> {
        let content = format!("terminal={}\n", self.terminal);
        write_to_file(CONFIG_FILE, &content)
    }

    fn generate_default() -> io::Result<()> {
        // TODO -> Ask what the prefred terminal is
        Config::new().store()
    }
}
