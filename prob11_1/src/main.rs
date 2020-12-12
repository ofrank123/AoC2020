extern crate aoc;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Seat {
    Floor,
    Empty,
    Full,
}

struct SeatGrid {
    grid: Vec<Vec<Seat>>,
    changed: bool
}

impl SeatGrid {
    fn new(input: &str) -> Self {
        SeatGrid {
            grid: get_seat_grid(input),
            changed: true,
        }
    }

    fn get_pos(&self, x: usize, y: usize) -> Option<Seat> {
        if x < self.grid[0].len() && y < self.grid.len() {
            Some(self.grid[y][x])
        }
        else {
            None
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> i32 {
        let mut neighbors = 0;

        let offsets = vec![(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
        for (x_off, y_off) in offsets {
            let neigh_x = x as i32 + x_off;
            let neigh_y = y as i32 + y_off;
            if neigh_x >= 0 && neigh_y >= 0 {
                let neigh_x = neigh_x as usize;
                let neigh_y = neigh_y as usize;

                let seat = self.get_pos(neigh_x, neigh_y);
                let seat = match seat {
                    Some(seat) => seat,
                    None => {continue}
                };

                match seat {
                    Seat::Full => neighbors += 1,
                    _ => {}
                }
            }
        }

        neighbors
    }

    fn pos_next_state(&self, x: usize, y: usize) -> Seat {
        match self.get_pos(x, y) {
            Some(seat) => match seat {
                Seat::Floor => Seat::Floor,
                Seat::Empty => {
                    match self.get_neighbors(x, y) {
                        0 => Seat::Full,
                        _ => Seat::Empty,
                    }
                },
                Seat::Full => {
                    match self.get_neighbors(x, y) {
                        4..=8 => Seat::Empty,
                        _ => Seat::Full,
                    }
                }
            },
            None => {
                panic!("Invalid pos")
            }
        }
    }

    fn next_state(&mut self) {
        let mut new_state = self.grid.clone();

        self.changed = false;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                //print!("{}", self.get_neighbors(x, y));
                let next_state_pos = self.pos_next_state(x, y);
                if next_state_pos != self.get_pos(x, y).unwrap() {
                    self.changed = true;
                }

                new_state[y][x] = next_state_pos;
            }
            //println!();
        }
        //println!();

        self.grid = new_state;
    }

    fn get_occupied_seats(&self) -> i32 {
        let mut occ = 0;
        for row in &self.grid {
            for seat in row {
                if *seat == Seat::Full {
                    occ += 1;
                }
            }
        }

        occ
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.grid {
            for seat in row {
                print!("{}", seat_to_char(seat));
            }
            println!("");
        }
        println!("");
    }
}

fn main() {
    let input = aoc::input_to_str("11");
    let mut seat_grid = SeatGrid::new(&input);
    while seat_grid.changed {
        seat_grid.next_state();
        //seat_grid.print();
    }

    println!("Result: {}", seat_grid.get_occupied_seats());
}

fn get_seat_grid(input: &str) -> Vec<Vec<Seat>> {
    let mut ret = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for seat in line.chars() {
            let seat = match seat {
                '.' => Seat::Floor,
                'L' => Seat::Empty,
                '#' => Seat::Full,
                _ => panic!("Expected valid char, got: {}", seat)
            };
            row.push(seat);
        }
        ret.push(row);
    }

    ret
}

fn seat_to_char(seat: &Seat) -> char {
    match seat {
        Seat::Floor => '.',
        Seat::Empty => 'L',
        Seat::Full => '#',
    }
}
