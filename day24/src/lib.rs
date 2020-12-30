#![allow(dead_code)]

use std::collections::HashSet;
use std::fs;
use std::ops::Add;
use std::ops::AddAssign;

// Hexagonal coordinate system:
// even rows have an implicit x+0.5 shift.
//
//     0,3  1,3  2,3 3,3
//  0,2  1,2  2,2  3,2       N
//     0,1  1,1  2,1  3,1  W-+-E
//  0,0  1,0  2,0  3,0       S
//
// So, the movement delta from even and odd rows differ:
//  from 1,1: NW=1,2 ( 0, 1)  NE=2,2 ( 1,1)  SW=1,0 (0, -1) SE=2,0 (1,-1)
//  from 2,2: NW=1,3 (-1, 1)  NE=2,3 ( 0,1)  SW=1,1 (-1,-1) SE=2,1 (0,-1)

// HexCoord: coordinate on a hexagonal layout.
#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct HexCoord(isize, isize);

const DELTA_W: HexCoord = HexCoord(-1, 0);
const DELTA_E: HexCoord = HexCoord(1, 0);
// DELTA for odd rows:
const DELTA_ODD_NW: HexCoord = HexCoord(0, 1);
const DELTA_ODD_NE: HexCoord = HexCoord(1, 1);
const DELTA_ODD_SW: HexCoord = HexCoord(0, -1);
const DELTA_ODD_SE: HexCoord = HexCoord(1, -1);
// DELTA for even rows is DELTA_ODD minus 1.

impl Add for HexCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for HexCoord {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1);
    }
}

impl HexCoord {
    fn offset(self) -> HexCoord {
        // Returns -1 if current row is even, 0 otherwise.
        if (self.1 & 0x1) == 0 {
            HexCoord(-1, 0)
        } else {
            HexCoord(0, 0)
        }
    }

    fn go_e(&self) -> HexCoord {
        *self + DELTA_E
    }
    fn go_w(&self) -> HexCoord {
        *self + DELTA_W
    }
    fn go_ne(&self) -> HexCoord {
        *self + DELTA_ODD_NE + self.offset()
    }
    fn go_nw(&self) -> HexCoord {
        *self + DELTA_ODD_NW + self.offset()
    }
    fn go_se(&self) -> HexCoord {
        *self + DELTA_ODD_SE + self.offset()
    }
    fn go_sw(&self) -> HexCoord {
        *self + DELTA_ODD_SW + self.offset()
    }

    fn get_neighbors(&self) -> [HexCoord; 6] {
        [
            self.go_ne(),
            self.go_nw(),
            self.go_se(),
            self.go_sw(),
            self.go_e(),
            self.go_w(),
        ]
    }

    fn from_path(path: &str) -> HexCoord {
        let mut p = HexCoord(0, 0);
        let mut it = path.chars();
        loop {
            let ch = it.next();
            if ch.is_none() {
                break;
            }
            let ch = ch.unwrap();
            p = match ch {
                'e' => p.go_e(),
                'w' => p.go_w(),
                'n' => {
                    let c2 = it.next().unwrap();
                    match c2 {
                        'e' => p.go_ne(),
                        'w' => p.go_nw(),
                        _ => panic!("bad direction: n{}", c2),
                    }
                }
                's' => {
                    let c2 = it.next().unwrap();
                    match c2 {
                        'e' => p.go_se(),
                        'w' => p.go_sw(),
                        _ => panic!("bad direction"),
                    }
                }
                _ => panic!("bad direction: {}", ch),
            }
        }
        return p;
    }
}

#[derive(Debug, Clone)]
struct Tiles {
    set: HashSet<HexCoord>,
}

impl Tiles {
    fn new() -> Tiles {
        Tiles {
            set: HashSet::new(),
        }
    }

    fn parse(&mut self, text: &str) {
        for line in text.lines() {
            let p = HexCoord::from_path(line);
            if self.set.contains(&p) {
                self.set.remove(&p);
            } else {
                self.set.insert(p);
            }
        }
    }

    // Count all adjacent black tiles.
    fn count_neighbors(&self, p: HexCoord) -> usize {
        return p
            .get_neighbors()
            .iter()
            .filter(|n| self.set.contains(*n))
            .count();
    }

    fn step(&self) -> Tiles {
        let next = self.clone();
        // TODO with be...
        return next;
    }

    // Count all black tiles in the set.
    fn count_all(&self) -> usize {
        return self.set.len();
    }
}

#[test]
fn test_hexcoord_from_path() {
    assert_eq!(HexCoord(0, 0), HexCoord::from_path(""));
    assert_eq!(HexCoord(0, 1), HexCoord::from_path("ne"));
    assert_eq!(HexCoord(-1, 1), HexCoord::from_path("nw"));
    assert_eq!(HexCoord(0, -1), HexCoord::from_path("se"));
    assert_eq!(HexCoord(-1, -1), HexCoord::from_path("sw"));
    assert_eq!(HexCoord(1, 0), HexCoord::from_path("e"));
    assert_eq!(HexCoord(-1, 0), HexCoord::from_path("w"));
    assert_eq!(HexCoord(1, 2), HexCoord::from_path("nene"));
    assert_eq!(HexCoord(0, 2), HexCoord::from_path("nenw"));
}

#[test]
fn test_part1() {
    assert_eq!(254, part1());
}

pub fn part1() -> usize {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Loaded {} bytes", contents.len());
    let mut tiles = Tiles::new();
    tiles.parse(&contents);
    return dbg!(tiles.count_all());
}
