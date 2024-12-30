use day_04::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");

    // Part 1
    println!("Day 04 - Part 1:");
    println!(
        "Number of XMAS found: {}",
        // map text input to <vec<vec<char>>> and count XMAS
        count_xmas(input.lines().map(|line| line.chars().collect()).collect())
    );

    // Part 2
    println!("Day 04 - Part 2:");
    println!(
        "Number of MAS crosses found: {}",
        // map text input to <vec<vec<char>>> and count MAS crosses
        count_mas_crosses(input.lines().map(|line| line.chars().collect()).collect())
    );
}
