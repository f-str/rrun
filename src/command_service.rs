use std::io::{self};

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input};

use crate::config::Config;
use crate::fzf::{fzf_select, fzf_select_with_input};
use crate::invocations::{
    add_new_invocation, is_ordered_flag_set, reset_invocations, set_ordered_flag,
    update_invocations_in_long_term_memory,
};
use crate::ipc::ipc_execute;
use crate::long_term_memory::{LongTermMemory, LongTermMemoryEntry};
use crate::util::get_user_shell;

const TERM_PREFIX: &str = "@";

pub fn add_command(name: &Option<String>, command: &String) -> io::Result<()> {
    let name = name.clone().unwrap_or(command.to_owned());

    let mut ltm = LongTermMemory::load()?;

    if ltm.names.contains(&name) {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "A command with this name already exists.",
        ));
    }

    ltm.names.push(name.clone());
    ltm.memory.insert(
        name.clone(),
        LongTermMemoryEntry {
            name: name.clone(),
            invocations: 0,
            exclude_from_sorting: false,
            command: command.to_owned(),
        },
    );

    ltm.store()
}

pub fn edit_name(name: &Option<String>) -> io::Result<()> {
    let mut ltm = LongTermMemory::load()?;

    let name = name
        .clone()
        .unwrap_or_else(|| fzf_select(ltm.names.clone()));

    println!("Rename the entry with the name '{}'", name);

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap()
    {
        loop {
            let new_name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New name")
                .default(name.clone().to_string())
                .interact_text()
                .unwrap();

            if !ltm.names.contains(&new_name) && new_name != name {
                let mut entry = ltm
                    .memory
                    .remove(&name)
                    .unwrap_or_else(|| panic!("Could not find entry with name '{}'", name));
                entry.name = new_name.clone();
                ltm.memory.insert(new_name.clone(), entry);
                ltm.names
                    .remove(ltm.names.iter().position(|n| n == &name).unwrap());
                ltm.names.push(new_name.clone());
                ltm.store()?;
                println!("Successfully renamed the entry to '{}'", new_name);
                break;
            } else {
                println!("A command with this name already exists");
            }
        }
    }
    Ok(())
}

pub fn edit_command(name: &Option<String>) -> io::Result<()> {
    let mut ltm = LongTermMemory::load()?;

    let name = name
        .clone()
        .unwrap_or_else(|| fzf_select(ltm.names.clone()));

    println!("Edit the command with the name '{}'", name);

    let new_command: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("New command")
        .default(ltm.memory.get(&name).unwrap().command.clone())
        .interact_text()
        .unwrap();

    let entry = ltm
        .memory
        .get_mut(&name)
        .unwrap_or_else(|| panic!("Could not find entry with name '{}'", name));
    entry.command = new_command;

    ltm.store()?;
    println!("Successfully updated the command");

    Ok(())
}

pub fn delete_command(name: &Option<String>) -> io::Result<()> {
    let mut ltm = LongTermMemory::load()?;

    let name = name
        .clone()
        .unwrap_or_else(|| fzf_select(ltm.names.clone()));

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Do you want to delete the entry with the name '{}'",
            name
        ))
        .interact()
        .unwrap()
    {
        ltm.memory.remove(&name);
        ltm.names
            .remove(ltm.names.iter().position(|n| n == &name).unwrap());
        ltm.store()?;
        println!("Successfully deleted the entry");
    }
    Ok(())
}

pub fn statistics() -> io::Result<()> {
    let ltm = LongTermMemory::load()?;

    println!("Statistics:");
    for (name, entry) in &ltm.memory {
        println!("{}: {}", name, entry.invocations);
    }

    Ok(())
}

pub fn exec(name: &String) -> io::Result<()> {
    let mut ltm = LongTermMemory::load()?;

    exec_internal(name, &mut ltm)
}

fn exec_internal(name: &String, ltm: &mut LongTermMemory) -> io::Result<()> {
    let cmd = if let Some(entry) = ltm.memory.get(name) {
        &entry.command
    } else {
        let command = name;

        let mut name = name.clone();
        if name.starts_with(TERM_PREFIX) {
            name = name
                .strip_prefix(TERM_PREFIX)
                .unwrap()
                .trim_start()
                .to_string();
        }

        ltm.names.push(name.clone());
        ltm.memory.insert(
            name.clone(),
            LongTermMemoryEntry {
                name: name.clone(),
                invocations: 0,
                exclude_from_sorting: false,
                command: command.clone(),
            },
        );

        ltm.store()?;
        command
    };

    if cmd.starts_with(TERM_PREFIX) {
        let config = Config::load()?;

        let shell = get_user_shell()?;

        let cmd = &format!(
            "{} {} -ic '{}'",
            config.terminal,
            shell,
            cmd.strip_prefix(TERM_PREFIX).unwrap().trim_start()
        );

        ipc_execute(cmd)?;
    } else {
        ipc_execute(cmd)?;
    }

    add_new_invocation(name)?;

    if !is_ordered_flag_set() {
        update_invocations_in_long_term_memory(ltm)?;

        set_ordered_flag()?;
        reset_invocations()?;
    }

    Ok(())
}

pub fn select_and_execute() -> io::Result<()> {
    let mut ltm = LongTermMemory::load()?;

    if let Some(name) = fzf_select_with_input(ltm.names.clone()) {
        exec_internal(&name, &mut ltm)?;
    }

    Ok(())
}
