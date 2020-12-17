use std::collections::HashMap;

extern crate aoc;

fn main() {
    let input = aoc::input_to_str("15");

    let mut nums: HashMap<i32, i32> = HashMap::new();
    let mut turn = 0;
    let mut last = -1;

    for n in input[0..input.len() - 1].split(",") {
        turn += 1;
        let n: i32 = n.parse().expect("expected integral value");

        if turn != 1 {
            nums.insert(last, turn - 1);
        }
        println!("turn: {} \t num: {}", turn, n);

        last = n;
    }

    while turn < 2020 {
        turn += 1;
        let n = match nums.get(&last) {
            Some(prev) => turn - 1 - prev,
            None => 0,
        };

        nums.insert(last, turn - 1);
        println!("turn: {} \t num: {}", turn, n);

        last = n;
    }
}
