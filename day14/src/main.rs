#[macro_use]
extern crate lazy_static;
extern crate regex;

use advent;
use regex::Regex;
use std::collections::HashMap;

type DataType = u64;

#[derive(Clone, Debug)]
struct Mask {
    ones_mask: DataType,
    zeros_mask: DataType,
    xlist: Vec<u8>,
}

impl Mask {
    fn new(s: &str) -> Mask {
        let mut m = Mask {
            ones_mask: 0,
            zeros_mask: 0,
            xlist: Vec::new(),
        };
        let mut bitpos: u8 = s.len() as u8;
        assert_eq!(36, bitpos);
        for ch in s.chars() {
            bitpos -= 1;
            m.ones_mask <<= 1;
            m.zeros_mask <<= 1;
            match ch {
                '0' => m.zeros_mask |= 1, // mask = mask | 1;  mask = mask + 1
                '1' => m.ones_mask |= 1,
                'X' => m.xlist.push(bitpos),
                _ => panic!("Wat {}", ch),
            }
        }
        m.xlist.reverse(); // put the LSB first.
        return m;
    }

    fn xmask(&self) -> DataType {
        return !(self.ones_mask | self.zeros_mask);
    }

    // DataType is u64
    fn apply(&self, w: DataType) -> DataType {
        return (w | self.ones_mask) & !self.zeros_mask;
    }

    fn addr_iter(&self, a: DataType) -> MaskIterator {
        MaskIterator::new(self.clone(), a)
    }
} // impl Mask

// My first iterator!
struct MaskIterator {
    index: u64,
    max: u64,
    base: u64,
    xlist: Vec<u8>,
}

impl MaskIterator {
    fn new(m: Mask, a: u64) -> MaskIterator {
        let mut it = MaskIterator {
            index: 0,
            max: 0,
            base: a,
            xlist: m.xlist.clone(),
        };
        it.base |= m.ones_mask;
        it.base &= !m.xmask();
        it.max = (1u64 << it.xlist.len()) - 1; // inclusive bound
        return it;
    }
}

impl Iterator for MaskIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.index > self.max {
            return None;
        }
        let mut a = self.base;
        for bitpos in 0..self.xlist.len() {
            if (self.index & (1 << bitpos)) != 0 {
                a |= 1 << self.xlist[bitpos];
            }
        }
        self.index += 1;
        return Some(a);
    }
}

struct Memory {
    data: HashMap<u64, u64>,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            data: HashMap::new(),
        }
    }

    fn get(&self, i: u64) -> u64 {
        return self.data.get(&i).cloned().unwrap_or(0);
    }

    fn set(&mut self, i: u64, v: u64) {
        self.data.insert(i, v);
    }

    fn sum(&self) -> u64 {
        self.data.values().sum()
    }

    fn bigsum(&self) -> u128 {
        self.data.values().map(|x| *x as u128).sum()
    }
}

enum Instruction {
    MaskOp(Mask),
    Store(u64, u64),
}

impl Instruction {
    fn new(s: &str) -> Instruction {
        lazy_static! {
            static ref RE_STORE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        }
        if &s[0..7] == "mask = " {
            return Instruction::MaskOp(Mask::new(&s[7..]));
        }
        let caps = RE_STORE.captures(s).unwrap();
        return Instruction::Store(
            caps.get(1).unwrap().as_str().parse::<u64>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<u64>().unwrap(),
        );
    }
}

struct System {
    mask: Mask,
    mem: Memory,
}

type Program = Vec<Instruction>;

fn parse_program(content: &str) -> Program {
    content
        .lines()
        .map(|line| Instruction::new(line.trim()))
        .collect()
}

impl System {
    fn new() -> System {
        System {
            mask: Mask::new("000000000000000000000000000000000000"),
            mem: Memory::new(),
        }
    }

    fn run(&mut self, program: &Program) {
        for instruction in program.iter() {
            match instruction {
                Instruction::MaskOp(m) => self.mask = m.clone(),
                Instruction::Store(a, d) => self.store(*a, *d),
            }
        }
    }

    fn run_part2(&mut self, program: &Program) {
        for instruction in program.iter() {
            match instruction {
                Instruction::MaskOp(m) => self.mask = m.clone(),
                Instruction::Store(a, d) => self.store_part2(*a, *d),
            }
        }
    }

    fn store(&mut self, a: u64, d: u64) {
        let v = self.mask.apply(d);
        self.mem.set(a, v);
    }

    fn store_part2(&mut self, a: u64, d: u64) {
        for address in self.mask.addr_iter(a) {
            self.mem.set(address, d);
        }
    }
}

fn part1(content: &str) -> u64 {
    let mut system = System::new();
    let program = parse_program(content);
    system.run(&program);
    system.mem.sum()
}

fn part2(content: &str) -> u128 {
    let mut system = System::new();
    let program = parse_program(content);
    system.run_part2(&program);
    system.mem.bigsum()
}

fn main() {
    let content = advent::load_input();

    dbg!(part1(&content));
    dbg!(part2(&content));
}
