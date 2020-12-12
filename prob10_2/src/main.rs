use std::collections::HashMap;

extern crate aoc;

fn main() {
    let input = aoc::input_to_str("10");
    let adapters = get_sorted_adapters(&input);
    println!("{}", get_possible_seqs(&adapters));
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

fn get_compat_adapters(current_adapt: i32, adapters: &Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    for &adapt in adapters {
        let diff = adapt - current_adapt;
        if 0 < diff && diff <= 3 {
            ret.push(adapt);
        }
    }

    ret
}

struct SeqCacher<'a> {
    adapters: &'a Vec<i32>,
    cached_results: HashMap<i32, i64>,
}

impl<'a> SeqCacher<'a> {
    fn get_result(&mut self, current_adapt: i32) -> i64 {
        let res = self.cached_results.get(&current_adapt);
        match res {
            Some(ret) => {
                println!("Getting cached value for: {}", current_adapt);
                *ret
            },
            None => {
                let mut total = 0;
                if current_adapt == self.adapters[self.adapters.len() - 1] {
                    total = 1
                } else {
                    for adapt in get_compat_adapters(current_adapt, &self.adapters) {
                        total += self.get_result(adapt);
                    }
                }
                self.cached_results.insert(current_adapt, total);
                total
            }
        }
    }
}

fn get_possible_seqs(adapters: &Vec<i32>) -> i64 {
    let mut cacher = SeqCacher {
        adapters,
        cached_results: HashMap::new(),
    };

    cacher.get_result(0)
}
