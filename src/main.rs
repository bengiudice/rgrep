use std::{env, process};

use rgrep::Config;

fn main() {
    //TODO: handle panic on invalid unicode
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = rgrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}
