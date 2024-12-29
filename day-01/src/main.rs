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

    println!("Total distance: {}", total_distance);
}
