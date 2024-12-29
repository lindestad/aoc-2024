pub fn is_safe(report: &[i32]) -> bool {
    // Calculate absolute differences between each pair of levels
    let diffs: Vec<i32> = report.windows(2).map(|w| (w[1] - w[0]).abs()).collect();

    // Check if all differences are in bounds
    let all_diffs_in_bounds = diffs.iter().all(|&diff| diff >= 1 && diff <= 3);

    // Check if all levels are increasing or decreasing
    let increasing = report.windows(2).all(|w| w[1] > w[0]);
    let decreasing = report.windows(2).all(|w| w[1] < w[0]);

    return all_diffs_in_bounds && (increasing || decreasing);
}

pub fn is_safe_with_dampening(report: &[i32]) -> bool {
    // Dampening means that a single level causing fault can be removed, yielding safe report
    let result = (0..report.len()).any(|i| {
        let mut report = report.to_vec();
        report.remove(i);
        is_safe(&report)
    });
    return result;
}
