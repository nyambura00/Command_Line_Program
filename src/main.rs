use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Search query: {:?}", query);
    println!("In this file: {:?}", filename);

    let file_contents = fs::read_to_string(filename)
        .expect("There is a problem reading the file");
    
    println!("These are the file contents {:?}", file_contents)
}
