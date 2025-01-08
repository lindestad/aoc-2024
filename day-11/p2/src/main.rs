use p2::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading input.txt");
    let target_blinks = 75;
    let stones: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let total_stones = calculatestones(&stones, target_blinks);
    println!("Day 11 Part 2");
    println!(
        "After {} blinks, there are {} stones",
        target_blinks, total_stones
    );
}
