use p1::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt.");
    let cost = solve_maze(&parse_input(&input)).expect("No solution found.");

    println!("Day 16, Part 1.");
    println!("The best path has a cost of {cost}.");
}
