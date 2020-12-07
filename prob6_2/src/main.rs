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
    let mut iter = group.into_iter();
    for question in iter.next().expect("Group Empty").chars() {
        questions.insert(question);
    }

    for person in iter {
        let mut c_questions: HashSet<char> = HashSet::new();
        for question in person.chars() {
            c_questions.insert(question);
        }

        questions.retain(|&k| c_questions.contains(&k));
    }

    questions.len() as i32
}
