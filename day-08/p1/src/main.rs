use p1::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");
    let grid = input_to_vec(&input);
    let unique_antennas = find_unique_antinodes(&grid);
    let sum = unique_antennas.len();
    println!("Day 08, Part 1");
    println!("Unique antennas: {}", sum);
}
