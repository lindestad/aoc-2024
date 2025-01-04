use day_06::*;

#[test]
fn test_move_forward_no_obstacle() {
    let grid = vec![
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
    ];
    assert_eq!(next_guard_position(&grid, (1, 1), '^'), (1, 0, '^'));
    assert_eq!(next_guard_position(&grid, (1, 1), 'v'), (1, 2, 'v'));
    assert_eq!(next_guard_position(&grid, (1, 1), '<'), (0, 1, '<'));
    assert_eq!(next_guard_position(&grid, (1, 1), '>'), (2, 1, '>'));
}

#[test]
fn test_turn_on_obstacle() {
    let grid = vec![
        vec!['.', '.', '.'],
        vec!['.', '#', '.'],
        vec!['.', '.', '.'],
    ];
    assert_eq!(next_guard_position(&grid, (1, 2), '^'), (2, 2, '>')); // Turns right to '>'
    assert_eq!(next_guard_position(&grid, (1, 0), 'v'), (0, 0, '<')); // Turns left to '<'
}

#[test]
fn test_exit_on_out_of_bounds() {
    let grid = vec![
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
    ];
    assert_eq!(next_guard_position(&grid, (0, 0), '<'), (0, 0, 'Q')); // Out of bounds
    assert_eq!(next_guard_position(&grid, (2, 0), '>'), (0, 0, 'Q')); // Out of bounds
    assert_eq!(next_guard_position(&grid, (1, 0), '^'), (0, 0, 'Q')); // Out of bounds
    assert_eq!(next_guard_position(&grid, (2, 2), 'v'), (0, 0, 'Q')); // Out of bounds
}

#[test]
#[should_panic(expected = "Rotated four times without finding a valid direction")]
fn test_panic_if_no_valid_move() {
    let grid = vec![
        vec!['#', '#', '#'],
        vec!['#', '#', '#'],
        vec!['#', '#', '#'],
    ];
    next_guard_position(&grid, (1, 1), '^');
}
