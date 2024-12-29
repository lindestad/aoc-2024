use std::collections::HashMap;
use std::fs;

fn main() {
    // Read the input file
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    // Parse the input file, splitting each line into a vector of numbers
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Failed to parse number"))
            .collect();

        // Push to the left and right lists
        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        } else {
            panic!("Invalid input line: {}", line);
        }
    }

    // Sort each list
    left_list.sort();
    right_list.sort();

    // Calculate the distance
    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("Day 01: Part one");
    println!("Total distance: {}", total_distance);

    // Part two
    // lists are already sorted, but this does not impact the solution
    let mut counts = HashMap::new();
    // Iterate over the right list and count the number of occurrences, increment key: value in
    // the counts HashMap, if the key does not exist, insert it with a value of 0
    for &num in right_list.iter() {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for &num in left_list.iter() {
        if let Some(&count) = counts.get(&num) {
            // counts.get(&num) returns Option<&i32>, Some(count) returns &i32
            similarity_score += num * count;
        }
    }

    println!("Day 01: Part two");
    println!("Similarity score: {}", similarity_score);
}
