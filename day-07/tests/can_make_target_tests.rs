use day_07::can_make_target;

#[test]
fn test_empty_values() {
    // No values at all => cannot match any target.
    assert_eq!(can_make_target(10, &[]), false);
}

#[test]
fn test_single_value_equal() {
    // Single element that matches the target exactly.
    assert_eq!(can_make_target(42, &[42]), true);
}

#[test]
fn test_single_value_not_equal() {
    // Single element that does not match the target.
    assert_eq!(can_make_target(100, &[42]), false);
}

#[test]
fn test_two_values_plus_match() {
    // 10 + 5 = 15
    let values = [10, 5];
    assert_eq!(can_make_target(15, &values), true);
    // 10 * 5 = 50 => also possible, but we just need at least one combo to match.
    // So target=15 is still valid because the + path yields 15.
}

#[test]
fn test_two_values_multiply_match() {
    // 10 * 5 = 50
    let values = [10, 5];
    assert_eq!(can_make_target(50, &values), true);
}

#[test]
fn test_two_values_no_match() {
    // 10 + 5 = 15, 10 * 5 = 50 => none are 20
    let values = [10, 5];
    assert_eq!(can_make_target(20, &values), false);
}

#[test]
fn test_three_values_simple() {
    // 2 + 3 + 4 = 9
    // 2 + 3 * 4 = 20 (evaluated left-to-right => (2+3)*4=5*4=20)
    // 2 * 3 + 4 = 10
    // 2 * 3 * 4 = 24
    // So possible results are {9, 20, 10, 24}
    // Checking each:
    let values = [2, 3, 4];
    assert_eq!(can_make_target(9, &values), true);
    assert_eq!(can_make_target(20, &values), true);
    assert_eq!(can_make_target(10, &values), true);
    assert_eq!(can_make_target(24, &values), true);

    // Something not in that set:
    assert_eq!(can_make_target(12, &values), false);
}

#[test]
fn test_larger_example() {
    // Example from puzzle statement: 81, 40, 27 => two positions for +/*, left-to-right
    // Possible expressions:
    // (81+40)+27  = 121+27=148
    // (81+40)*27  = 121*27=3267
    // (81*40)+27  = 3240+27=3267
    // (81*40)*27  = 3240*27=87480
    // So 3267 is definitely reachable (twice, in fact).
    let values = [81, 40, 27];
    assert_eq!(can_make_target(3267, &values), true);
    // Some value not in {148, 3267, 87480}:
    assert_eq!(can_make_target(9999, &values), false);
}
