use std::{fs, error::Error};

pub struct Config<'a> {
    pub search_term: &'a String,
    pub file: &'a String,
}

impl Config<'_> {
    pub fn new (args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Insufficient arguments. Make sure you pass both the string you're searching for and the filepath.")
        }
        let search_term = &args[1];
        let file = &args[2];

        Ok(Config { search_term, file })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file)?;

    println!("Contents: \n {}", file_contents);

    Ok(())
}
