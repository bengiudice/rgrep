use std::{env, process};

use rgrep::Config;

fn main() {
    //TODO: handle panic on invalid unicode
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = rgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
