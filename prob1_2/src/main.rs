extern crate aoc;

fn main() {
    match solve() {
        None => println!("no value found"),
        Some(n) => println!("Answer: {}", n),
    }
}

fn solve() -> Option<i32> {
    let input = aoc::input_to_str("1_1");

    let mut ints: Vec<i32> = Vec::new();
    for line in input.lines() {
        ints.push(line.parse::<i32>().expect("could not parse integer"));
    }
    for i in &ints {
        for j in &ints {
            for k in &ints {
                if i + j + k == 2020 {
                    return Some(i * j * k);
                }
            }
        }
    }

    None
}
