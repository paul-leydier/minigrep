use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config {
            query: query,
            file_path: file_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for found in search(&config.query, &contents) {
        println!("{}", found);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
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
}
