use p1::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading input.txt");
    let target_blinks = 25;
    let stones = input.replace(&['\n', '\r'][..], "");
    let result = blink(&stones, target_blinks);
    println!("Day 11 Part 1");
    println!(
        "After {} blinks, there are {} stones",
        target_blinks,
        count_stones(&result)
    );
}
