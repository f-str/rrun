mod cli;
mod persistent;
mod util;

fn main() {
    if !util::check_if_program_is_installed("fzf") {
        println!("fzf is not installed");
        std::process::exit(1);
    }
    if !util::check_if_program_is_installed("rg") {
        println!("ripgrep is not installed");
        std::process::exit(1);
    }

    cli::parse_arguments();
}
