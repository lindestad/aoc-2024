use day_04::count_xmas;

#[test]
fn test_count_xmas() {
    let grid = vec![
        // Row 0
        vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
        // Row 1
        vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
        // Row 2
        vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
        // Row 3
        vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
        // Row 4
        vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
        // Row 5
        vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
        // Row 6
        vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
        // Row 7
        vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
        // Row 8
        vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
        // Row 9
        vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
    ];

    assert_eq!(count_xmas(grid), 18);
}

#[test]
fn test_horizontal_xmas() {
    let grid = vec![
        vec!['X', 'M', 'A', 'S', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.'],
    ];
    assert_eq!(count_xmas(grid), 1, "Horizontal match failed");
}
#[test]
fn test_horizontal_reversed_xmas() {
    let grid = vec![
        vec!['S', 'A', 'M', 'X', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.'],
    ];
    assert_eq!(count_xmas(grid), 1, "Horizontal reversed match failed");
}
#[test]
fn test_vertical_xmas() {
    let grid = vec![
        vec!['X', '.'],
        vec!['M', '.'],
        vec!['A', '.'],
        vec!['S', '.'],
    ];
    assert_eq!(count_xmas(grid), 1, "Vertical match failed");
}
#[test]
fn test_vertical_reversed_xmas() {
    let grid = vec![
        vec!['S', '.'],
        vec!['A', '.'],
        vec!['M', '.'],
        vec!['X', '.'],
    ];
    assert_eq!(count_xmas(grid), 1, "Vertical reversed match failed");
}
#[test]
fn test_diagonal_xmas() {
    let grid = vec![
        vec!['X', '.', '.', '.'],
        vec!['.', 'M', '.', '.'],
        vec!['.', '.', 'A', '.'],
        vec!['.', '.', '.', 'S'],
    ];
    assert_eq!(count_xmas(grid), 1, "Diagonal match failed");
}
#[test]
fn test_diagonal_reversed_xmas() {
    let grid = vec![
        vec!['.', '.', '.', 'S'],
        vec!['.', '.', 'A', '.'],
        vec!['.', 'M', '.', '.'],
        vec!['X', '.', '.', '.'],
    ];
    assert_eq!(count_xmas(grid), 1, "Diagonal reversed match failed");
}
#[test]
fn test_diagonal_top_right_xmas() {
    let grid = vec![
        vec!['.', '.', '.', 'X'],
        vec!['.', '.', 'M', '.'],
        vec!['.', 'A', '.', '.'],
        vec!['S', '.', '.', '.'],
    ];
    assert_eq!(
        count_xmas(grid),
        1,
        "Diagonal top-right to bottom-left match failed"
    );
}
#[test]
fn test_diagonal_bottom_left_xmas() {
    let grid = vec![
        vec!['S', '.', '.', '.'],
        vec!['.', 'A', '.', '.'],
        vec!['.', '.', 'M', '.'],
        vec!['.', '.', '.', 'X'],
    ];
    assert_eq!(
        count_xmas(grid),
        1,
        "Diagonal bottom-left to top-right match failed"
    );
}
#[test]
fn test_multiple_xmas() {
    let grid = vec![
        vec!['X', 'M', 'A', 'S', '.', '.'],
        vec!['.', '.', 'S', '.', '.', 'S'],
        vec!['.', 'X', 'M', 'A', 'S', 'A'],
        vec!['.', '.', '.', '.', 'M', 'M'],
        vec!['X', 'M', 'A', 'S', '.', 'X'],
    ];
    assert_eq!(count_xmas(grid), 5, "Multiple matches failed");
}
