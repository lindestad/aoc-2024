use day_05::parse_input;

#[test]
fn test_parse_input_basic() {
    let input = "12|30\n14|9\n25|6\n\n1,2,3\n4,5,6";
    let (pairs, values) = parse_input(input);
    assert_eq!(pairs, vec![(12, 30), (14, 9), (25, 6)]);
    assert_eq!(values, vec![vec![1, 2, 3], vec![4, 5, 6]]);
}

#[test]
fn test_parse_input_empty_input() {
    let input = "";
    let (pairs, values) = parse_input(input);
    assert!(pairs.is_empty());
    assert!(values.is_empty());
}

#[test]
fn test_parse_input_only_pairs() {
    let input = "12|30\n14|9\n25|6";
    let (pairs, values) = parse_input(input);
    assert_eq!(pairs, vec![(12, 30), (14, 9), (25, 6)]);
    assert!(values.is_empty());
}

#[test]
fn test_parse_input_only_values() {
    let input = "\n1,2,3\n4,5,6";
    let (pairs, values) = parse_input(input);
    assert!(pairs.is_empty());
    assert_eq!(values, vec![vec![1, 2, 3], vec![4, 5, 6]]);
}

#[test]
fn test_parse_input_malformed_pairs() {
    let input = "12|30\n14\n25|6\n\n1,2,3\n4,5,6";
    let (pairs, values) = parse_input(input);
    assert_eq!(pairs, vec![(12, 30), (25, 6)]); // Skips malformed pair "14"
    assert_eq!(values, vec![vec![1, 2, 3], vec![4, 5, 6]]);
}

#[test]
fn test_parse_input_malformed_values() {
    let input = "12|30\n14|9\n25|6\n\n1,2,x\n4,5,6";
    let (pairs, values) = parse_input(input);
    assert_eq!(pairs, vec![(12, 30), (14, 9), (25, 6)]);
    assert_eq!(values, vec![vec![1, 2], vec![4, 5, 6]]); // Skips non-integer "x"
}

#[test]
fn test_parse_input_blank_lines() {
    let input = "12|30\n14|9\n\n\n1,2,3\n\n4,5,6\n";
    let (pairs, values) = parse_input(input);
    assert_eq!(pairs, vec![(12, 30), (14, 9)]);
    assert_eq!(values, vec![vec![1, 2, 3], vec![4, 5, 6]]);
}
