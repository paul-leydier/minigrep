use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query: query,
            file_path: file_path,
            ignore_case: ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for result in results {
        println!("{}", result);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn two_results() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Better than duct tape!";
        assert_eq!(
            vec!["safe, fast, productive.", "Better than duct tape!"],
            search(query, content)
        );
    }

    #[test]
    fn zero_result() {
        let query = "duct";
        let content = "\
Rust:
Pick three.";
        assert_eq!(Vec::<&str>::new(), search(query, content));
    }

    #[test]
    fn case_sensitive() {
        let query = "rUst";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(Vec::<&str>::new(), search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, content));
    }
}
