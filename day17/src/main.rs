use day17::*;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    dbg!(part1(&contents));
    dbg!(part2(&contents));
}
