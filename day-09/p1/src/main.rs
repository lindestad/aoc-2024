use p1::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt");
    let checksum = calculate_checksum(&input.unwrap());
    println!("Day 09: Part 1");
    println!("Checksum: {}", checksum);
}
