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
    let lower_bound = split_range[0].parse::<i32>().expect("could not parse int");
    let upper_bound = split_range[1].parse::<i32>().expect("could not parse int");
    let letter: char = split_rule[1].chars().collect::<Vec<char>>()[0];

    let mut letters_count = 0;
    for char in password.chars() {
        if char == letter {
            letters_count += 1;
        }
    }

    lower_bound <= letters_count && letters_count <= upper_bound
}
