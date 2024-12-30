use day_05::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");
    let (pairs, values) = parse_input(&input);
}
