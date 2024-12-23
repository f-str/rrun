use std::io;

use clap::{Parser, Subcommand};

use crate::command_service::{
    add_command, delete_command, edit_command, edit_name, exec, select_and_execute, statistics,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
    // List all runnable commands
    // List,

    // Generates the list of commands
    // Generate,
    /// Adds a new command with a optional name to the long term memory.
    AddCommand {
        /// The name of the new command to add. If no name is provided the command is also used as
        /// its name.
        #[clap(long, short = 'n')]
        name: Option<String>,
        /// The command to execute. If prefixed with '#', the command is executed in the configured
        /// terminal.
        #[clap(long, short = 'c')]
        command: String,
    },

    /// Edit the name of the given name. If no name is supplied you can choose from a fzf list of
    /// all available names
    EditName {
        /// Name of the entry in the long term memory, which should be changed.
        name: Option<String>,
    },

    /// Edit the command of the given name. If no name is supplied you can choose from a fzf list of
    /// all available names.
    EditCommand {
        /// Name of the entry in the long term memory, which command should be edited.
        name: Option<String>,
    },

    /// Delete the entry with the given name from the long term memory. If no name is supplied you
    /// can choose from a fzf list of all available names.
    DeleteCommand {
        /// Name of the entry in the long term memory, which should be deleted.
        name: Option<String>,
    },

    /// Prints the usage statistics about the usage of the differnt commands.
    Statistics,

    /// Executes given commands, and adds them to the long term memory.
    Exec {
        /// The named command to execute. It is executed in the default shell.
        /// If the command behind the name is prefixed with '#' a new terminal is spawend executing the command.
        name: String,
    },
}

pub fn parse_arguments() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::AddCommand { name, command }) => add_command(name, command)?,
        Some(Commands::EditName { name }) => edit_name(name)?,
        Some(Commands::EditCommand { name }) => edit_command(name)?,
        Some(Commands::DeleteCommand { name }) => delete_command(name)?,
        Some(Commands::Statistics) => statistics()?,
        Some(Commands::Exec { name }) => exec(name)?,
        None => select_and_execute()?,
    }

    Ok(())
}
