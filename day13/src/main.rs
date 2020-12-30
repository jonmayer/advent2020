// advent 2020 - day 13
//
// Be Han helped with this solution!

use advent;
use howlong;

// PART 1

fn part1(content: &str) -> i64 {
    // part 1
    let lines: Vec<&str> = content.lines().collect();
    let timestamp = lines[0].parse::<i64>().unwrap();
    let bus_ids: Vec<i64> = lines[1]
        .split(",")
        .filter(|x| *x != "x")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    dbg!(&bus_ids);
    // find the bus with the least wait time.
    // wait time is bus interval - (timestamp % interval).
    let remainders: Vec<i64> = bus_ids.iter().map(|x| x - (timestamp % x)).collect();
    dbg!(&remainders);
    let (min_i, min_v) = remainders
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    dbg!(min_i, min_v);
    return bus_ids[min_i] * min_v;
}

// PART 2

// True iff all the items in a vector are equal.
fn all_equal(x: &Vec<i128>) -> bool {
    for v in x[1..].iter() {
        if *v != x[0] {
            return false;
        };
    }
    return true;
}

// A brute force implementation for solving the system of equations.  This approach is predictably
// too slow for this problem, but I wanted something reliable to regress my fancier design against.
//
// k(i) * ids(i) - offsets(i) = k(j) * ids(j) - offsets(j)
// for all i,j
//
// returns that value.
fn bf_converge_vector(ids: Vec<i128>, offsets: Vec<i128>) -> i128 {
    let mut k: Vec<i128> = ids.iter().map(|_| 1).collect();
    let mut step = 0;
    loop {
        // Compute r = k * ids - offset
        // Could have used ndarray but didn't want to open a new crate.
        let r = ids
            .iter()
            .zip(k.iter())
            .map(|(a, b)| a * b)
            .zip(offsets.iter())
            .map(|(a, b)| a - b)
            .collect();

        if step % 100 == 0 {
            println!("step {}", step);
        }
        step += 1;

        if all_equal(&r) {
            break;
        }

        // Increment k for the element with the lowest r value.
        let (min_i, _) = r
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();
        k[min_i] += 1;
    }
    return k[0] * ids[0];
}

// PART 2 - the fancy approach
//
// Instead of thinking of the buses as having staggered arrival times, I decided that it was
// conceptually easier to give the buses staggered start times, and solve for a timestamp
// where they would all arrive simultaneously.
//
// This approach takes a pair of buses and finds the first time those buses intersect, and
// the interval with which they intersection will repeat.  From that, we can synthesize
// a new bus with a new interval and start_time (the time of the first intersection).
//
// By combining one bus at a time into our synthetic super-bus, we can iterate through
// the vector of buses and find the timestamp where all buses show up at the same time.
// This is isomorphic with the buses showing up on a staggered schedule, since we moved
// the staggering offset to be staggered start_times instead.

#[derive(Clone, Debug)]
struct Bus {
    interval: i128,
    start_time: i128,
}

impl Bus {
    // Make a super bus by combining this bus with another bus.
    fn combine_with_other_bus(&mut self, other: &Bus) {
        // We want to solve for n where:
        // (self.start_time + n * self.interval) % other.interval = (other.start_time % other.interval)
        let mut a0 = self.start_time;
        let seek = other.start_time.rem_euclid(other.interval);
        while (a0 % other.interval) != seek {
            a0 += self.interval;
        }

        // update bus:
        self.start_time = a0;
        self.interval = self.interval * other.interval;
    }
} // impl Bus

fn part2(text: &str) -> i128 {
    // Parse bus ids:
    let mut v: Vec<_> = text
        .split(",")
        .enumerate()
        .filter(|(_, id)| *id != "x")
        .collect();
    v.sort_by(|(_, a), (_, b)| b.cmp(a));
    dbg!(&v);

    // Construct bus ids to Bus objects, using index as the staggered start_time.
    let buses: Vec<Bus> = v
        .iter()
        .map(|(i, id)| Bus {
            interval: id.parse::<i128>().unwrap(),
            start_time: (*i as i128) * -1,
        })
        .collect();
    dbg!(&buses);
    let timer = howlong::HighResolutionTimer::new();

    // Combine all the buses into a super bus.
    let mut bus: Bus = buses[0].clone();
    for i in 1..buses.len() {
        bus.combine_with_other_bus(&buses[i]);
    }
    dbg!(timer.elapsed());
    return bus.start_time; // Timestamp of first convergence.
}

fn main() {
    let content = advent::load_input();

    dbg!(part1(&content));

    // brute force test of part2 solving
    let v: Vec<i128> = vec![3i128, 5, 7, 11];
    let o: Vec<i128> = vec![0i128, 1, 2, 3];
    dbg!(bf_converge_vector(v, o));

    // elegant faster solution over the same data:
    dbg!(part2(&"3,5,7,11"));

    // the real solution to the input data set:
    let lines: Vec<&str> = content.lines().collect();
    dbg!("solution: ", part2(&lines[1]));
}
