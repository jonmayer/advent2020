// TODO: learn how to extend existing types (HashMap) using traits.
use howlong;
use std::collections::HashMap;

struct Tracker {
    seen: HashMap<u64, u64>,
    turn: u64, // last turn
}

impl Tracker {
    fn new() -> Tracker {
        Tracker {
            seen: HashMap::new(),
            turn: 0,
        }
    }

    // returns the number of turns since number was added, or 0 if number is new.
    fn append_number(&mut self, num: u64) -> u64 {
        self.turn += 1;
        let v = self.seen.insert(num, self.turn);
        match v {
            None => return 0,
            Some(x) => return self.turn - x,
        }
    }
}

fn run(starters: Vec<u64>, until: u64) -> u64 {
    let timer = howlong::HighResolutionTimer::new();
    let mut tracker = Tracker::new();
    let mut v: u64 = 0;
    for num in starters {
        v = tracker.append_number(num);
    }
    while tracker.turn < until - 1 {
        v = tracker.append_number(v);
    }
    let elapsed = timer.elapsed();
    println!("completed {} steps in {:?}", until, elapsed);
    return v;
}

fn main() {
    let starting = vec![5, 1, 9, 18, 13, 8, 0];
    dbg!(run(starting.clone(), 2020));
    dbg!(run(starting.clone(), 30000000));
}
