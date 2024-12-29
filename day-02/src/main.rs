use day_02::*;
use std::fs;

fn part_one() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");

    // Parse input into a vector of reports
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect()
        })
        .collect();

    let safe_reports = reports.iter().filter(|&report| is_safe(report)).count();
    println!("Number of safe reports: {}", safe_reports);
}

fn part_two() {
    let _input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");
}

fn main() {
    println!("Day 02 - part one:");
    part_one();
    println!();
}
