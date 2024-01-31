use crate::persistent::{read_from_persistent, write_to_persistent};
use clap::{Parser, Subcommand};
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all runnable commands
    List,

    /// Generates the list of commands
    Generate,
}

pub fn parse_arguments() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List) => list(),
        Some(Commands::Generate) => generate(),
        None => select_and_execute(),
    }
}

fn list() {
    let content = _get_content();
    println!("{}", content);
}

fn generate() {
    println!("Generating...");
    let compgen = Command::new(_get_shell())
        .arg("-ic")
        .arg("compgen -c -a")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute compgen");
    let filter = Command::new("rg")
        .arg("-x")
        .arg("^[a-zA-Z]+\\S*$")
        .stdin(compgen.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute rg");
    let sort = Command::new("sort")
        .arg("-u")
        .stdin(filter.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute sort");
    let output = sort.wait_with_output().unwrap();
    let content = String::from_utf8(output.stdout).unwrap();
    write_to_persistent(content.as_str()).unwrap();
    println!("Done");
}

fn select_and_execute() {
    let content = _get_content();
    let mut fzf = Command::new("fzf")
        .arg("--info=hidden")
        .arg("--border")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute fzf");

    let mut stdin = fzf.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(content.as_bytes())
            .expect("Failed to write to stdin");
    });
    let output = fzf.wait_with_output().expect("Failed to read stdout");
    let command = String::from_utf8_lossy(&output.stdout);

    Command::new("i3-msg")
        .arg("exec")
        .arg(_get_shell() + " -ic")
        .arg(command.trim())
        .output()
        .expect("Failed to execute i3-msg");
}

fn _get_content() -> String {
    read_from_persistent().unwrap_or_else(|_| {
        generate();
        read_from_persistent().unwrap()
    })
}
fn _get_shell() -> String {
    match std::env::var_os("SHELL") {
        Some(val) => val.into_string().unwrap(),
        None => panic!("SHELL is not set"),
    }
}
