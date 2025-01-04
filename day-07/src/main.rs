use day_07::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::thread;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");
    let (keys, values_vec_vec) = parse_input(&input);

    let num_threads = num_cpus::get() - 1; // Use one less thread than the number of CPUs
    let chunk_size = (keys.len() + num_threads - 1) / num_threads; // Divide work into chunks
    let mut handles = vec![];

    // Progress bar
    let pb = ProgressBar::new(keys.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    for chunk in keys
        .chunks(chunk_size)
        .zip(values_vec_vec.chunks(chunk_size))
    {
        let chunk_keys = chunk.0.to_vec();
        let chunk_values = chunk.1.to_vec();

        // Spawn a thread for this chunk
        let pb_clone = pb.clone();
        let handle = thread::spawn(move || {
            let mut local_sum = 0;
            for (key, values_vec) in chunk_keys.iter().zip(chunk_values.iter()) {
                if can_make_target(*key, &values_vec) {
                    local_sum += key;
                }
                pb_clone.inc(1); // Update the progress bar
            }
            local_sum
        });

        handles.push(handle);
    }

    // Collect results from threads
    let mut sum: i128 = 0;
    for handle in handles {
        sum += handle.join().unwrap();
    }

    pb.finish_with_message("All keys processed!");
    println!("Day 07: Part 1");
    println!("Sum of keys that can be made from values: {}", sum);

    // Part 2
    let mut handles = vec![];

    // Progress bar
    let pb = ProgressBar::new(keys.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    for chunk in keys
        .chunks(chunk_size)
        .zip(values_vec_vec.chunks(chunk_size))
    {
        let chunk_keys = chunk.0.to_vec();
        let chunk_values = chunk.1.to_vec();

        // Spawn a thread for this chunk
        let pb_clone = pb.clone();
        let handle = thread::spawn(move || {
            let mut local_sum = 0;
            for (key, values_vec) in chunk_keys.iter().zip(chunk_values.iter()) {
                if can_make_target_2(*key, &values_vec) {
                    local_sum += key;
                }
                pb_clone.inc(1); // Update the progress bar
            }
            local_sum
        });

        handles.push(handle);
    }

    // Collect results from threads
    let mut sum: i128 = 0;
    for handle in handles {
        sum += handle.join().unwrap();
    }
    pb.finish_with_message("All keys processed!");
    println!("Day 07: Part 2");
    println!("Sum of keys that can be made from values: {}", sum);
}
