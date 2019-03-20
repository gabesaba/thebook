use std::error::Error;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(&config.filename)?;
    for (i, line) in file.split("\n").enumerate() {
        if line.contains(&config.pattern) {
            println!("{}: {}", i, line);
        }
    }
    Ok(())
}

pub struct Config {
    pattern: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() == 3 {
            true => {
                let pattern = args[1].to_owned();

                let filename = args[2].to_owned();
                Result::Ok(Config { pattern, filename })
            }
            false => Result::Err("minigrep requires two args"),
        }
    }
}
