use p1::*;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");
    let characters: HashSet<char> = find_characters(&input);
}
