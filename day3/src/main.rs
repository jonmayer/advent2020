use std::fs;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

type TreeMatrix = Vec<Vec<bool>>;

fn count_trees(data: &TreeMatrix, slope: &Coord) -> u64 {
    let width = data[0].len();
    let height = data.len();
    let mut p = Coord {x: 0, y: 0};
    let mut count: u64 = 0;
    while p.y < height {
        // dbg!(&p, &data[p.y][p.x]);
        if data[p.y][p.x] { count += 1; }
        p.y += slope.y;
        p.x = (p.x + slope.x) % width;
    }
    return count;
}

fn main() {
    // let inputfile = "sample.txt";
    let inputfile = "input.txt";

    let contents = fs::read_to_string(inputfile)
                 .expect("Something went wrong reading the file");
    let lines = contents.lines().map(|x| x.trim());
    let data: TreeMatrix = lines
        .map(|line| line.chars().map(|y| y == '#').collect())
        .collect();
    // dbg!(&data);

    println!("part 1");
    dbg!(count_trees(&data, &Coord {x: 3, y: 1}));

    println!("part 2");
    /*
    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.
    */

    let product = dbg!(count_trees(&data, &Coord {x: 1, y: 1}))
                * dbg!(count_trees(&data, &Coord {x: 3, y: 1}))
                * dbg!(count_trees(&data, &Coord {x: 5, y: 1}))
                * dbg!(count_trees(&data, &Coord {x: 7, y: 1}))
                * dbg!(count_trees(&data, &Coord {x: 1, y: 2}));
    dbg!(product);
}
