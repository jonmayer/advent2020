use advent;

fn main() {
    println!("Hello, world!");
    let content = advent::load_input();
    dbg!(day20::part1(&content));
    dbg!(day20::part2(&content));
}
