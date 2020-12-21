use day19::*;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let parts: Vec<&str> = contents.split("\n\n").collect();

    let part1_count = count_matches(parts[0], parts[1], false);
    dbg!(part1_count);

    let part2_count = count_matches(parts[0], parts[1], true);
    dbg!(part2_count);

    // Prevent future breakage when optimizing:
    assert_eq!(113, part1_count);
    assert_eq!(253, part2_count);
}
