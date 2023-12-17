use minigrep::Config;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)?;
    minigrep::run(config)?;
    Ok(())
}
