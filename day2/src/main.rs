use regex::Regex;
use std::fs;

fn main() {
    let inputfile = "input.txt";

    let contents = fs::read_to_string(inputfile).expect("Something went wrong reading the file");
    let lines = contents.lines();

    let re_line: Regex = Regex::new(r"^\s*(\d+)-(\d+)\s+(\w):\s+(\w+)\s*$").unwrap();

    // part 1 rules:
    let mut p1_valid = 0;

    // part 2 rules:
    let mut p2_valid = 0;

    for line in lines {
        let ro = re_line.captures(line);
        if ro.is_none() {
            println!("no match: {}", line);
            continue;
        }
        let ro = ro.unwrap();
        let min = ro.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let max = ro.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let chr: Vec<char> = ro.get(3).unwrap().as_str().chars().collect();
        let password: &str = ro.get(4).unwrap().as_str();
        let chr_count = password.chars().filter(|c| *c == chr[0]).count();
        if (chr_count >= min) && (chr_count <= max) {
            p1_valid += 1;
        }
        // part 2: "n-m c" indicates c must be present in either position n or m, but not both.
        // n and m are 1-indexed positions.
        let p1: usize = min - 1;
        let p2: usize = max - 1;
        let c1 = password.chars().nth(p1).unwrap();
        let c2 = password.chars().nth(p2).unwrap();
        let p2_ok = (c1 == chr[0]) ^ (c2 == chr[0]);
        if p2_ok {
            p2_valid += 1;
        }
    }
    println!("p1_valid: {}", p1_valid);
    println!("p2_valid: {}", p2_valid);
}
