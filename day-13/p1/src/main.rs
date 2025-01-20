use p1::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading file");
    let problems = parse_input(&input);
    let solutions: Vec<usize> = problems
        .iter()
        .map(|(b1, b2, p)| solve(b1, b2, p))
        .collect();
    let sum: usize = solutions.iter().sum();

    println!("Day 13 - Part 1");
    println!("Minimun number of button presses: {}", sum);
}
