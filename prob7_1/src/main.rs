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

    fn get_parents(&self, bags: &HashSet<Bag>) -> HashSet<Bag> {
        let mut parents = HashSet::new();
        'outer: for bag in bags {
            if self != bag {
                for (_, child) in &bag.contains {
                    if child == &self.name {
                        parents.insert(bag.clone());
                        continue 'outer;
                    }
                }
            }
        }

        parents
    }
}

fn main() {
    let input = aoc::input_to_str("7");
    //println!("{:?}", get_bag_set(&input));
    let bags = get_bag_set(&input);

    //println!("{:?}", bags.get(&Bag::new("shiny_gold".to_string(), vec![])));

    let mut parents = bags.get(&Bag::new("shiny_gold".to_string(), vec![])).unwrap().get_parents(&bags);
    //println!("{:?}", parents);

    let mut old_size = parents.len();
    loop {
        let mut new_bags = HashSet::new();
        for bag in &parents {
            for add_bag in bag.get_parents(&bags) {
                new_bags.insert(add_bag);
            }
        }

        for bag in new_bags {
            parents.insert(bag);
        }

        if old_size == parents.len() {
            break;
        }
        old_size = parents.len();
    }

    for parent in &parents {
        println!("Parent Bag: {}", parent.name);
    }

    println!("Result: {}", parents.len());
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
