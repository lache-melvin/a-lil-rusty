use std::{env, fs, error::Error};

pub struct Config<'a> {
    pub search_term: &'a String,
    pub file: &'a String,
    pub case_sensitive: bool,
}

impl Config<'_> {
    pub fn new (args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Insufficient arguments. Make sure you pass both the string you're searching for and the filepath.")
        }
        let search_term = &args[1];
        let file = &args[2];
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { search_term, file, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file)?;
    let search_results = if config.case_sensitive {
        search(&config.search_term, &file_contents)
    } else {
        search_case_insensitive(&config.search_term, &file_contents)
    };
    for line in search_results {
        println!("{}", line);
    };
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut search_results = Vec::new();
    for current_line in contents.lines() {
        if current_line.contains(query) {
            search_results.push(current_line);
        }
    };
    search_results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut search_results = Vec::new();
    for current_line in contents.lines() {
        let lowercase_line = current_line.to_lowercase();
        if lowercase_line.contains(&query) {
            search_results.push(current_line);
        }
    };
    search_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "hello";
        let contents = "\
Here are some greetings:
hello there
howdy
hello hello
Hello, good friend!
goes well?";

        assert_eq!(vec!["hello there", "hello hello"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "hElLo";
        let contents = "\
Here are some greetings:
hello there
howdy
hello hello
Hello, good friend!
goes well?";

        assert_eq!(vec!["hello there", "hello hello", "Hello, good friend!"], search_case_insensitive(query, contents));
    }

    #[test]
    fn config_new_creates_config() {
        let test_args = vec![String::from("binary name"), String::from("search for me"), String::from("testfile.txt")];
        let config = Config::new(&test_args).unwrap();
        assert_eq!(config.search_term, "search for me");
        assert_eq!(config.file, "testfile.txt");
    }

    #[test]
    fn error_if_insufficient_args() {
        let test_args = vec![String::from("binary name")];
        assert!(Config::new(&test_args).is_err(), "not enough arguments were passed but you said nothing!");
    }
}
