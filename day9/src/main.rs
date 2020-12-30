use advent;
use std::collections;

#[derive(Debug)]
struct Decoder {
    window: collections::VecDeque<i64>,
}

impl Decoder {
    fn new() -> Decoder {
        Decoder {
            window: collections::VecDeque::new(),
        }
    }

    // returns true if the submitted number is valid
    fn consume(&mut self, num: i64) -> bool {
        let mut valid: bool = false;
        if self.window.len() < 25 {
            valid = true;
        } else {
            for i in 0..24 {
                for j in i..25 {
                    if self.window[i] + self.window[j] == num {
                        valid = true;
                        break;
                    }
                }
                if valid {
                    break;
                }
            }
        }

        self.window.push_back(num);
        while self.window.len() > 25 {
            self.window.pop_front();
        }

        return valid;
    }
}

fn main() {
    let content = advent::load_input();
    let numbers: Vec<i64> = content.lines().map(|x| x.parse::<i64>().unwrap()).collect();

    let mut part1_result: i64 = -1;
    let mut decoder = Decoder::new();
    for num in numbers.iter() {
        if !decoder.consume(*num) {
            part1_result = *num;
        }
    }

    let mut i: usize = 0;
    let mut j: usize = 1;
    let mut sum = numbers[i] + numbers[j];
    while sum != part1_result {
        if sum < part1_result {
            j += 1;
            sum += numbers[j];
        } else {
            sum -= numbers[i];
            i += 1;
        }
    }
    dbg!(&i, &j);
    let min = numbers[i..j + 1].iter().min().unwrap();
    let max = numbers[i..j + 1].iter().max().unwrap();
    let part2_result = min + max;

    dbg!(part1_result);
    dbg!(part2_result);
}
