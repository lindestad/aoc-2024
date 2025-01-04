use day_07::can_make_sum;
use num_rational::Ratio;

#[test]
fn test_empty_values() {
    let target = Ratio::new(1, 1);
    let values = vec![];
    assert!(!can_make_sum(target, values));
}

#[test]
fn test_single_value_equal_target() {
    let target = Ratio::new(5, 1);
    let values = vec![Ratio::new(5, 1)];
    assert!(can_make_sum(target, values));
}

#[test]
fn test_single_value_not_equal_target() {
    let target = Ratio::new(5, 1);
    let values = vec![Ratio::new(3, 1)];
    assert!(!can_make_sum(target, values));
}

#[test]
fn test_multiple_values_simple_addition() {
    let target = Ratio::new(7, 1);
    let values = vec![Ratio::new(3, 1), Ratio::new(4, 1)];
    assert!(can_make_sum(target, values));
}

#[test]
fn test_multiple_values_simple_multiplication() {
    let target = Ratio::new(12, 1);
    let values = vec![Ratio::new(3, 1), Ratio::new(4, 1)];
    assert!(can_make_sum(target, values));
}

#[test]
fn test_multiple_values_fractional_result() {
    let target = Ratio::new(1, 1);
    let values = vec![Ratio::new(1, 7), Ratio::new(7, 1)];
    assert!(can_make_sum(target, values));
}

#[test]
fn test_multiple_values_combination() {
    let target = Ratio::new(10, 1);
    let values = vec![Ratio::new(2, 1), Ratio::new(3, 1), Ratio::new(5, 1)];
    assert!(can_make_sum(target, values));
}

#[test]
fn test_multiple_values_no_solution() {
    let target = Ratio::new(100, 1);
    let values = vec![Ratio::new(3, 1), Ratio::new(4, 1), Ratio::new(5, 1)];
    assert!(!can_make_sum(target, values));
}

#[test]
fn test_large_numbers() {
    let target = Ratio::new(1_000_000, 1);
    let values = vec![Ratio::new(500_000, 1), Ratio::new(2, 1), Ratio::new(1, 1)];
    assert!(can_make_sum(target, values));
}

#[test]
fn test_large_numbers_no_solution() {
    let target = Ratio::new(1_000_001, 1);
    let values = vec![Ratio::new(500_000, 1), Ratio::new(2, 1), Ratio::new(1, 1)];
    assert!(!can_make_sum(target, values));
}
