use std::{env, fs, process};

fn main() {
    //TODO: handle panic on invalid unicode
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let contents = fs::read_to_string(&config.file_path).expect("Error while reading file.");

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone(); // TODO avoid clone
        let file_path = args[2].clone(); // TODO avoid clone

        Ok(Config { query, file_path })
    }
}
