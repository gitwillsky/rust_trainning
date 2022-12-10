use std::{env, error::Error, fs};

pub struct Config {
    filename: String,
    query_str: String,
    case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("invalid program params");
        }
        let query_str = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            filename,
            query_str,
            case_insensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&contents, &config.query_str, config.case_insensitive) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(contents: &'a str, query_str: &str, case_insensitive: bool) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if case_insensitive {
            if line.contains(query_str) {
                result.push(line);
            }
        } else {
            if line.to_lowercase().contains(&(query_str.to_lowercase())) {
                result.push(line);
            }
        }
    }
    return result;
}
