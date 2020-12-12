extern crate aoc;

use std::cmp;

fn main() {
    let input = aoc::input_to_str("9");
    let nums = get_nums(&input);

    const PREAMB_SIZE: usize = 25;

    let mut invalid_pos = 0;
    for i in PREAMB_SIZE..nums.len() {
        if !valid_position(&nums, i, PREAMB_SIZE) {
            invalid_pos = nums[i];
            break;
        }
    }

    println!("Value: {}", invalid_pos);

    let mut min = std::i64::MAX;
    let mut max = 0;
    for n in find_sum_set(&nums, invalid_pos).expect("No continguous region found") {
        min = cmp::min(min, n);
        max = cmp::max(max, n);
    }

    println!("Max: {} \t Min: {}", max, min);
    println!("Result: {}", max + min);
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

fn find_sum_set(nums: &Vec<i64>, target: i64) -> Option<Vec<i64>> {
    for i in 0..nums.len() {
        let mut sum = 0;
        let mut ret = vec![];
        for j in i..nums.len() {
            sum += nums[j];
            ret.push(nums[j]);
            if sum == target {
                return Some(ret);
            }
            if sum > target {
                break;
            }
        }
    }

    None
}
