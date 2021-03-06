use std::{error::Error,fs};
use colored::Colorize;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents){
        for word in line.split(" "){
            if word.contains(&config.query){
                print!("{} ",format!("{}",word.red()));
            } else {
                print!("{} ",word);
            }
        }
    }
    Ok(())
}


pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("not enough arguments.");
        }
        let config = Config{query: args[1].clone(), filename: args[2].clone()};
        return Ok(config);
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
