use std::{fs, error::Error, env};

pub struct Config {
    pub query_string: String,
    pub file_name: String,
    pub ignore_case: bool,
}

impl Config {
    // env::args implements the Iterator trait and returns String values
    pub fn new(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        args.next();

        let query_string = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query_string,
            file_name,
            ignore_case,
        })
    }
}

// handling logic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query_string, &contents)
    } else {
        search(&config.query_string, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {   
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}
