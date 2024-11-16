use std::collections::HashMap;
use std::io;

use crate::util::{does_file_exist, read_from_file, write_to_file};

const STORAGE_FILE: &str = "~/.rrun/ltm";

pub struct LongTermMemoryEntry {
    pub name: String,
    pub invocations: i32,
    pub exclude_from_sorting: bool, // Entries with this flag will be always on top (according to the order of adding)
    pub command: String,
}

pub struct LongTermMemory {
    version: String,
    pub names: Vec<String>,
    pub memory: HashMap<String, LongTermMemoryEntry>,
}

impl LongTermMemory {
    pub fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            names: Vec::new(),
            memory: HashMap::new(),
        }
    }

    pub fn load() -> io::Result<LongTermMemory> {
        if !does_file_exist(STORAGE_FILE) {
            Self::generate_default()?;
        }
        let content = read_from_file(STORAGE_FILE)?;

        let mut lines = content.lines();
        let mut memory = LongTermMemory::new();
        memory.version =
            lines.next().unwrap().replace("###", "").trim().to_string()[2..].to_string(); // First remove the '###' prefix and suffix and then remove the 'v.' prefix
        for line in lines {
            let mut parts = line.split("#");
            let invocations = parts.next().unwrap().parse::<i32>().unwrap();
            let name = parts.next().unwrap().to_string();
            let exclude_from_sorting = parts.next().unwrap().parse::<bool>().unwrap();
            let command = parts.next().unwrap().to_string();
            memory.names.push(name.clone());
            memory.memory.insert(
                name.clone(),
                LongTermMemoryEntry {
                    name: name.clone(),
                    invocations,
                    exclude_from_sorting,
                    command,
                },
            );
        }
        Ok(memory)
    }

    pub fn store(&self) -> io::Result<()> {
        let mut content = format!("### v.{} ###\n", self.version);
        for name in &self.names {
            let entry = self.memory.get(name).unwrap();
            content.push_str(&format!(
                "{}#{}#{}#{}\n",
                entry.invocations, entry.name, entry.exclude_from_sorting, entry.command
            ));
        }
        write_to_file(STORAGE_FILE, &content)
    }

    fn generate_default() -> io::Result<()> {
        LongTermMemory::new().store()
    }
}
