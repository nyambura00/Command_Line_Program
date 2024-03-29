use std::{env, process};
use minigrep::Config;

fn main() {
    // passing ownership of iterator to a new Config struct
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query_string);
    println!("In file: {}", config.file_name);

    if let Err(e) = minigrep::run(config) { //handling Err
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}