use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    // println!("{}", content);

    let result = if config.is_case_sensitive {
        search(&config.query, &content)
    } else {
        case_insensitive_search(&config.query, &content)
    };

    for line in result{
        println!("{}",line)
    }
    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments Passed");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let is_case_sensitive = env::var("IS_CASE_SENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            is_case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

pub fn case_insensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results
}
