use std::{env, process};

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("There was a problem while parsing your arguments:");
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("There was an application error:");
        eprintln!("{}", err);
        process::exit(1);
    }
}
