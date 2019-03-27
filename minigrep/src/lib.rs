use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(&config.filename)?;

    let result;
    if config.case_insensitive {
        result = search_insensitive(&config.pattern, &file);
    } else {
        result = search(&config.pattern, &file);
    }

    for line in result {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, document: &'a str) -> Vec<&'a str> {
    document
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_insensitive<'a>(query: &str, document: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    document
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

pub struct Config {
    pattern: String,
    filename: String,
    case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // throw away program name
        args.next();

        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing pattern"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing filename"),
        };

        let case_insensitive = !env::var("CASE_INSENSITIVE").is_err();
        ;

        Result::Ok(Config {
            pattern,
            filename,
            case_insensitive,
        })
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

    #[test]
    fn test_case_insensitive_match() {
        let query = "HeLlO";
        let result = search_insensitive(&query, DOCUMENT);
        assert_eq!(1, result.len());
        assert_eq!("hello", result[0]);
    }
}
