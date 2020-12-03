use std::fs;


fn main() {
    let inputfile = "input.txt";

    let contents = fs::read_to_string(inputfile)
                 .expect("Something went wrong reading the file");

    let nums: Vec<i64> = contents.split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    println!("Parsed input data: {:?}", nums);

    println!("Day 1 part 1");
    for i in 0..nums.len() {
        for j in (i+1)..nums.len() {
            if nums[i] + nums[j] == 2020 {
                println!("{} {} {}", nums[i], nums[j], nums[i] * nums[j]);
            }
        }
    }

    println!("Day 1 part 2");
    for i in 0..nums.len() {
        for j in (i+1)..nums.len() {
            for k in (j+1)..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    println!("{}:{} {}:{} {}:{} = {}",
                        i, nums[i],
                        j, nums[j],
                        k, nums[k],
                        nums[i] * nums[j] * nums[k]);
                }
            }
        }
    }
}

