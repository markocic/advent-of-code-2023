use std::fs;

fn main() {

    let contents = fs::read_to_string("src/input.txt").expect("Could not open file.");

    let result = process(contents);
    println!("{}", result);
}

fn process(contents: String) -> usize {
    0
}
