use p2::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt.");
    let maze = parse_input(&input);
    let num_unique = find_num_unique_tiles(&maze);

    println!("Day 16, Part 2.");
    println!(
        "The number of unique tiles on which at least one optimal path lay on is {num_unique}."
    );
}
