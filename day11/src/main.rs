use advent;

#[derive(Clone, Debug)]
struct SeatMap {
    seats: Vec<u8>,
    width: i32,
    height: i32,
}

impl SeatMap {
    fn new() -> SeatMap {
        SeatMap {
            seats: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn parse(&mut self, text: &str) {
        self.seats = Vec::new();
        for line in text.lines() {
            let l = line.trim();
            if l == "" {
                continue;
            };
            let bytes = l.as_bytes();
            self.seats.extend_from_slice(&bytes);
            if self.width == 0 {
                self.width = l.len() as i32;
            } else {
                assert_eq!(l.len() as i32, self.width);
            }
            self.height += 1;
        }
    }

    fn pretty_print(&self) {
        for row in 0..self.height {
            print!("{}: ", row);
            for col in 0..self.width {
                let x = self.get(row, col);
                print!("{}", x as char);
            }
            println!("");
        }
    }

    fn get(&self, row: i32, col: i32) -> u8 {
        if (row < 0) || (col < 0) || (row >= self.height) || (col >= self.width) {
            return '.' as u8;
        }
        let index = row * self.width + col;
        return self.seats[index as usize];
    }

    fn set(&mut self, row: i32, col: i32, v: u8) {
        if (row < 0) || (col < 0) || (row >= self.height) || (col >= self.width) {
            panic!("invalid coordinate row {}, col {}", row, col);
        }
        let index = row * self.width + col;
        self.seats[index as usize] = v;
    }

    fn is_occupied(&self, row: i32, col: i32) -> bool {
        return self.get(row, col) == '#' as u8;
    }

    fn has_chair(&self, row: i32, col: i32) -> bool {
        return self.get(row, col) != '.' as u8;
    }

    fn count_occupants(&self, row: i32, col: i32) -> u8 {
        return if self.is_occupied(row, col) { 1 } else { 0 };
    }

    fn count_adjacent_occupants(&self, row: i32, col: i32) -> u8 {
        return self.count_occupants(row - 1, col - 1)
            + self.count_occupants(row - 1, col + 0)
            + self.count_occupants(row - 1, col + 1)
            + self.count_occupants(row + 0, col - 1)
            + self.count_occupants(row + 0, col + 1)
            + self.count_occupants(row + 1, col - 1)
            + self.count_occupants(row + 1, col + 0)
            + self.count_occupants(row + 1, col + 1);
    }

    // returns 1 if an occupant is seen, 0 otherwise.
    fn look(&self, row: i32, col: i32, deltarow: i32, deltacol: i32) -> u8 {
        let mut x: u8 = '.' as u8;
        let mut r = row;
        let mut c = col;
        loop {
            r += deltarow;
            c += deltacol;
            if (r < 0) || (r >= self.height) || (c < 0) || (c >= self.width) {
                break;
            }
            x = self.get(r, c);
            if x != '.' as u8 {
                break;
            }
        }
        return if x == '#' as u8 { 1 } else { 0 };
    }

    fn count_visible_occupants(&self, row: i32, col: i32) -> u8 {
        return self.look(row, col, -1, -1)
            + self.look(row, col, -1, 0)
            + self.look(row, col, -1, 1)
            + self.look(row, col, 0, -1)
            + self.look(row, col, 0, 1)
            + self.look(row, col, 1, -1)
            + self.look(row, col, 1, 0)
            + self.look(row, col, 1, 1);
    }

    fn count_all_occupants(&self) -> u32 {
        return self
            .seats
            .iter()
            .map(|x| if *x == '#' as u8 { 1 } else { 0 })
            .sum();
    }

    fn run_one_step(&mut self, part2: bool) -> bool {
        let mut changed = false;
        let mut counts: Vec<u8> = self.seats.iter().map(|_| 0u8).collect();
        for row in 0..self.height {
            for col in 0..self.width {
                let index = row * self.width + col;
                if part2 {
                    counts[index as usize] = self.count_visible_occupants(row, col);
                } else {
                    counts[index as usize] = self.count_adjacent_occupants(row, col);
                }
            }
        }
        let threshold = if part2 { 5 } else { 4 };
        for row in 0..self.height {
            for col in 0..self.width {
                let index = row * self.width + col;
                if self.has_chair(row, col) {
                    if self.is_occupied(row, col) {
                        // seat is occupied
                        if counts[index as usize] >= threshold {
                            self.set(row, col, 'L' as u8);
                            changed = true;
                        }
                    } else {
                        // seat is empty
                        if counts[index as usize] == 0 {
                            self.set(row, col, '#' as u8);
                            changed = true;
                        }
                    }
                } // if seat has chair
            } // for col
        } // for row
        return changed;
    }

    fn run(&mut self, part2: bool) -> u32 {
        let mut steps = 0u32;
        while self.run_one_step(part2) {
            steps += 1
        }
        return steps;
    }
}

fn visibility_test(text: &str, row: i32, col: i32, visible: u8) {
    let mut seatmap_visibility_test1 = SeatMap::new();
    seatmap_visibility_test1.parse(text);
    seatmap_visibility_test1.pretty_print();
    assert_eq!(
        seatmap_visibility_test1.count_visible_occupants(row, col),
        visible
    );
    println!("ok: {},{} can see {}", row, col, visible);
}

fn run_test(text: &str, final_count: u32) {
    let mut seatmap = SeatMap::new();
    seatmap.parse(text);
    let mut step = 0;
    loop {
        println!("step {}", step);
        step += 1;
        seatmap.pretty_print();
        let changed = seatmap.run_one_step(true);
        println!(
            "changed={} count={}",
            changed,
            seatmap.count_all_occupants()
        );
        if !changed {
            break;
        };
    }
    assert_eq!(final_count, seatmap.count_all_occupants());
}

fn main() {
    // Run some visibility_tests first:
    visibility_test(
        r"
      .##.##.
      #.#.#.#
      ##...##
      ...L...
      ##...##
      #.#.#.#
      .##.##.",
        3,
        3,
        0,
    );
    visibility_test(
        r"
        .......#.
        ...#.....
        .#.......
        .........
        ..#L....#
        ....#....
        .........
        #........
        ...#.....",
        4,
        3,
        8,
    );
    visibility_test(
        r"
    .............
    .L.L.#.#.#.#.
    .............",
        1,
        2,
        0,
    );
    visibility_test(
        r"
    .............
    .L.L.#.#.#.#.
    .............",
        1,
        4,
        1,
    );

    run_test(
        r"
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL",
        26,
    );

    let content = advent::load_input();
    let part1: u32;
    {
        let mut seatmap = SeatMap::new();
        seatmap.parse(&content);
        dbg!(seatmap.run(false));
        part1 = seatmap.count_all_occupants();
    }

    let part2: u32;
    {
        let mut seatmap = SeatMap::new();
        seatmap.parse(&content);
        dbg!(seatmap.run(true));
        part2 = seatmap.count_all_occupants();
    }

    dbg!(part1);
    dbg!(part2);
}
