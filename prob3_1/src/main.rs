extern crate aoc;

fn main() {
    let input = aoc::input_to_str("3");

    let matrix = input_to_matrix(&input);
    println!("{:?}", matrix);
    println!("Result: {}", count_trees(3, 1, matrix));
}

fn input_to_matrix(input: &str) -> Vec<Vec<bool>> {
    let mut ret_matrix = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c == '#');
        }
        ret_matrix.push(row);
    } 

    ret_matrix
}

fn count_trees(across: usize, down: usize, matrix: Vec<Vec<bool>>) -> i32 {
    let mut count = 0;

    let mut x: usize = 0;
    let mut y: usize = 0;

    while y < matrix.len() {
        if matrix[y][x] {
            count += 1;
        }

        x = (x + across) % matrix[0].len();
        y += down;
    }

    count
}
