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
        let field_data = splits[1];

        valid_fields += match field_name {
            "byr" => {
                let year = field_data.to_string().parse::<i32>().expect("Could not parse byr as int");

                if 1920 <= year && year <= 2002 {1} else {0}
            },
            "iyr" => {
                let year = field_data.to_string().parse::<i32>().expect("Could not parse iyr as int");

                if 2010 <= year && year <= 2020 {1} else {0}
            },
            "eyr" => {
                let year = field_data.to_string().parse::<i32>().expect("Could not parse eyr as int");

                if 2020 <= year && year <= 2030 {1} else {0}
            },
            "hgt" => {
                let num = &field_data[0..(field_data.len() - 2)];
                let unit = &field_data[(field_data.len() - 2)..field_data.len()];

                let mut ret = 0;
                if unit == "cm" {
                    let data = num.to_string().parse::<i32>().expect("Could not parse hgt as int");
                    if 150 <= data && data <= 193 {
                        ret = 1;
                    }
                }
                else if unit == "in" {
                    let data = num.to_string().parse::<i32>().expect("Could not parse hgt as int");
                    if 59 <= data && data <= 76 {
                        ret = 1;
                    }
                }

                ret
            },
            "hcl" => {
                let hash = &field_data[0..1];
                let mut ret = 0;
                if hash == "#" {
                    let matcher = Regex::new(r"[0-9a-f]+").expect("Invalid regex");
                    if matcher.is_match(&field_data[1..field_data.len()]) {
                        ret = 1
                    }
                }

                ret
            },
            "ecl" => {
                match field_data {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => 1,
                    _ => 0
                }
            },
            "pid" => {
                let mut ret = 0;

                let matcher = Regex::new(r"[0-9]").expect("Invalid regex");
                if matcher.is_match(field_data) && field_data.len() == 9 {
                    ret = 1;
                }

                ret
            }
            _ => 0,
        }
    }

    valid_fields == 7
}
