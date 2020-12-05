extern crate aoc;

fn main() {
    let input = aoc::input_to_str("5");

    let mut id_list: Vec<i32> = vec![];
    for line in input.lines() {
        let mut inserted = false;
        let current_id = get_id(line);
        for (i, &id) in (&id_list).into_iter().enumerate() {
            if current_id < id {
                id_list.insert(i, current_id);
                inserted = true;
                break;
            }
        }
        if !inserted {
            id_list.push(current_id);
        }
    }

    let mut iter = id_list.into_iter();
    let mut prev = iter.next().expect("Empty list");
    let mut res_id = 0;
    for id in iter {
        if id - 1 != prev {
            res_id = id - 1;
            break;
        }
        prev = id;
    }

    println!("Result {}", res_id);
}

fn get_row(pass: &str) -> i32 {
    let row_code = &pass[0..7];
    let mut lower = 0;
    let mut upper = 127;

    for c in row_code.chars() {
        let shift = ((upper - lower) / 2) + 1;
        if c == 'F' {
            upper = upper - shift;
        }
        else {
            lower = lower + shift;
        }
    }

    upper
}

fn get_col(pass: &str) -> i32 {
    let col_code = &pass[7..];
    let mut lower = 0;
    let mut upper = 7;

    for c in col_code.chars() {
        let shift = ((upper - lower) / 2) + 1;
        if c == 'L' {
            upper = upper - shift;
        }
        else {
            lower = lower + shift;
        }
    }

    upper
}

fn get_id(pass: &str) -> i32 {
    (get_row(pass) * 8) + get_col(pass)
}
