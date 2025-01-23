use p2::*;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::thread;
use std::time::Duration;

const BATCH_SIZE: i64 = 24; // Number of counters processed in parallel
const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;
const DELTA_TIME: i64 = 100;

fn main() {
    let input = read_to_string("src/input.txt").expect("Failed to read input.txt");
    let points: Vec<(Point, Point)> = parse_input(&input);

    println!("Day 14 - Part 2.");
    let mut counter = 1;
    let mut seen_positions: HashSet<Vec<Point>> = HashSet::new(); // Stores unique guard positions

    let mut max_collinear = 0;
    let mut max_counter = 0;

    loop {
        // Run `BATCH_SIZE` counters in parallel
        let results: Vec<(i64, i64, Vec<Point>)> = (counter..counter + BATCH_SIZE)
            .into_par_iter() // Parallel execution over multiple counters
            .map(|c| {
                let guard_positions: Vec<Point> = points
                    .par_iter()
                    .map(|(start, velocity)| {
                        find_robot_position(*start, *velocity, WIDTH, HEIGHT, c)
                    })
                    .collect();

                let collinear_points = max_num_points_collinear(&guard_positions);
                (c, collinear_points, guard_positions)
            })
            .collect();

        // Sort results so they print in order
        let mut sorted_results = results;
        sorted_results.sort_by_key(|(c, _, _)| *c);

        for (c, collinear_points, guard_positions) in sorted_results {
            // Loop detection
            if seen_positions.contains(&guard_positions) {
                println!("\nLoop detected! Counter: {}", c);
                println!("Guards have returned to a previous state.");
                return; // Exit the loop once a cycle is detected
            } else {
                seen_positions.insert(guard_positions.clone());
            }
            if collinear_points >= 35 {
                println!("\nFound! Counter: {}", c);
                print_robots(&guard_positions, WIDTH, HEIGHT);
                println!("Number of collinear points: {}", collinear_points);
                thread::sleep(Duration::from_millis(500)); // Pause for visualization
            } else {
                if collinear_points > max_collinear {
                    max_collinear = collinear_points;
                    max_counter = c;
                }
                println!(
                    "Counter: {} | Collinear: {} | Max Col: {} | at {}",
                    c, collinear_points, max_collinear, max_counter
                );
            }
        }

        counter += BATCH_SIZE; // Move to next batch
    }
}
