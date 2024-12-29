use day_02::is_safe;

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
