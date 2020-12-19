use std::fs;
use std::collections;

#[derive(Copy,Clone,Debug,Default)]
struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn new(text: &str) -> Range {
        let p: Vec<&str> = text.split("-").collect();
        Range {
            min: p[0].parse::<u32>().unwrap(),
            max: p[1].parse::<u32>().unwrap(),
        }
    }

    fn check(&self, i: u32) -> bool {
        (i >= self.min) && (i <= self.max)
    }
}

#[derive(Clone,Debug,Default)]
struct Field {
    name: String,
    id: u32,
    rules: Vec<Range>,
}

impl Field {
    fn new(line: &str, id: u32) -> Field {
        let mut f = Field {name: "".to_string(), id, rules: Vec::new()};
        let p1: Vec<&str> = line.split(": ").collect();
        f.name = p1[0].to_owned();
        let p2: Vec<&str> = p1[1].split(" or ").collect();
        for p in p2 {
            f.rules.push(Range::new(p));
        }
        return f;
    }

    fn check(&self, i: u32) -> bool {
        for r in &self.rules {
            if r.check(i) { return true; }
        }
        return false;
    }
}

#[derive(Clone,Debug,Default)]
struct Matcher {
    fields: Vec<Field>,
    map: collections::HashMap<String, u32>,
}

impl Matcher {
    fn new() -> Matcher {
        Matcher {
            fields: Vec::new(),
            map: collections::HashMap::new(),
        }
    }

    fn find_field(&self, value: u32) -> Option<String> {
        for field in &self.fields {
            if field.check(value) {
                return Some(field.name.to_owned());
            }
        }
        return None;
    }

    fn calc_error_rate(&self, value: u32) -> u32 {
        if self.find_field(value).is_none() {
            return value;
        } else {
            return 0u32;
        }
    }

    fn bitmap_of_valid_fields(&self, value: u32) -> u32 {
        let mut bitmap: u32 = 0;
        for field in &self.fields {
            if field.check(value) {
                bitmap |= field.id;
            }
        }
        return bitmap;
    }

    fn parse(&mut self, text: &str) {
        let mut id = 1u32;  // id is also bit position.
        for line in text.lines() {
            let line = line.trim();
            let field = Field::new(line, id);
            self.fields.push(field);
            id <<= 1;
        }
    }
}

#[derive(Clone,Debug,Default)]
struct Ticket {
    values: Vec<u32>,
}

impl Ticket {
    fn new(text: &str) -> Ticket {
        Ticket {
            values: text.split(",")
                .map(|a| a.parse::<u32>().unwrap())
                .collect(),
        }
    }

    fn calc_error_rate(&self, m: &Matcher) -> u32 {
        let mut acc = 0u32;
        for value in &self.values {
            acc += m.calc_error_rate(*value);
        }
        return acc;
    }

    fn p1_is_valid(&self, m: &Matcher) -> bool {
        for value in &self.values {
            if m.find_field(*value).is_none() {
                return false;
            }
        }
        return true;
    }

}

struct TicketDecoder {
    field_id: Vec<u32>,  // maps position to bitmap
}

fn bitmap_has_one_bit_set(bitmap: u32) -> bool {
    return (bitmap & (bitmap - 1)) == 0;
}

impl TicketDecoder {
    fn new() -> TicketDecoder {
        TicketDecoder {
            field_id: Vec::new(),
        }
    }

    fn id_to_position(&self, id: u32) -> u32 {
        return self.field_id.iter().position(|x| *x as u32 == id).unwrap() as u32;
    }

    fn guess_fields(&mut self, matcher: &Matcher, tickets: &Vec<Ticket>) {
        let positions = tickets[0].values.len() as u32;
        self.field_id.clear();
        self.field_id.resize(positions as usize, 0u32);
        for position in 0u32..positions {
            let mut bitmask: u32 = 0xFFFFFFFF;
            for ticket in tickets.iter().filter(|x| x.p1_is_valid(&matcher)) {
                let value = ticket.values[position as usize];
                bitmask &= matcher.bitmap_of_valid_fields(value);
            }
            self.field_id[position as usize] = bitmask;
        }
        // Now use process of elimninatio to cull the bitmasks.
        let mut iterations = 0;
        loop {
            iterations += 1;
            dbg!(iterations);
            let mut done: bool = true;
            for position in 0u32..positions {
                let bm = self.field_id[position as usize];
                if bitmap_has_one_bit_set(bm) {
                    for other_pos in 0u32..positions {
                        if other_pos == position { continue; }
                        let mut other_bm = self.field_id[other_pos as usize];
                        other_bm &= !bm;
                        self.field_id[other_pos as usize] = other_bm;
                    }
                } else {
                    done = false;
                }
            }
            if done { break; }
        }
        for position in 0u32..positions {
            let bitmask = self.field_id[position as usize];
            println!("Position {} -> bitmask {:#X}", position, bitmask);
            assert_eq!(bitmask & (bitmask - 1), 0, "bitmask had more than one bit set!");
        }

    }
}

fn main() {
    let inputfile = "input.txt";
    let contents = fs::read_to_string(&inputfile)
        .expect("Something went wrong reading the file");
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let mut matcher = Matcher::new();
    matcher.parse(parts[0]);
    dbg!(&matcher);

    let mut ticket_lines = parts[2].lines();
    ticket_lines.next();  // skip first line.
    let tickets: Vec<Ticket> = ticket_lines
        .map(|x| Ticket::new(x))
        .collect();
    dbg!(&tickets);

    let p1_valid_cnt = tickets.iter().filter(|&x| x.p1_is_valid(&matcher)).count();
    dbg!(p1_valid_cnt);
    let mut p1_error_rate = 0u32;
    for ticket in &tickets {
        p1_error_rate += ticket.calc_error_rate(&matcher);
    }
    dbg!(p1_error_rate);

    // part 2
    let my_ticket = Ticket::new(parts[1].lines().nth(1).unwrap());
    let mut decoder = TicketDecoder::new();
    decoder.guess_fields(&matcher, &tickets);
    let product: u64 = matcher.fields.iter()
        .filter(|f| f.name.starts_with("departure"))
        .map(|f| f.id)
        .map(|id| decoder.id_to_position(id))
        .map(|pos| my_ticket.values[pos as usize] as u64)
        .product();
    dbg!(product);
}
