use day_07::*;
use indicatif::{ProgressBar, ProgressStyle};
use num_rational::Ratio;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");
    let (keys, values_vec_vec) = parse_input(&input);
    let mut sum: i128 = 0;
    let total_keys = keys.len();
    let pb = ProgressBar::new(total_keys as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    for (key, values_vec) in keys.iter().zip(values_vec_vec.iter()) {
        let target = Ratio::new(*key, 1);
        let values: Vec<Ratio<i128>> = values_vec.iter().map(|&x| Ratio::new(x, 1)).collect();
        if can_make_sum(target, values.clone()) {
            sum += key;
        }
        pb.inc(1); // Increment the progress bar
    }
    pb.finish_with_message("All keys processed!");
    println!("Sum of keys that can be made from values: {}", sum);
}
