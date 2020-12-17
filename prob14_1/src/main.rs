extern crate aoc;

struct State {
    zero_mask: i64,
    one_mask: i64,
    memory: Vec<i64>
}

impl State {
    fn new() -> Self {
        let mut memory = Vec::new();
        memory.resize(1, 0);
        State {
            zero_mask: !0i64,
            one_mask: 0i64,
            memory,
        }
    }

    fn update_mask(&mut self, new_mask: &str) {
        let mut bit: i64 = 1;
        self.zero_mask = !0i64;
        self.one_mask = 0i64;
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
        let value = value & self.zero_mask;
        let value = value | self.one_mask;
        if self.memory.len() - 1 < index {
            self.memory.resize(index + 1, 0);
        }

        self.memory[index] = value;
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
    println!("{:?}", state.memory);
    println!("Result: {}", state.memory.iter().sum::<i64>());
}
