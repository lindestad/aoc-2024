use day_05::sort_values;

#[test]
fn test_empty_values() {
    let pairs = vec![(1, 2), (3, 4)];
    let mut values: Vec<i32> = vec![];
    sort_values(&pairs, &mut values);
    assert_eq!(values, vec![]); // Empty input should remain empty
}

#[test]
fn test_single_value() {
    let pairs = vec![(1, 2), (3, 4)];
    let mut values = vec![1];
    sort_values(&pairs, &mut values);
    assert_eq!(values, vec![1]); // Single element is always sorted
}

#[test]
fn test_already_sorted() {
    let pairs = vec![(1, 2), (2, 3)];
    let mut values = vec![1, 2, 3];
    sort_values(&pairs, &mut values);
    assert_eq!(values, vec![1, 2, 3]); // Already sorted input remains unchanged
}

#[test]
fn test_unsorted_values() {
    let pairs = vec![(1, 2), (2, 3)];
    let mut values = vec![3, 1, 2];
    sort_values(&pairs, &mut values);
    assert_eq!(values, vec![1, 2, 3]); // Should reorder to satisfy rules
}

#[test]
fn test_missing_dependencies() {
    let pairs = vec![(1, 2), (2, 3)];
    let mut values = vec![3, 2];
    sort_values(&pairs, &mut values);
    assert_eq!(values, vec![2, 3]); // Rules involving missing elements are ignored
}

#[test]
fn test_multiple_rules() {
    let pairs = vec![(3, 1), (1, 2), (2, 4)];
    let mut values = vec![4, 3, 1, 2];
    sort_values(&pairs, &mut values);
    assert_eq!(values, vec![3, 1, 2, 4]); // Should reorder according to all rules
}

#[test]
fn test_no_pairs() {
    let pairs: Vec<(i32, i32)> = vec![];
    let mut values = vec![3, 1, 2];
    sort_values(&pairs, &mut values);
    assert_eq!(values, vec![3, 1, 2]); // No rules mean no changes
}
