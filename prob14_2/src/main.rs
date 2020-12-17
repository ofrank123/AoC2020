extern crate aoc;

use std::collections::HashMap;

const TRUNC_MASK: usize = 0b0000000000000000000000000000111111111111111111111111111111111111usize;
const MAX_BIT: usize =    0b0000000000000000000000000000100000000000000000000000000000000000usize;

struct State {
    zero_mask: usize,
    one_mask: usize,
    memory: HashMap<usize, i64>,
}

impl State {
    fn new() -> Self {
        State {
            zero_mask: !0usize,
            one_mask: 0usize,
            memory: HashMap::new(),
        }
    }

    fn update_mask(&mut self, new_mask: &str) {
        let mut bit: usize = 1;
        self.zero_mask = !0usize;
        self.one_mask = 0usize;
        for c in new_mask.chars().rev() {
            match c {
                '0' => self.zero_mask = self.zero_mask & (!bit),
                '1' => self.one_mask = self.one_mask | bit,
                _ => {}
            }
            bit *= 2;
        }
        println!("{:#066b}", self.zero_mask);
        println!("{:#066b}", self.one_mask);
    }

    fn update_mem(&mut self, index: usize, value: i64) {
        println!("index: {:#066b}", index);
        let index = index | self.one_mask;
        let float_mask = (self.zero_mask ^ self.one_mask) & TRUNC_MASK;

        println!("index: {:#066b}", index);
        println!("float_mask: {:#066b}", float_mask);

        self.update_mem_floating(index, value, float_mask, 1);
    }

    fn update_mem_floating(&mut self,
                           index: usize,
                           value: i64,
                           float_mask: usize,
                           bit: usize) {
        if bit > MAX_BIT {
            println!("Index being written: {:#066b}", index);
            self.memory.insert(index, value);
        } else {
            if float_mask & bit > 0 {
                self.update_mem_floating(index | bit, value, float_mask, bit * 2);
                self.update_mem_floating(index & (!bit), value, float_mask, bit * 2);
            } else {
                self.update_mem_floating(index, value, float_mask, bit * 2);
            }
        }
    }
}

#[derive(Debug)]
enum Op {
    Mask(String),
    Mem(usize, i64),
}

fn get_ops(input: &str) -> Vec<Op> {
    let mut ret = Vec::new();
    for line in input.lines() {
        let split: Vec<_> = line.split(" = ").collect();
        let op = split[0];
        let val = split[1];
        if op == "mask" {
            ret.push(Op::Mask(val.to_string()));
        } else {
            let len = op.len();
            let ind: usize = (&op[4..len-1]).parse().expect("expected usize");
            ret.push(Op::Mem(ind, val.parse().expect("expected signed int")));
        }
    }

    ret
}

fn main() {
    let input = aoc::input_to_str("14");
    let ops = get_ops(&input);
    let mut state = State::new();
    for op in ops {
        match op {
            Op::Mask(mask) => {
                state.update_mask(mask.as_str());
            },
            Op::Mem(i, val) => {
                state.update_mem(i, val);
            }
        }
    }

    println!("Result: {}", state.memory.iter().fold(0, |a, (_, b)| a + b));
}
