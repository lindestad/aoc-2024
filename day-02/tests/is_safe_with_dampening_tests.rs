use day_02::is_safe_with_dampening;

#[test]
fn test_already_safe_report() {
    let report = vec![7, 6, 4, 2, 1];
    assert!(is_safe_with_dampening(&report)); // Already safe, no removal needed
}

#[test]
fn test_unsafe_fixable_by_removal() {
    let report = vec![1, 3, 2, 4, 5];
    assert!(is_safe_with_dampening(&report)); // Remove "3" to make it safe
}

#[test]
fn test_unsafe_fixable_by_removal_different_case() {
    let report = vec![8, 6, 4, 4, 1];
    assert!(is_safe_with_dampening(&report)); // Remove "4" (third element) to make it safe
}

#[test]
fn test_unfixable_report() {
    let report = vec![1, 2, 7, 8, 9];
    assert!(!is_safe_with_dampening(&report)); // No single removal can fix this report
}

#[test]
fn test_safe_with_one_problematic_level() {
    let report = vec![1, 3, 6, 7, 9];
    assert!(is_safe_with_dampening(&report)); // Already safe, even with minor problems
}

#[test]
fn test_empty_report() {
    let report: Vec<i32> = vec![];
    assert!(is_safe_with_dampening(&report)); // Trivially safe
}

#[test]
fn test_single_level_report() {
    let report = vec![42];
    assert!(is_safe_with_dampening(&report)); // Single level is trivially safe
}

#[test]
fn test_unsafe_two_level_report_fixable() {
    let report = vec![5, 15, 8];
    assert!(is_safe_with_dampening(&report)); // Remove "15" to make it safe
}

#[test]
fn test_complex_unfixable_case() {
    let report = vec![1, 10, 3, 15, 5];
    assert!(!is_safe_with_dampening(&report)); // Too many issues to fix by one removal
}
