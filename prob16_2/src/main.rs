extern crate aoc;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Field {
    name: String,
    ranges: Vec<(i32, i32)>,
}

impl Field {
    fn in_field(&self, val: i32) -> bool {
        for (lower, upper) in &self.ranges {
            if val >= *lower && val <= *upper {
                return true
            }
        }

        false
    }
}

fn get_fields(section: &str) -> Vec<Field>{
    let mut ret = Vec::new();
    for line in section.lines() {
        let split: Vec<_> = line.split(": ").collect();
        let name = split[0];
        let mut ranges = vec![];
        for range in split[1].split(" or ") {
            let nums: Vec<_> = range.split("-").collect();
            let lower: i32 = nums[0].parse().expect("Expected integral value");
            let upper: i32 = nums[1].parse().expect("Expected integral value");
            ranges.push((lower, upper));
        }
        ret.push(Field {
            name: name.to_string(),
            ranges,
        });
    }
    ret
}

fn get_your_ticket(section: &str) -> Vec<i32> {
    let mut ret = vec![];
    for num in section.split("\n").collect::<Vec<_>>()[1].split(",") {
        let n = num.parse().expect("Expected integral value");
        ret.push(n);
    }

    ret
}

fn get_tickets(section: &str) -> Vec<Vec<i32>> {
    let mut ret = vec![];

    let mut iter = section.split("\n");
    iter.next();
    for ticket in iter {
        if ticket != "" {
            let mut t = vec![];
            for num in ticket.split(",") {
                t.push(num.parse().expect("expected integral value"));
            }
            ret.push(t);
        }
    }

    ret
}

fn valid_in_any_fields(fields: &Vec<Field>, val: i32) -> bool {
    for field in fields {
        if field.in_field(val) {
            return true;
        }
    }

    false
}

fn ticket_valid(ticket: &Vec<i32>, fields: &Vec<Field>) -> bool {
    let mut sum = 0;
    for &num in ticket {
        if !valid_in_any_fields(fields, num) {
            return false;
        }
    }

    return true;
}

fn possible_fields(field: usize, tickets: &Vec<&Vec<i32>>, fields: &Vec<Field>) -> Vec<Field> {
    let mut valid_fields = fields.clone();

    for ticket in tickets {
        let field_val = ticket[field];
        valid_fields.retain(| field| field.in_field(field_val));
    }

    valid_fields
}

fn main() {
    let input = aoc::input_to_str("16");
    let sections: Vec<_> = input.split("\n\n").collect();
    let fields = get_fields(sections[0]);
    let your_ticket = get_your_ticket(sections[1]);
    let tickets = get_tickets(sections[2]);
    let tickets: Vec<_> = tickets.iter().filter(| t | ticket_valid(t, &fields)).collect();

    let num_fields = fields.len();
    let mut unknown_fields = fields.clone();
    let mut known_fields: Vec<Option<Field>> = vec![];
    known_fields.resize(num_fields, None);
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..num_fields {
            match known_fields[i] {
                Some(_) => {},
                None => {
                    let possible = possible_fields(i, &tickets, &unknown_fields);
                    if possible.len() == 1 {
                        unknown_fields.retain(|a| a != &possible[0]);
                        known_fields[i] = Some(possible[0].clone());
                        changed = true;
                    }
                }
            }
        }
    }

    let mut departure_inds = vec![];
    for (i, field) in known_fields.iter().enumerate() {
        let prefix = field.clone().unwrap();
        let prefix = prefix.name.split(" ").collect::<Vec<_>>()[0];
        if prefix == "departure" {
            departure_inds.push(i);
        }
    }

    let mut res: i64 = 1;
    for i in departure_inds {
        println!("{}", res);
        res *= your_ticket[i] as i64;
    }

    println!("Result: {}", res);
}
