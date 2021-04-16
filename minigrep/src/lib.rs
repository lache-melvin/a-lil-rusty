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
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_creates_config() -> Result<(), String> {
        let test_args = [String::from("binary name"), String::from("search for me"), String::from("testfile.txt")];
        let config = Config::new(&test_args);
        match config {
            Ok(conf) => {
                assert_eq!(conf.search_term, "search for me");
                assert_eq!(conf.file, "testfile.txt");
                Ok(())
            },
            Err(e) => {
                Err(e.to_string())
            },
        }
    }

    #[test]
    fn error_if_insufficient_args() -> Result<(), String> {
        let test_args = [String::from("binary name")];
        let config = Config::new(&test_args);
        match config {
            Ok(_) => {
                Err("this should not work omg".to_string())
            },
            Err(e) => {
                assert_eq!(e, "Insufficient arguments. Make sure you pass both the string you're searching for and the filepath.");
                Ok(())
            },
        }
    }
}
