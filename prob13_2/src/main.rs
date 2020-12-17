extern crate aoc;

fn main() {
    let input = aoc::input_to_str("13");
    let lines: Vec<_> = input.lines().collect();
    let mut ids: Vec<Option<i64>> = Vec::new();
    for id in lines[1].split(",") {
        if id != "x" {
            ids.push(Some(id.parse().expect("Expected integral value")));
        } else {
            ids.push(None);
        }
    }

    let mut current_time: i64 = 0;
    let mut increment = ids[0].expect("Expected some value");
    let mut mults_found = 1;
    let mut last_mult = 0;
    let mut looking_next_mult = false;
    let mut valid = false;
    while !valid {
        //println!("{}", current_time);
        let mut offset = 0;
        let mut mults = 0;
        valid = true;
        for id in &ids {
            match id {
                Some(id) => {
                    if (current_time + offset) % id != 0 {
                        valid = false;
                        break;
                    } else {
                        //println!("{}", id);
                        mults += 1;
                    }
                },
                None => {}
            };
            offset += 1;
        }
        if mults > mults_found {
            if looking_next_mult {
                increment = current_time - last_mult;
                mults_found = mults;
                looking_next_mult = false;
                println!("New increment: {}", increment);
            } else {
                last_mult = current_time;
                looking_next_mult = true;
            }
        }
        if !valid {
            current_time += increment;
        }
    }

    println!("Result: {}", current_time);
}
