use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str>{
        args.next();
        let query = match args.next(){
            Some(query) => query,
            None => return Err("Don't get query string!"),
        };
        let filename = match args.next(){
            Some(filename) => filename,
            None => return Err("Don't get a filename!"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("With txt:\n {}", contents);
    println!("----------------------------");
    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for result in results {
        println!("The result is {}", result);
    }
    Ok(())
}

pub fn search_case_sensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {

    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests;