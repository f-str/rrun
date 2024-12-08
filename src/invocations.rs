use std::{collections::HashMap, fs, io};

use crate::{
    long_term_memory::LongTermMemory,
    util::{append_to_file, does_file_exist, expand_path, read_from_file},
};

const INVOCATION_FILE: &str = "~/.rrun/invocations";
const FLAG_FILE: &str = "/tmp/rrun_ordered";

pub fn add_new_invocation(name: &String) -> io::Result<()> {
    append_to_file(INVOCATION_FILE, &format!("{}\n", name).to_string())
}

pub fn reset_invocations() -> io::Result<()> {
    fs::remove_file(expand_path(INVOCATION_FILE.to_string()))
}

pub fn set_ordered_flag() -> io::Result<()> {
    fs::File::create(FLAG_FILE)?;
    Ok(())
}

pub fn is_ordered_flag_set() -> bool {
    does_file_exist(FLAG_FILE)
}

pub fn update_invocations_in_long_term_memory(ltm: &mut LongTermMemory) -> io::Result<()> {
    let invocations = read_invocations()?;
    add_invocations_to_long_term_memory(ltm, invocations);
    reorder_long_term_memory(ltm);

    set_ordered_flag()?;

    ltm.store()
}

fn read_invocations() -> io::Result<HashMap<String, i32>> {
    let content = read_from_file(INVOCATION_FILE)?;
    let mut map = HashMap::new();
    for line in content.lines() {
        let value_ref: &mut i32 = map.entry(line.to_string()).or_insert(0);
        *value_ref += 1;
    }
    Ok(map)
}

fn add_invocations_to_long_term_memory(
    ltm: &mut LongTermMemory,
    invocations: HashMap<String, i32>,
) {
    for (name, invocations) in invocations {
        if let Some(entry) = ltm.memory.get_mut(&name) {
            entry.invocations += invocations;
        } // If the entry does not exist, we simply ignore it
    }
}

fn reorder_long_term_memory(ltm: &mut LongTermMemory) {
    ltm.names.sort_by(|a, b| {
        let entry_a = ltm.memory.get(a).unwrap();
        let entry_b = ltm.memory.get(b).unwrap();
        if entry_a.exclude_from_sorting && !entry_b.exclude_from_sorting {
            return std::cmp::Ordering::Less;
        }
        if !entry_a.exclude_from_sorting && entry_b.exclude_from_sorting {
            return std::cmp::Ordering::Greater;
        }
        entry_b.invocations.cmp(&entry_a.invocations)
    });
}
