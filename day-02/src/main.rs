use day_02::*;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");

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
    println!("Day 02 - part one:");
    println!("Number of safe reports: {}\n", safe_reports);

    let safe_reports_with_dampening = reports
        .iter()
        .filter(|&report| is_safe_with_dampening(report))
        .count();
    println!("Day 02 - part two:");
    println!(
        "Number of safe reports with dampening: {}",
        safe_reports_with_dampening
    );
}
