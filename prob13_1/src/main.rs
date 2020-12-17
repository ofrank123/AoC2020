extern crate aoc;

fn main() {
    let input = aoc::input_to_str("13");
    let lines: Vec<_> = input.lines().collect();
    let min_wait = lines[0].parse::<i32>().expect("Expected integral value");
    let mut ids: Vec<i32> = Vec::new();
    for id in lines[1].split(",") {
        if id != "x" {
            ids.push(id.parse().expect("Expected integral value"));
        }
    }

    let mut ids_wait: Vec<i32> = Vec::new();
    for id in &ids {
        let mut wait = 0;
        while wait < min_wait {
            wait += id;
        }
        ids_wait.push(wait);
    }

    let mut min_ind = 0;
    for (i, &wait) in ids_wait.iter().enumerate() {
        if wait < ids_wait[min_ind] {
            min_ind = i;
        }
    }

    println!("Result: {}", ids[min_ind] * (ids_wait[min_ind] - min_wait));
}
