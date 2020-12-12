extern crate aoc;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug)]
enum TurnDir {
    Left,
    Right
}

#[derive(Debug)]
enum InstructType {
    Dir(Direction),
    Turn(TurnDir),
    Forward
}

#[derive(Debug)]
struct Instruction {
    i_type: InstructType,
    value: i32,
}

struct Ferry {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Ferry {
    fn new() -> Self {
        Ferry {
            x: 0,
            y: 0,
            dir: Direction::East,
        }
    }

    fn process_instruction(&mut self, instruct: Instruction) {
        match instruct.i_type {
            InstructType::Dir(dir) => {
                match dir {
                    Direction::North => self.y += instruct.value,
                    Direction::South => self.y -= instruct.value,
                    Direction::East =>  self.x += instruct.value,
                    Direction::West =>  self.x -= instruct.value,
                }
            },
            InstructType::Turn(dir) => {
                self.dir = match dir {
                    TurnDir::Left => turn_degree(&self.dir, -instruct.value),
                    TurnDir::Right => turn_degree(&self.dir, instruct.value),
                }
            },
            InstructType::Forward => {
                match self.dir {
                    Direction::North => self.y += instruct.value,
                    Direction::South => self.y -= instruct.value,
                    Direction::East =>  self.x += instruct.value,
                    Direction::West =>  self.x -= instruct.value,
                }
            }
        }
    }
}

fn turn_degree(dir: &Direction, degrees: i32) -> Direction {
    let offset = degrees / 90;
    let mut current = match dir {
        Direction::North => 0,
        Direction::East => 1,
        Direction::South => 2,
        Direction::West => 3,
    };
    current = current + offset;
    if current < 0 {
        current = 4 + current;
    }
    current %= 4;
    match current {
        0 => Direction::North,
        1 => Direction::East,
        2 => Direction::South,
        3 => Direction::West,
        _ => panic!("New direction value out of range")
    }
}

fn main() {
    let input = aoc::input_to_str("12");
    let mut ferry = Ferry::new();
    for instruct in get_instructions(&input) {
        ferry.process_instruction(instruct);
    }
    println!("Result: {}", ferry.x.abs() + ferry.y.abs());
    //println!("{:?}", turn_degree(&Direction::North, -90));
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(| line | {
        let letter = &line[..1];
        let value = (&line[1..]).parse::<i32>().expect("Expected integral value");
        Instruction {
            i_type: match letter {
                "N" => InstructType::Dir(Direction::North),
                "S" => InstructType::Dir(Direction::South),
                "E" => InstructType::Dir(Direction::East),
                "W" => InstructType::Dir(Direction::West),
                "L" => InstructType::Turn(TurnDir::Left),
                "R" => InstructType::Turn(TurnDir::Right),
                "F" => InstructType::Forward,
                _ => panic!("Expected instruction type")
            },
            value
        }
    }).collect()
}
