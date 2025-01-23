use p1::*;
use std::fs::*;

fn main() {
    let input = read_to_string("src/input.txt").expect("Failed to read input.txt");
    let points: Vec<(Point, Point)> = parse_input(&input);

    const WIDTH: i64 = 101;
    const HEIGHT: i64 = 103;
    const DELTA_TIME: i64 = 100;

    let guard_positions = points
        .iter()
        .map(|(start, velocity)| find_robot_position(*start, *velocity, WIDTH, HEIGHT, DELTA_TIME))
        .collect::<Vec<_>>();

    let safety_factor = find_safety_factor(&guard_positions, WIDTH, HEIGHT);
    println!("Day 14 - Part 1.");
    println!("Safety factor: {}", safety_factor);
}
