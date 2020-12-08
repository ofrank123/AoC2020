extern crate aoc;

#[derive(Debug, Copy, Clone)]
enum OpCode {
    ACC,
    JMP,
    NOP,
}

#[derive(Debug, Copy, Clone)]
struct Op {
    code: OpCode,
    arg: i32,
}

#[derive(Debug)]
struct OpMachine {
    position: usize,
    accumulator: i32,
    ops: Vec<Op>
}

impl OpMachine {
    fn new(input: &str) -> OpMachine {
        let mut ops = vec![];
        for line in input.lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            let code_str = words[0];
            let arg_str = words[1];

            let code = match code_str {
                "acc" => OpCode::ACC,
                "jmp" => OpCode::JMP,
                "nop" => OpCode::NOP,
                _ => panic!("Expected valid op code, got {}", code_str),
            };

            let arg = arg_str.parse::<i32>().expect(format!("Expected integral value, got {}", arg_str).as_str());

            ops.push(Op {
                code,
                arg
            })
        }

        OpMachine {
            position: 0,
            accumulator: 0,
            ops
        }
    }

    fn print_state(&self) {
        println!("=== STATE ===");
        println!("Position: {}", self.position);
        println!("Accumulator: {}", self.accumulator);
        println!("Current Op: {:?}", self.current_op());
        println!("=============");
    }

    fn current_op(&self) -> Op {
        self.ops[self.position]
    }

    fn next(&mut self) {
        let current_op = self.current_op();

        let next_op = match &current_op.code {
            OpCode::ACC => self.op_acc(current_op.arg),
            OpCode::JMP => self.op_jmp(current_op.arg),
            OpCode::NOP => self.op_nop(),
        };

        if next_op {
            self.position += 1;
        }
    }

    fn op_acc(&mut self, arg: i32) -> bool {
        self.accumulator = self.accumulator + arg;
        true
    }

    fn op_jmp(&mut self, arg: i32) -> bool {
        self.position = (self.position as i32 + arg) as usize;
        false
    }

    fn op_nop(&mut self) -> bool {
        true
    }
}


fn main() {
    let input = aoc::input_to_str("8");
    let mut machine = OpMachine::new(&input);

    println!("{}", accum_at_first_loop(&mut machine));
}

fn accum_at_first_loop(machine: &mut OpMachine) -> i32 {
    let mut visited: Vec<i32> = Vec::new();
    visited.resize(machine.ops.len(), 0);

    while visited[machine.position] < 1 {
        visited[machine.position] += 1;
        machine.next();
    }

    machine.accumulator
}
