use std::fs;

pub fn load_input() -> String {
    let inputfile: String = match std::env::args().nth(1) {
        None => "input.txt".to_string(),
        Some(x) => x,
    };

    let contents = fs::read_to_string(&inputfile).expect("Something went wrong reading the file");
    println!("{}: {} bytes", inputfile, contents.len());
    return contents;
}
