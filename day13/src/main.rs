// advent 2020 - day 13
//
// Be Han helped with this solution!

use advent;

// PART 1

fn part1(content: &str) -> i64 {
    // part 1
    let lines: Vec<&str> = content.lines().collect();
    let timestamp = lines[0].parse::<i64>().unwrap();
    let bus_ids: Vec<i64> = lines[1].split(",")
        .filter(|x| *x != "x")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    dbg!(&bus_ids);
    // find the bus with the least wait time.
    // wait time is bus interval - (timestamp % interval).
    let remainders: Vec<i64> = bus_ids.iter()
        .map(|x| x - (timestamp % x))
        .collect();
    dbg!(&remainders);
    let (min_i, min_v) = remainders.iter().enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    dbg!(min_i, min_v);
    return bus_ids[min_i] * min_v;
}

// PART 2

// True iff all the items in a vector are equal.
fn all_equal(x: &Vec<i128>) -> bool {
    for v in x[1..].iter() {
        if *v != x[0] { return false };
    }
    return true;
}

// A brute force implementation for solving the system of equations.
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
        let r = ids.iter()
            .zip(k.iter())
            .map(|(a, b)| a*b)
            .zip(offsets.iter())
            .map(|(a, b)| a - b)
            .collect();

        if step % 100 == 0 { println!("step {}", step); }
        step += 1;

        if all_equal(&r) { break; }

        // Increment k for the element with the lowest r value.
        let (min_i, _) = r.iter().enumerate()
            .min_by(|(_,a), (_,b)| a.cmp(b))
            .unwrap();
        k[min_i] += 1;
    }
    return k[0] * ids[0];
}

fn part2(text: &str) -> i128 {
    let v: Vec<_> = text.split(",")
        .enumerate()
        .filter(|(_, id)| *id != "x")
        .collect();
    dbg!(&v);
    let buses: Vec<Bus> = v.iter()
        .map(|(i, id)|
             Bus { interval: id.parse::<i128>().unwrap(),
                   launch: (*i as i128) * -1, })
        .collect();
    dbg!(&buses);
    let mut bus: Bus = buses[0].clone();
    for i in 1..buses.len() {
        bus.combine_with_other_bus(&buses[i]);
        dbg!(&bus);
    }
    return bus.launch;
}

fn main() {
    let content = advent::load_input();

    dbg!(part1);

    // brute force test of part2 solving
    let v: Vec<i128> = vec![3i128, 5, 7, 11];
    let o: Vec<i128> = vec![0i128, 1, 2, 3];
    dbg!(bf_converge_vector(v, o));

    // elegant faster solution over the same data:
    dbg!(part2(&"3,5,7,11"));
  
    // the real solution to the input data set:
    println!("the real answer");
    dbg!(part2(&lines[1]));
}

#[derive(Clone, Debug)]
struct Bus {
    interval: i128,
    launch: i128,
}

impl Bus {
    fn new(i: i128, n: i128) -> Bus {
        Bus {
            interval: i,
            launch: n,
        }
    }

    fn arrival_time(&self, k: i128) -> i128 {
        return k * self.interval + self.launch;
    }

    fn combine_with_other_bus(&mut self, other: &Bus) {
        let mut k0: i128 = 1;
        let mut k1: i128 = 1;
        loop {
            let a0 = self.arrival_time(k0);
            let a1 = other.arrival_time(k1);
            if a0 == a1 { break; };
            if a0 < a1 {
                // a0 < a1
                k0 += 1;
            } else {
                // a0 > a1
                let mut incr = (a0 - a1) / other.interval;
                if incr < 1 { incr = 1; }
                k1 += incr;
            };
        }
        dbg!(k0, k1, self.arrival_time(k0), other.arrival_time(k1));

        // update bus:
        self.launch = self.arrival_time(k0);
        self.interval = self.interval * other.interval;
    }
}  // impl Bus

// solve: k0 * id0 = k1 * id1 - offset
//      n*k0*id0 = m * k1*id1 - offset
//               = m * k1*id1 - offset
//               
fn pair_converge(id0: i128, id1: i128,
                 offset: i128) -> (i128, i128) {
    let mut k0: i128 = 1;
    let mut k1: i128 = 1;
    while (k0*id0 != k1*id1 - offset) {
        if k0*id0 < k1*id1 - offset {
            k0 += 1;
        } else {
            k1 += 1;
        }
    }
    return (k0, k1);
    // new bus looks likes:
    //   interval of id0 * id1
    //   offset 
    //
    // id0=3 id1=5
    //  k0=3  k1=2
    // original bus:
    //    interval of id0
    //    launch of 0
    // new bus... id(01) = id0*id1 = 15
    //    interval of id0*id1 = 15
    //    launch of 9
    //    x = 9 + n*15
    // for example:
    //    first intersect = 9, 10
    //    second: 24, 25
    //    third:  39, 40
}

// k1*i1 = k2*i2 - 1 = k3*i3 - 2 = k4*i4 - 3
//
// k2 = (k1*i1 + 1) / i2
// n * k2 = (n*k1*i1 + n) / i2
//

/*
fn converge_vector(ids: Vec<i128>, offsets: Vec<i128>) -> i128 {
    let mut k: Vec<i128> = ids.iter().map(|_| 1).collect();
    let mut incr = 1;
    for i in 0..len(ids)-1 {
        let j = i + 1;
        let k = i + 2;
        let (p1, p2) = converge_pairs(ids[i], ids[j], incr, 1);
        let (n1, n2) = converge_pairs(ids[j], ids[k], incr, 1);
        incr = 
    }
    return 0;
}
*/

    
/*
fn find_lowest_common(text: &str) -> u128 {
    // Part 2
    // bus 0 departs at time x
    // bus 1 departs at time x+1
    // bus n departs at time x+n
    //
    // timestamp = k1*id1 = k2*id2 - 1 = k3*id3 - 2 = ...
    // it turns out all bus ids are prime numbers.
    //
    // k1*id1 = k2*id2 - 1
    // this pattern repeats id1*id2 units later, ie...
    // (k1+n*id2)*id1 = (k2+n*id1)*id2 - 1  will be true for all integer n
    //
    // k1*id1 = k2*id2 - 1   *c
    //   k1*id1*c = k2*id2*c - c
    //
    let bus_ids: Vec<i64> = text.split(",")
        .map(|id| if *id == "x" { "1" } else { *id })
        .map(|id| id.parse::<i64>().unwrap())
        .collect();

}
*/
