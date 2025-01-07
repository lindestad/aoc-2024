use p2::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading input.txt");
    let grid = parse_input_to_grid(&input);
    let rating = sum_trail_ratings(&grid);
    println!("Day 10, Part 2");
    println!("Total rating: {}", rating);
}
