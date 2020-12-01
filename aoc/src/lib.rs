use std::fs::File;
use std::io::prelude::*;

pub fn input_to_str(problem: &str) -> String {
    let filepath = format!("input/{}.input", problem);

    let mut file = File::open(filepath).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}
