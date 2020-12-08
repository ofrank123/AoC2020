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

impl Clone for OpMachine {
    fn clone(&self) -> Self {
        OpMachine {
            position: self.position,
            accumulator: self.accumulator,
            ops: self.ops.clone(),
        }
    }
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

    #[allow(dead_code)]
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

    fn is_terminated(&self) -> bool {
        self.position == self.ops.len()
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
    let machine = OpMachine::new(&input);

    println!("{}", try_with_flipped_instructs(&machine).expect("No valid code swap"));
}

fn try_with_flipped_instructs(machine: &OpMachine) -> Option<i32> {
    for i in 0..machine.ops.len() {
        if match machine.ops[i].code  {
            OpCode::NOP | OpCode::JMP => true,
            _ => false
        } {
            let mut test_machine = machine.clone();
            test_machine.ops[i].code = match test_machine.ops[i].code {
                OpCode::NOP => OpCode::JMP,
                OpCode::JMP => OpCode::NOP,
                _ => panic!{"Expected JMP or NOP code"}
            };
            if run_program_to_loop_or_term(&mut test_machine) {
                return Some(test_machine.accumulator);
            }
        }
    }

    None
}

fn run_program_to_loop_or_term(machine: &mut OpMachine) -> bool {
    let mut visited: Vec<i32> = Vec::new();
    visited.resize(machine.ops.len(), 0);

    while visited[machine.position] < 1 {
        visited[machine.position] += 1;
        machine.next();

        if machine.is_terminated() {
            return true;
        }
    }

    false
}
