use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }

    pub fn get_query<'a>(&'a self) -> &'a str {
        &self.query
    }

    pub fn get_filename<'a>(&'a self) -> &'a str {
        &self.filename
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.get_filename())?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let res = search(config.get_query(), &contents);
    for line in res {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
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
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}