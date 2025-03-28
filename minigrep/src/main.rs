use std::env;
use std::process;

use minigrep::Config;

// Responsabilities of main:
// Calling the command line parsing logic with the argument values
// Setting up any other configuration
// Calling a run funcion in lib.rs
// Handling the error if run returns an error

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("unexpected error: {err}");
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
