use p1::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading input.txt");
    let grid = parse_input_to_grid(&input);
    let score = sum_trail_scores(&grid);
    println!("Day 10, Part 1");
    println!("Total score: {}", score);
}
