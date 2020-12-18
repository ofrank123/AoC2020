extern crate aoc;

use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Active,
    Inactive,
}

#[derive(Debug)]
struct Dimension {
    points: HashMap<(i32, i32, i32, i32), State>
}

impl Dimension {
    fn new(input: &str) -> Self {
        let mut points = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        points.insert((x as i32, y as i32, 0, 0), State::Active);
                    },
                    _ => {}
                }
            }
        }

        Dimension {
            points,
        }
    }

    fn get_point(&self, point: &(i32, i32, i32, i32)) -> State {
        if self.points.contains_key(&point) && self.points.get(&point).unwrap() == &State::Active {
            State::Active
        } else {
            State::Inactive
        }
    }

    fn get_next_state_point(&self, (x, y, z, w): &(i32, i32, i32, i32)) -> State {
        let mut neighbors = 0;
        for (off_x, off_y, off_z, off_w) in get_adjacency_offsets() {
            match self.get_point(&(x + off_x, y + off_y, z + off_z, w + off_w)) {
                State::Active => neighbors += 1,
                State::Inactive => {}
            }
        }

        let current_state = self.get_point(&(*x, *y, *z, *w));
        if current_state == State::Active {
            if neighbors == 2 || neighbors == 3 {
                State::Active
            } else {
                State::Inactive
            }
        } else {
            if neighbors == 3 {
                State::Active
            } else {
                State::Inactive
            }
        }
    }

    fn update(&mut self) {
        let mut updates: HashMap<(i32, i32, i32, i32), State> = HashMap::new();
        for ((x, y, z, w), _) in &self.points {
            updates.insert((*x, *y, *z, *w), self.get_next_state_point(&(*x, *y, *z, *w)));
            for (off_x, off_y, off_z, off_w) in get_adjacency_offsets() {
                updates.insert((x + off_x, y + off_y, z + off_z, w + off_w),
                               self.get_next_state_point(&(x + off_x, y + off_y, z + off_z, w + off_w)));
            }
        }

        for (point, state) in updates {
            match state {
                State::Active => {
                    self.points.insert(point, State::Active);
                },
                State::Inactive => {
                    self.points.remove(&point);
                }
            }
        }
    }

    fn count(&self) -> i32 {
        let mut sum = 0;
        for (_, state) in &self.points {
            sum += match state {
                State::Active => 1,
                State::Inactive => 0,
            }
        }
        sum
    }
}


fn get_adjacency_offsets() -> Vec<(i32, i32, i32, i32)> {
    let mut ret = vec![];

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if (x, y, z, w) != (0, 0, 0, 0) {
                        ret.push((x, y, z, w));
                    }
                }
            }
        }
    }

    ret
}

fn main() {
    let input = aoc::input_to_str("17");
    let mut dimension = Dimension::new(&input);
    for _ in 0..6 {
        dimension.update();
    }
    println!("Result: {}", dimension.count());
}
