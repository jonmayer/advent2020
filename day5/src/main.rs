use std::convert::TryInto;
use std::fs;

fn str_to_seat_id(s: &str) -> i32 {
    let mut value = 0i32;
    for ch in s.chars() {
        value *= 2;
        if (ch == 'B') || (ch == 'R') {
            value += 1;
        }
    }
    println!("{} {}", s, value);
    return value;
}

fn seat_id_to_row(id: i32) -> i32 {
    return id >> 3;
}

fn main() {
    // let inputfile = "example.txt";
    let inputfile = "input.txt";

    let contents = fs::read_to_string(inputfile).expect("Something went wrong reading the file");
    let seat_ids: Vec<i32> = contents.lines().map(|x| str_to_seat_id(x)).collect();
    dbg!(seat_ids.iter().max().unwrap());

    // Part 2:
    let min_row = seat_ids.iter().map(|&x| seat_id_to_row(x)).min().unwrap();
    let max_row = seat_ids.iter().map(|&x| seat_id_to_row(x)).max().unwrap();
    dbg!(min_row, max_row);
    let min_seatid: usize = (min_row << 3).try_into().unwrap();
    let max_seatid: usize = ((max_row << 3) + 7).try_into().unwrap();
    let mut idset: Vec<bool> = Vec::new();
    idset.resize(max_seatid + 1, false);
    for i in seat_ids {
        let id: usize = i.try_into().unwrap();
        idset[id] = true;
    }
    for id in (min_seatid + 8)..(max_seatid + 1 - 8) {
        if !idset[id] {
            println!("my seat: {}", id);
            assert_eq!(649, id, "want: 649, got: {}", id);
        }
    }
}
