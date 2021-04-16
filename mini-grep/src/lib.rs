pub struct Config {
    pub search_term: String,
    pub file: String,
}

impl Config {
    pub fn new (args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { search_term: query, file: filename }
    }
}
