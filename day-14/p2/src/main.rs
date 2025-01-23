use indicatif::{ProgressBar, ProgressStyle};
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
const MAX_COUNTER: i64 = 10404; // Estimate of max loops until all permutations are exhausted

fn main() {
    let input = read_to_string("src/input.txt").expect("Failed to read input.txt");
    let points: Vec<(Point, Point)> = parse_input(&input);

    println!("Day 14 - Part 2.");

    let mut counter = 1;
    let mut seen_positions: HashSet<Vec<Point>> = HashSet::new(); // Stores unique guard positions

    // Initialize progress bar
    let pb = ProgressBar::new(MAX_COUNTER as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    let mut max_collinear = 0;
    let mut max_counter = 0;

    let mut max_col_or_row = 0;
    let mut max_counter_2 = 0;

    'main_loop: loop {
        // Run `BATCH_SIZE` counters in parallel
        let results: Vec<(i64, i64, Vec<Point>, i64)> = (counter..counter + BATCH_SIZE)
            .into_par_iter() // Parallel execution over multiple counters
            .map(|c| {
                let guard_positions: Vec<Point> = points
                    .par_iter()
                    .map(|(start, velocity)| {
                        find_robot_position(*start, *velocity, WIDTH, HEIGHT, c)
                    })
                    .collect();

                let collinear_points = max_num_points_collinear(&guard_positions);
                let col_or_row = max_consecutive_col_row(&guard_positions, WIDTH, HEIGHT);
                (c, collinear_points, guard_positions, col_or_row)
            })
            .collect();

        // Sort results so they print in order
        let mut sorted_results = results;
        sorted_results.sort_by_key(|(c, _, _, _)| *c);

        for (c, collinear_points, guard_positions, col_or_row) in sorted_results {
            // Loop detection
            if seen_positions.contains(&guard_positions) {
                pb.finish_with_message("Completed.");
                println!("\nLoop detected after {} iterations. Exiting...", c);
                break 'main_loop; // Exit the loop once a cycle is detected
            } else {
                seen_positions.insert(guard_positions.clone());
            }
            // if collinear_points >= 35 {
            //     println!("\nFound! Counter: {}", c);
            //     print_robots(&guard_positions, WIDTH, HEIGHT);
            //     println!("Number of collinear points: {}", collinear_points);
            //     thread::sleep(Duration::from_millis(500)); // Pause for visualization
            if col_or_row >= 25 {
                print_robots(&guard_positions, WIDTH, HEIGHT);
                println!("\nFound a likely solution! Counter: {}\n", c);
                // println!("Number of guards in a row: {}", col_or_row);
                thread::sleep(Duration::from_millis(500)); // Pause for visualization
            }

            if collinear_points > max_collinear {
                max_collinear = collinear_points;
                max_counter = c;
            }

            if col_or_row > max_col_or_row {
                max_col_or_row = col_or_row;
                max_counter_2 = c;
            }

            // println!(
            //     "Counter: {} | Collinear: {} | Max Col: {} | at {} | Max Row: {} | at {}",
            //     c, collinear_points, max_collinear, max_counter, max_col_or_row, max_counter_2
            // );
        }

        counter += BATCH_SIZE; // Move to next batch
        pb.inc(BATCH_SIZE as u64); // Update progress bar
    }
}
