use p1::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.");
    let warehouse_map: Vec<Vec<char>>;
    let moves: Vec<char>;
    (warehouse_map, moves) = parse_input(&input);

    let result_map = solve_map(warehouse_map, moves);
    let gps_value = gps(result_map);

    println!("Day 15, Part 1.");
    println!("Sum of all boxes' GPS coordinates: {gps_value}.");
}
