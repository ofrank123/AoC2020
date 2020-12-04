extern crate aoc;

use regex::Regex;

fn main() {
    let input = aoc::input_to_str("4");

    let mut valid_passports = 0;
    for passport in get_passports(&input) {
        if validate_passport(passport) {
            valid_passports += 1;
        }
    }

    println!("Result: {}", valid_passports);
}

fn get_passports(input: &str) -> Vec<Vec<&str>> {
    let seperator = Regex::new(r"([ \n])").expect("Invalid regex");
    let splits: Vec<_> = seperator.split(&input).into_iter().collect();

    let mut passports: Vec<Vec<&str>> = vec![vec![]];
    for split in splits {
        if split == "" {
            passports.push(vec![]);
        } else {
            let current_last = passports.len() - 1;
            passports[current_last].push(split);
        }
    }
    let current_last = passports.len() - 1;
    passports.remove(current_last);

    passports
}

fn validate_passport(passport: Vec<&str>) -> bool {
    let mut valid_fields = 0;

    for field in passport {
        let splits: Vec<&str> = field.split(":").collect();
        let field_name = splits[0];

        valid_fields = match field_name {
            "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => valid_fields + 1,
            _ => valid_fields,
        }
    }

    valid_fields == 7
}
