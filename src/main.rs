use std::{env, fs};

fn main() {
    //TODO: handle panic on invalid unicode
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    let contents = fs::read_to_string(file_path).expect("Error while reading file.");

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    println!("With text:\n{contents}");
}

fn parse_config<'a>(args: &Vec<String>) -> (&String, &String) {
    let query = &args[1]; // TODO handle panic
    let file_path = &args[2]; // TODO handle panic

    (query, file_path)
}
