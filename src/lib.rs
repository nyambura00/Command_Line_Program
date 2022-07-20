use std::{fs, error::Error};

pub struct Config {
    pub query_string: String,
    pub file_name: String,
}

impl Config{
    pub fn new(args:&[String])-> Result<Config, &'static str> { //handling error
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query_string = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config {query_string, file_name})
    }
}

// handling logic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    for line in search(&config.query_string, &contents) {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}