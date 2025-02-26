use p2::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt.");
    let map = expand_map(&get_map(&input));
    let moves = get_moves(&input);

    let solution_map = solve_maze(&map, moves);
    let gps_value = gps(solution_map);

    println!("Day 15, Part 2.");
    println!("The sum of the boxes GPS coordinates is {gps_value}.");
}
