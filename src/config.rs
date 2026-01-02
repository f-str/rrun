use crate::terminals::Terminal;
use crate::util::{does_file_exist, read_from_file, write_to_file};
use std::io;

const CONFIG_FILE: &str = "~/.config/rrun/config";

// Maybe add fzf colors, ltm location, etc.
pub struct Config {
    pub terminal: Terminal,
}

impl Config {
    pub fn new() -> Config {
        Config {
            terminal: Terminal::default(),
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
            if key == Terminal::config_key() {
                config.terminal = Terminal::from_string(value).expect("Unknown terminal config.")
            }
        }
        Ok(config)
    }

    pub fn store(&self) -> io::Result<()> {
        let content = format!("{}={}\n", Terminal::config_key(), self.terminal);
        write_to_file(CONFIG_FILE, &content)
    }

    fn generate_default() -> io::Result<()> {
        // TODO -> Ask what the prefred terminal is
        Config::new().store()
    }
}
