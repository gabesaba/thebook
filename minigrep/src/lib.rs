use std::error::Error;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(&config.filename)?;
    search(&config.pattern, &file);
    Ok(())
}

fn search<'a>(query: &str, document: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in document.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
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

#[cfg(test)]
mod tests {
    use super::*;

    const DOCUMENT: &str = "\
        hello
hallo
hola";

    #[test]
    fn test_single_match() {
        let query = "ol";
        assert_eq!(vec!["hola"], search(&query, DOCUMENT))
    }

    #[test]
    fn test_no_matches() {
        let query = "lol";
        assert_eq!(0, search(&query, DOCUMENT).len())
    }

    #[test]
    fn test_all_match() {
        let query = "l";
        let result = search(&query, DOCUMENT);
        assert_eq!(3, result.len());
        for (i, line) in DOCUMENT.split("\n").enumerate() {
            assert_eq!(line, result[i]);
        }
    }

}
