extern crate aoc;

fn main() {
    let input = aoc::input_to_str("10");
    let adapters = get_sorted_adapters(&input);
    println!("{:?}", get_jolt_diffs(&adapters));
    let diffs = get_jolt_diffs(&adapters);
    println!("Result: {}", diffs.0 * diffs.1);
}

fn get_sorted_adapters(input: &str) -> Vec<i32> {
    let mut ret = vec![0];

    for line in input.lines() {
        ret.push(line.parse().expect(format!("Expected integral value, got {}", line).as_str()));
    }

    ret.sort();
    ret.push(ret[ret.len() - 1] + 3);

    ret
}

fn get_jolt_diffs(adapters: &Vec<i32>) -> (i32, i32) {
    let mut diffs_1 = 0;
    let mut diffs_3 = 0;

    let mut iter = adapters.iter();
    let mut prev = iter.next().expect("Expected non-empty list");

    for adapter in iter {
        if adapter - prev == 1 {
            diffs_1 += 1;
        }
        if adapter - prev == 3 {
            diffs_3 += 1;
        }
        prev = adapter;
    }

    (diffs_1, diffs_3)
}
