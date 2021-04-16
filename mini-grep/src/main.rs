use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("There was a problem while parsing your arguments:");
        println!("{}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}", config.search_term, config.file);

    if let Err(err) = minigrep::run(config) {
        println!("There was an application error:");
        println!("{}", err);
        process::exit(1);
    }
}
