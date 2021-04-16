use std::{env, fs};

mod lib;
use lib::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {} in {}", config.search_term, config.file);

    let file_contents = fs::read_to_string(config.file)
        .expect("Gasp! Couldn't read that file...");

    println!("Contents: \n {}", file_contents)
}
