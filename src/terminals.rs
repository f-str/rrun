use crate::command_service::TERM_PREFIX;
use crate::util::get_user_shell;
use std::fmt::Display;

pub enum Terminal {
    Kitty,
    Ghostty,
}

impl Terminal {
    pub fn from_string(string: &str) -> Option<Terminal> {
        match string.to_lowercase().as_ref() {
            "kitty" => Some(Terminal::Kitty),
            "ghostty" => Some(Terminal::Ghostty),
            _ => None,
        }
    }

    pub fn format_cmd(&self, cmd: &str, name: &str) -> String {
        let shell = get_user_shell().expect("Failed to get shell");

        let stripped_command = cmd.strip_prefix(TERM_PREFIX).unwrap().trim_start();

        match self {
            Terminal::Kitty => {
                format!("kitty {} -ic '{}'", shell, stripped_command)
            }
            Terminal::Ghostty => {
                format!(
                    "ghostty --title={} -e {} -ic '{}'",
                    name, shell, stripped_command
                )
            }
        }
    }

    pub fn default() -> Self {
        Terminal::Kitty
    }

    pub fn config_key() -> String {
        "terminal".to_string()
    }
}

impl Display for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terminal::Kitty => write!(f, "kitty"),
            Terminal::Ghostty => write!(f, "ghostty"),
        }
    }
}
