use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query_string);
    println!("In file: {}", config.file_name);

    if let Err(e) = minigrep::run(config) { //handling Err
        println!("Application error: {}", e);

        process::exit(1);
    }
}