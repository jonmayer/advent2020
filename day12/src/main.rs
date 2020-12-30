#[macro_use]
extern crate lazy_static;

use advent;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Ship {
    facing: i32, // 0 is east, 90 is north, 180 is west, 270 is south
    x: i32,      // west is negative, east is positive
    y: i32,      // south is negative, north is positive
}

type ShipMethodPtr = fn(&mut Ship, i32);

impl Ship {
    fn new() -> Ship {
        Ship {
            facing: 0,
            x: 0,
            y: 0,
        }
    }

    // There are less verbose ways to do this, but I just want to learn how to use rust method
    // pointers:

    fn do_command_n(&mut self, value: i32) {
        self.y += value;
    }

    fn do_command_s(&mut self, value: i32) {
        self.y -= value;
    }

    fn do_command_e(&mut self, value: i32) {
        self.x += value;
    }

    fn do_command_w(&mut self, value: i32) {
        self.x -= value;
    }

    fn do_command_l(&mut self, value: i32) {
        self.facing = (self.facing + value + 360) % 360;
    }

    fn do_command_r(&mut self, value: i32) {
        self.facing = (self.facing - value + 360) % 360;
    }

    fn do_command_f(&mut self, value: i32) {
        match self.facing {
            0 => self.do_command_e(value),
            90 => self.do_command_n(value),
            180 => self.do_command_w(value),
            270 => self.do_command_s(value),
            _ => panic!("invalid facing {}", self.facing),
        };
    }

    fn do_command(&mut self, text: &str) {
        lazy_static! {
            static ref CMD_MAP: HashMap<&'static str, ShipMethodPtr> = {
                let mut map: HashMap<&'static str, ShipMethodPtr> = HashMap::new();
                map.insert("N", Ship::do_command_n);
                map.insert("S", Ship::do_command_s);
                map.insert("E", Ship::do_command_e);
                map.insert("W", Ship::do_command_w);
                map.insert("L", Ship::do_command_l);
                map.insert("R", Ship::do_command_r);
                map.insert("F", Ship::do_command_f);
                map
            };
        }

        println!("{}", text);
        let cmd = &text[0..1];
        let value = text[1..].parse::<i32>().unwrap();
        let method = CMD_MAP[cmd];
        method(self, value);
    }

    fn do_all_commands(&mut self, text: &str) {
        for line in text.lines() {
            self.do_command(line.trim());
        }
    }

    fn manhattan_distance(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
}

// part 2

#[derive(Clone, Debug)]
struct Ship2 {
    x: i32,  // west is negative, east is positive
    y: i32,  // south is negative, north is positive
    wx: i32, // waypoint x
    wy: i32, // waypoint y
}
impl Ship2 {
    fn new() -> Ship2 {
        Ship2 {
            x: 0,
            y: 0,
            wx: 10,
            wy: 1,
        }
    }

    // positive angle means turn left, negative angle means turn right
    fn rotate(&mut self, angle: i32) {
        let mut a = angle;
        // rotate counterclockwise (left)
        while a > 0 {
            let temp_wy = self.wy;
            self.wy = self.wx;
            self.wx = -temp_wy;
            a -= 90;
        }
        // rotate clockwise (right)
        while a < 0 {
            let temp_wy = self.wy;
            self.wy = -self.wx;
            self.wx = temp_wy;
            a += 90;
        }
    }

    fn move_towards_waypoint(&mut self, value: i32) {
        self.x += self.wx * value;
        self.y += self.wy * value;
    }

    fn do_command(&mut self, text: &str) {
        let cmd = &text[0..1];
        let value = text[1..].parse::<i32>().unwrap();
        match cmd {
            "N" => self.wy += value,
            "S" => self.wy -= value,
            "E" => self.wx += value,
            "W" => self.wx -= value,
            "L" => self.rotate(value),
            "R" => self.rotate(-value),
            "F" => self.move_towards_waypoint(value),
            _ => panic!("invalid cmd {}", cmd),
        };
    }

    fn do_all_commands(&mut self, text: &str) {
        for line in text.lines() {
            self.do_command(line);
        }
    }

    fn manhattan_distance(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }

    fn assert_waypoint(&self, want_wx: i32, want_wy: i32) {
        assert!(want_wx == self.wx, "want {} wx={}", want_wx, self.wx);
        assert!(want_wy == self.wy, "want {} wy={}", want_wy, self.wy);
    }

    fn assert_position(&self, want_x: i32, want_y: i32) {
        assert!(want_x == self.x, "want {} x={}", want_x, self.x);
        assert!(want_y == self.y, "want {} y={}", want_y, self.y);
    }
}

fn test() {
    // Quick regression test.
    println!("test...");
    let mut ship = Ship2::new();
    ship.assert_waypoint(10, 1);
    ship.assert_position(0, 0);

    ship.do_command("F10");
    ship.assert_waypoint(10, 1);
    ship.assert_position(100, 10);

    ship.do_command("N3");
    ship.assert_waypoint(10, 4);
    ship.assert_position(100, 10);

    ship.do_command("F7");
    ship.assert_waypoint(10, 4);
    ship.assert_position(170, 38);

    ship.do_command("R90");
    ship.assert_waypoint(4, -10);
    ship.assert_position(170, 38);
}

fn main() {
    test();

    let content = advent::load_input();
    let part1: i32;
    {
        let mut ship = Ship::new();
        ship.do_all_commands(&content);
        part1 = ship.manhattan_distance();
    }

    let part2: i32;
    {
        let mut ship2 = Ship2::new();
        ship2.do_all_commands(&content);
        part2 = ship2.manhattan_distance();
    }

    dbg!(part1);
    dbg!(part2);
}
