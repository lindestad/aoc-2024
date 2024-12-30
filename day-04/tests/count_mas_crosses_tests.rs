use day_04::count_mas_crosses;

#[test]
fn test_no_x_mas() {
    // 3×3 grid, but it has no `M`, `A`, and `S` in the needed diagonal arrangement
    let grid = vec![
        vec!['A', 'B', 'C'],
        vec!['B', 'B', 'B'],
        vec!['C', 'B', 'A'],
    ];
    assert_eq!(count_mas_crosses(grid), 0);
}
#[test]
fn test_single_x_mas() {
    let grid = vec![
        vec!['M', 'X', 'S'],
        vec!['X', 'A', 'X'],
        vec!['M', 'X', 'S'],
    ];
    // This should match:
    //  - Diagonal 1: M -> A -> S
    //  - Diagonal 2: M -> A -> S
    // forming exactly one X shape in the center.
    assert_eq!(count_mas_crosses(grid), 1);
}
#[test]
fn test_single_x_mas_reversed() {
    let grid = vec![
        vec!['S', 'Z', 'M'],
        vec!['Z', 'A', 'Z'],
        vec!['S', 'Z', 'M'],
    ];
    // Each diagonal spells SAM instead of MAS. But that's allowed.
    assert_eq!(count_mas_crosses(grid), 1);
}
#[test]
fn test_part_two_provided_example() {
    // 10x10 puzzle from Part Two puzzle text
    let grid = vec![
        // Row 0: ".M.S......"
        vec!['.', 'M', '.', 'S', '.', '.', '.', '.', '.', '.'],
        // Row 1: "..A..MSMS."
        vec!['.', '.', 'A', '.', '.', 'M', 'S', 'M', 'S', '.'],
        // Row 2: ".M.S.MAA.."
        vec!['.', 'M', '.', 'S', '.', 'M', 'A', 'A', '.', '.'],
        // Row 3: "..A.ASMSM."
        vec!['.', '.', 'A', '.', 'A', 'S', 'M', 'S', 'M', '.'],
        // Row 4: ".M.S.M...."
        vec!['.', 'M', '.', 'S', '.', 'M', '.', '.', '.', '.'],
        // Row 5: ".........."
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        // Row 6: "S.S.S.S.S."
        vec!['S', '.', 'S', '.', 'S', '.', 'S', '.', 'S', '.'],
        // Row 7: ".A.A.A.A.."
        vec!['.', 'A', '.', 'A', '.', 'A', '.', 'A', '.', '.'],
        // Row 8: "M.M.M.M.M."
        vec!['M', '.', 'M', '.', 'M', '.', 'M', '.', 'M', '.'],
        // Row 9: ".........."
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ];

    // The puzzle states it should yield 9 valid "X-MAS" shapes
    assert_eq!(count_mas_crosses(grid), 9);
}
