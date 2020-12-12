extern crate aoc;

fn main() {
    let input = aoc::input_to_str("9");
    let nums = get_nums(&input);

    const PREAMB_SIZE: usize = 25;

    for i in PREAMB_SIZE..nums.len() {
        if !valid_position(&nums, i, PREAMB_SIZE) {
            println!("{}", nums[i]);
        }
    }
}

fn get_nums(input: &str) -> Vec<i64> {
    let mut vec = vec![];

    for line in input.lines() {
        vec.push(line.parse::<i64>().expect("Expected an integral value"));
    }

    vec
}

fn valid_position(nums: &Vec<i64>, position: usize, preamb: usize) -> bool {
    for i in position - preamb..position {
        for j in position - preamb..position {
            if i != j {
                if nums[i] + nums[j] == nums[position] {
                    return true;
                }
            }
        }
    }

    false
}
