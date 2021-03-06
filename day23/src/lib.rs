use std::fmt;

#[derive(Default, Debug)]
struct State {
    cups: [u8; 9], // digits are 0 based: 0 through 8 inclusive
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..9 {
            write!(f, "{}", self.cups[i] + 1)?;
        }
        Ok(())
    }
}

impl State {
    fn new() -> State {
        Default::default()
    }

    fn do_round(&self) -> State {
        let mut next = State::new();
        let mut dest = (self.cups[0] + 8) % 9; // -1 mod 9
        while self.cups[1..4].contains(&dest) {
            dest = (dest + 8) % 9;
        }
        let mut j = 0;
        for i in 4..9 {
            next.cups[j] = self.cups[i];
            j += 1;
            if self.cups[i] == dest {
                next.cups[j] = self.cups[1];
                j += 1;
                next.cups[j] = self.cups[2];
                j += 1;
                next.cups[j] = self.cups[3];
                j += 1;
            }
        }
        next.cups[j] = self.cups[0];
        return next;
    }

    fn parse(&mut self, text: &str) {
        let chars: Vec<char> = text.chars().collect();
        for i in 0..9 {
            self.cups[i] = chars[i] as u8 - '1' as u8;
        }
    }

    pub fn dump(&self) {
        for i in 0..9 {
            print!("{}", self.cups[i] + 1);
        }
        println!("");
    }
}

struct Game {
    start: u32,     // index
    cups: Vec<u32>, // each cup knows the index of the next cup
}

const MILL: u32 = 1000u32 * 1000;
const MILLMIN1: u32 = MILL - 1u32;

// sub 1 modulo a million.
fn decrement(i: u32) -> u32 {
    let mut a = i;
    if a == 0 {
        a = MILLMIN1;
    } else {
        a -= 1;
    }
    return a;
}

impl Game {
    fn new() -> Game {
        let mut g = Game {
            start: 0,
            cups: Vec::with_capacity(1000 * 1000),
        };
        g.cups.resize(1000000, 0);
        for i in 0u32..(MILL - 1) {
            g.cups[i as usize] = i + 1;
        }
        g.cups[MILL as usize - 1] = 0;
        return g;
    }

    fn do_round(&mut self) {
        let mut dest = decrement(self.start);
        let n = self.start;
        let np1 = self.cups[n as usize];
        let np2 = self.cups[np1 as usize];
        let np3 = self.cups[np2 as usize];
        let np4 = self.cups[np3 as usize];

        // modify dest to not include the np1,np2,np3 cups:
        while (dest == np1) || (dest == np2) || (dest == np3) {
            dest = decrement(dest);
        }

        // delete cups n+1 through n+3:
        self.cups[self.start as usize] = np4;

        // insert cups n+1 through n+3 at the destination:
        let dest_next = self.cups[dest as usize];
        self.cups[dest as usize] = np1;
        self.cups[np3 as usize] = dest_next;

        // update self.start for next round:
        self.start = np4;
    }

    fn do_n_rounds(&mut self, n: u32) {
        for _i in 0..n {
            self.do_round();
        }
    }

    fn parse(&mut self, text: &str) {
        let chars: Vec<char> = text.chars().collect();
        self.start = chars[0] as u32 - '1' as u32;
        self.cups[1000 * 1000 - 1] = self.start;
        // if "462"
        // start=4
        // cups[4] = 6
        // cups[6] = 2
        // cups[2] = 9
        let mut j = self.start;
        for i in 1u32..9 {
            let next_index: u32 = chars[i as usize] as u32 - '1' as u32;
            self.cups[j as usize] = next_index;
            j = next_index;
        }
        self.cups[j as usize] = 9;
    }

    fn dump(&self) {
        self.dump_range(self.start, 11);
    }

    fn dump_range(&self, start: u32, count: u32) {
        let mut j = start;
        for _ in 0..count {
            print!("{},", j + 1);
            j = self.cups[j as usize];
        }
        println!("");
    }

    fn range_to_string(&self, start: u32, count: u32) -> String {
        let mut s = String::with_capacity(count as usize * 3); // guess
        let mut j = start;
        for _ in 0..count {
            s.push_str((j + 1).to_string().as_str());
            s.push(',');
            j = self.cups[j as usize];
        }
        return s;
    }
}

#[test]
fn test_state() {
    let mut s = State::new();
    s.parse("389125467");
    for _i in 0..10 {
        s = s.do_round();
    }
    s.dump();
    assert_eq!("837419265", s.to_string());
}

pub fn part_1() -> String {
    let mut s = State::new();
    s.parse("284573961");
    for _ in 0..100 {
        s = s.do_round();
    }
    return s.to_string();
}

#[test]
fn test_part_2() {
    // const EXPECTED: [&'static str, ..20] = [
    let expected = vec![
        "1000000,2,8,4,5,7,3,9,6,1,10,11,12,13,14,15,16,17,18,19,",
        "1000000,2,7,3,9,6,1,8,4,5,10,11,12,13,14,15,16,17,18,19,",
        "1000000,2,7,1,8,4,5,3,9,6,10,11,12,13,14,15,16,17,18,19,",
        "1000000,8,4,5,2,7,1,3,9,6,10,11,12,13,14,15,16,17,18,19,",
        "1000000,8,4,5,2,9,6,10,7,1,3,11,12,13,14,15,16,17,18,19,",
        "1000000,8,4,5,2,9,6,10,12,13,14,7,1,3,11,15,16,17,18,19,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,7,1,3,11,15,19,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,7,1,3,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,24,25,26,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,24,25,26,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,24,25,26,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,24,25,26,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,24,25,26,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,24,25,26,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,24,25,26,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,24,25,26,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,24,25,26,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,24,25,26,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,24,25,26,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,24,25,26,",
        "1000000,8,4,5,2,9,6,10,12,13,14,16,17,18,20,21,22,24,25,26,",
    ];
    let mut game = Game::new();
    game.parse("284573961");
    assert_eq!(expected[0], game.range_to_string(999999, 20));
    for i in 0..20 {
        game.do_round();
        assert_eq!(expected[i + 1], game.range_to_string(999999, 20));
    }
}

pub fn part_2() -> String {
    let mut game = Game::new();
    game.parse("284573961");
    game.do_n_rounds(10 * 1000 * 1000);
    let a = game.cups[0] as u64 + 1;
    let b = game.cups[game.cups[0] as usize] as u64 + 1;
    let c = a * b;
    assert_eq!(166298218695, c);
    return c.to_string();
}
