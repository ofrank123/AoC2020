extern crate aoc;

fn main() {
    let input = aoc::input_to_str("5");

    let mut largest = 0;
    for line in input.lines() {

        // DEBUG
        // println!("Row: {} \tCol:{} \tID: {}", get_row(line), get_col(line), get_id(line));

        let id = get_id(line);
        if id > largest {
            largest = id;
        }
    }

    println!("Result: {}", largest);
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
