use std::{env, fs};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)?;
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    run(config)?;
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config{query: query, file_path: file_path})
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}