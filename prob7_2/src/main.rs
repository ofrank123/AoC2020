extern crate aoc;

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct Bag {
    name: String,
    contains: Vec<(i32, String)>,
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Bag {}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Bag {
    fn new(name: String, contains: Vec<(i32, String)>) -> Self {
        Self {
            name,
            contains
        }
    }

    fn get_required_children(&self, bags: &HashSet<Bag>) -> i32 {
        let mut total_bags = 0;
        for (num, name) in &self.contains {
            total_bags += num;
            let child_bag = bags.get(&Bag::new(name.to_string(), vec![])).expect("Expected bag to exist");
            total_bags += child_bag.get_required_children(bags) * num;
        }

        total_bags
    }
}

fn main() {
    let input = aoc::input_to_str("7");
    //println!("{:?}", get_bag_set(&input));
    let bags = get_bag_set(&input);

    let shiny_gold = bags.get(&Bag::new("shiny_gold".to_string(), vec![])).expect("Expected bag to exist");
    println!("Result: {}" ,shiny_gold.get_required_children(&bags));
}

fn get_bag_set(input: &str) -> HashSet<Bag> {
    let mut bags = HashSet::new();

    for line in input.lines() {
        let mut word_iter = line.split_whitespace().into_iter();
        let color = format!{"{}_{}", word_iter.next().unwrap(), word_iter.next().unwrap()};
        word_iter.next().expect("Expected bags");
        word_iter.next().expect("Expected contain");

        let mut contains = vec![];

        let mut next = word_iter.next();
        while next != None && next.unwrap() != "no" {
            let contain_number = next.unwrap().parse::<i32>().expect("Could not parse as integral");
            let contain_color = format!("{}_{}", word_iter.next().unwrap(), word_iter.next().unwrap());
            word_iter.next().expect("Expect bag");

            contains.push((contain_number, contain_color));

            next = word_iter.next();
        }

        bags.insert(Bag::new(color, contains));
    }

    bags
}
