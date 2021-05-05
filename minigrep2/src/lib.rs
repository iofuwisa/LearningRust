//! # minigrep2
//!
//! A library for grep textfile
//!

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

/// The setting values.
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    /// Constructor
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }

    pub fn get_query<'a>(&'a self) -> &'a str {
        &self.query
    }

    pub fn get_filename<'a>(&'a self) -> &'a str {
        &self.filename
    }
    pub fn get_case_insentive<'a>(&'a self) -> &'a bool {
        &self.case_sensitive
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.get_filename())?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let res =
    if *config.get_case_insentive() {
        search(config.get_query(), &contents)
    } else {
        search_case_insensitive(config.get_query(), &contents)
    };
    
    for line in res {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
// (最後の行のみ)
// 私を信じて
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