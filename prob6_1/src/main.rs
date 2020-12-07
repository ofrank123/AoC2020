extern crate aoc;

use std::collections::HashSet;

fn main() {
    let input = aoc::input_to_str("6");

    let groups = get_groups(&input);

    let mut result = 0;
    for group in groups {
        result += group_value(group);
    }

    println!("Result: {}", result);
}

fn get_groups(input: &str) -> Vec<Vec<&str>> {
    let mut groups = vec![vec![]];

    for line in input.lines() {
        if line == "" {
            groups.push(vec![]);
        } else {
            let last_ind = groups.len() - 1;
            groups[last_ind].push(line);
        }
    }

    groups
}

fn group_value(group: Vec<&str>) -> i32 {
    let mut questions: HashSet<char> = HashSet::new();

    for person in group {
        for question in person.chars() {
            questions.insert(question);
        }
    }

    questions.len() as i32
}
