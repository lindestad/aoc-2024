use p1::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.");
    let warehouse_map: Vec<Vec<char>>;
    let moves: Vec<char>;
    (warehouse_map, moves) = parse_input(&input);

    print_map(warehouse_map);
}
