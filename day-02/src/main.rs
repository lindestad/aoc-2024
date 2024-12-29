use std::fs;

fn is_safe(report: &[i32]) -> bool {
    // Calculate absolute differences between each pair of levels
    let diffs: Vec<i32> = report.windows(2).map(|w| (w[1] - w[0]).abs()).collect();

    // Check if all differences are in bounds
    let all_diffs_in_bounds = diffs.iter().all(|&diff| diff >= 1 && diff <= 3);

    // Check if all levels are increasing or decreasing
    let increasing = report.windows(2).all(|w| w[1] > w[0]);
    let decreasing = report.windows(2).all(|w| w[1] < w[0]);

    return all_diffs_in_bounds && (increasing || decreasing);
}

fn main() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_increasing() {
        let report = vec![1, 3, 4, 6];
        assert!(is_safe(&report));
    }

    #[test]
    fn test_safe_decreasing() {
        let report = vec![7, 5, 3, 2];
        assert!(is_safe(&report));
    }

    #[test]
    fn test_unsafe_large_difference() {
        let report = vec![1, 3, 9];
        assert!(!is_safe(&report)); // Difference of 6 is too large
    }

    #[test]
    fn test_unsafe_mixed_directions() {
        let report = vec![1, 3, 2, 4];
        assert!(!is_safe(&report)); // Changes direction
    }

    #[test]
    fn test_empty_report() {
        let report: Vec<i32> = vec![];
        assert!(is_safe(&report)); // Edge case: empty report is trivially safe
    }

    #[test]
    fn test_single_level_report() {
        let report = vec![42];
        assert!(is_safe(&report)); // Edge case: single number is trivially safe
    }
}
