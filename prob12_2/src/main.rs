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
    boat_x: i32,
    boat_y: i32,
    way_x: i32,
    way_y: i32,
}

impl Ferry {
    fn new() -> Self {
        Ferry {
            boat_x: 0,
            boat_y: 0,
            way_x: 10,
            way_y: 1,
        }
    }

    fn process_instruction(&mut self, instruct: Instruction) {
        match instruct.i_type {
            InstructType::Dir(dir) => {
                match dir {
                    Direction::North => self.way_y += instruct.value,
                    Direction::South => self.way_y -= instruct.value,
                    Direction::East =>  self.way_x += instruct.value,
                    Direction::West =>  self.way_x -= instruct.value,
                }
            },
            InstructType::Turn(dir) => {
                match dir {
                    TurnDir::Left => {
                        let (new_x, new_y) = turn_degree(self.way_x, self.way_y, -instruct.value);
                        self.way_x = new_x;
                        self.way_y = new_y;
                    },
                    TurnDir::Right => {
                        let (new_x, new_y) = turn_degree(self.way_x, self.way_y, instruct.value);
                        self.way_x = new_x;
                        self.way_y = new_y;
                    },
                }
            },
            InstructType::Forward => {
                self.boat_x += self.way_x * instruct.value;
                self.boat_y += self.way_y * instruct.value;
            }
        }
    }
}

fn turn_degree(x: i32, y: i32, degrees: i32) -> (i32, i32) {
    let mut degrees = degrees;
    if degrees < 0 {
        degrees = 360 + degrees;
    }
    match degrees {
        0 => (x, y),
        90 =>  (y, -x),
        180 => (-x, -y),
        270 => (-y, x),
        _ => panic!("Expected valid turn degree")
    }
}

fn main() {
    let input = aoc::input_to_str("12");
    let mut ferry = Ferry::new();
    for instruct in get_instructions(&input) {
        ferry.process_instruction(instruct);
    }
    println!("X: {} \t Y: {}", ferry.boat_x, ferry.boat_y);
    println!("Result: {}", ferry.boat_x.abs() + ferry.boat_y.abs());
    //println!("{:?}", turn_degree(2, -5, 180));
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
