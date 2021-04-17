use std::{env, fs, error::Error};

pub struct Config {
    pub search_term: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new (mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let search_term = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't receive a search term")
        };
        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't receive a file name")
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            search_term,
            file,
            case_sensitive
        })
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
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
