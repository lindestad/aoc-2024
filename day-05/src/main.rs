use day_05::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");
    let (pairs, values) = parse_input(&input);

    let mut valid_values: Vec<i32> = Vec::new();
    for value in &values {
        if check_validity(&pairs, value) {
            if let Some(middle) = return_middle_if_valid(&pairs, value) {
                valid_values.push(middle);
            }
        }
    }
    let sum = valid_values.iter().sum::<i32>();
    println!("Day 05: Part 1");
    println!("Sum of valid middle values: {}", sum);

    let mut invalid_values: Vec<Vec<i32>> = Vec::new();
    for line_of_values in &values {
        if !check_validity(&pairs, line_of_values) {
            invalid_values.push(line_of_values.clone());
        }
    }
    let mut middle_values: Vec<i32> = Vec::new();
    for line_of_values in &invalid_values {
        let mut sorted_values = line_of_values.clone();
        sort_values(&pairs, &mut sorted_values);
        if check_validity(&pairs, &sorted_values) {
            if let Some(middle) = return_middle_if_valid(&pairs, &sorted_values) {
                middle_values.push(middle);
            }
        }
    }
    let sum = middle_values.iter().sum::<i32>();
    println!("Day 05: Part 2");
    println!("Sum of valid middle values after sorting: {}", sum);
}
