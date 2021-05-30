use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        // Skip args[0] as it is the program's name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a filename string"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    if config.case_sensitive {
        for result in search_case_sensitive(&config.query, &contents) {
            println!("{}", result);
        }
    } else {
        for result in search_case_insensitive(&config.query, &contents) {
            println!("{}", result);
        }
    }
    Ok(())
}

// Illustrates how to right the search function using iterators and closures
pub fn search_case_sensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str>
{
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// Procedural programming style
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str>
{
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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
            Pick three.
        ";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Duct tape!
        ";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me...
        ";

        assert_eq!(
            vec!["Rust:", "Trust me..."],
            search_case_insensitive(query, contents)
        );
    }
}
