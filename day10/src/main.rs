use advent;

#[derive(Debug)]
struct Adapters {
    joltage: Vec<i64>,
    sequence: Vec<i64>,
}

// Takes a length of delta-1 sequence and returns the number of legal
// mutations of that sequence (including the original).
fn length_to_mutations(l: usize) -> u128 {
    // All 1-delta subsequences are isomorphic with the following examples.
    // The first joltage and the final joltage are fixed, but some of the
    // steps in between can be removed as long as no joltage jump is greater
    // than 3.
    //
    // length 1: 0 1  --> 1 mutation
    // length 2: 0 1 2
    //           0   2  --> 2 mutations (the 1 can be dropped)
    // length 3: 0 1 2 3
    //           0   2 3
    //           0 1   3
    //           0     3 --> 4 mutations
    // length 4: 0 1 2 3 4
    //           0   2 3 4
    //           0 1   3 4
    //           0 1 2   4
    //           0   2   4
    //           0     3 4
    //           0 1     4 -> 7 mutations
    return match l {
        1 => 1u128,
        2 => 2u128,
        3 => 4u128,
        4 => 7u128,
        _ => 0u128,
    };
}

impl Adapters {
    fn new() -> Adapters {
        Adapters {
            joltage: Vec::new(),
            sequence: Vec::new(),
        }
    }

    fn parse(&mut self, text: &str) {
        self.joltage = text.lines().map(|x| x.parse::<i64>().unwrap()).collect();
        self.sequence = self.joltage.clone();
        // add the starting joltage:
        self.sequence.push(0);
        // add the final joltage:
        self.sequence.push(self.sequence.iter().max().unwrap() + 3);
        self.sequence.sort();
        dbg!(&self.sequence);
        dbg!(self.sequence.len());
    }

    fn count_deltas(&self) -> Vec<u64> {
        let mut counters: Vec<u64> = vec![0u64, 0, 0, 0];
        for i in 1..self.sequence.len() {
            let delta = (self.sequence[i] - self.sequence[i - 1]) as usize;
            counters[delta] += 1;
        }
        return counters;
    }

    fn count_subsequences(&self) -> Vec<usize> {
        let mut lengths: Vec<usize> = Vec::new();
        let mut count = 0usize;
        for i in 1..self.sequence.len() {
            let delta = (self.sequence[i] - self.sequence[i - 1]) as usize;
            if delta == 1 {
                count += 1
            } else if count != 0 {
                let j = i - count;
                println!("{}:{} = {:?}", j, i, &self.sequence[j..i]);
                lengths.push(count);
                count = 0;
            }
        }
        if count != 0 {
            lengths.push(count);
        }
        return lengths;
    }

    fn count_legal_combos(&self) -> u128 {
        // count up the various 1-delta subsequences.
        let lengths: Vec<usize> = self.count_subsequences();
        return lengths.iter().map(|x| length_to_mutations(*x)).product();
    }
}

fn main() {
    let content = advent::load_input();
    let mut adapters = Adapters::new();
    adapters.parse(&content);
    let deltas = dbg!(adapters.count_deltas());
    let part1 = deltas[1] * deltas[3];

    dbg!(adapters.count_subsequences());
    let part2 = adapters.count_legal_combos();
    assert_eq!(3454189699072, part2); // now that I know the answer...

    dbg!(part1);
    dbg!(part2);
}
