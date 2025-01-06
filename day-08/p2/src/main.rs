use p2::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");
    let grid = input_to_vec(&input);
    let antinodes = find_unique_antinodes(&grid);
    let sum = antinodes.len();
    println!("Day 08, Part 2");
    println!("Unique antinodes: {}", sum);
}
