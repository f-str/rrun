use std::io;

mod cli;
mod command_service;
mod config;
mod fzf;
mod invocations;
mod ipc;
mod long_term_memory;
mod notification;
mod util;

fn main() -> io::Result<()> {
    if !util::check_if_program_is_installed("fzf") {
        println!("fzf is not installed");
        std::process::exit(1);
    }
    if !util::check_if_program_is_installed("rg") {
        println!("ripgrep is not installed");
        std::process::exit(1);
    }

    cli::parse_arguments()
}
