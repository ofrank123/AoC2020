extern crate aoc;

fn main() {
    let input = aoc::input_to_str("2");

    let mut valid_passwords = 0;

    for line in input.lines() {
        if valid(line) {
            valid_passwords += 1;
        }
    }

    println!("Result: {}", valid_passwords);
}

fn valid(line: &str) -> bool {
    let split_line: Vec<&str> = line.split(": ").collect();
    let password: &str = &split_line[1];

    let split_rule: Vec<&str> = split_line[0].split(" ").collect();
    let split_range: Vec<&str> = split_rule[0].split("-").collect();

    let first_index = split_range[0].parse::<usize>().expect("could not parse int") - 1;
    let second_index = split_range[1].parse::<usize>().expect("could not parse int") - 1;
    let letter: char = split_rule[1].chars().collect::<Vec<char>>()[0];

    let chars: Vec<char> = password.chars().collect();

    (chars[first_index] == letter || chars[second_index] == letter)
        && !(chars[first_index] == letter && chars[second_index] == letter)
}
