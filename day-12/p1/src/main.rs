use p1::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input file");
    let price = calculate_price(&input);
    println!("Day 12: Part 1");
    println!("The total price is: {}", price);
}
