use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);
    // dbg!(args);
    let query = &args[1];
    let file_path = &args[2];

    println!("searching for {query}");
    println!("in file {file_path}");

    let contents = fs::read_to_string(file_path).expect("should have been able to read the file");
    println!("With text:\n{contents}");
}
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
