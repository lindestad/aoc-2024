use day_03::*;
use std::fs;

fn main() {
    // Part 1
    println!("Day 03- part 1:");
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");
    println!("Result: {}", find_valid_mul_no_regex(&input));

    // Part 2
    println!("Day 03- part 2:");
    let input_without_dont_sections = strip_dont_sections(&input);
    println!(
        "Result: {}",
        find_valid_mul_no_regex(&input_without_dont_sections)
    );
}
