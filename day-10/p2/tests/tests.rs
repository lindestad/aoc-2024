use p2::*;

#[test]
fn test_input_parse() {
    let input = "A.B\nC.D\n\r";
    let expected = vec![vec![b'A', b'.', b'B'], vec![b'C', b'.', b'D']];
    assert_eq!(parse_input_to_grid(input), expected);
}

#[test]
fn test_recursive_trail_search() {
    let input = vec![
        vec![b'.', b'.', b'.', b'0', b'.', b'.', b'.'],
        vec![b'.', b'.', b'.', b'1', b'.', b'.', b'.'],
        vec![b'.', b'.', b'.', b'2', b'.', b'.', b'.'],
        vec![b'6', b'5', b'4', b'3', b'4', b'5', b'6'],
        vec![b'7', b'.', b'.', b'.', b'.', b'.', b'7'],
        vec![b'8', b'.', b'.', b'.', b'.', b'.', b'8'],
        vec![b'9', b'.', b'.', b'.', b'.', b'.', b'9'],
    ];
    assert_eq!(recursive_trail_search(&input, (3, 0)), 2);
    assert_eq!(recursive_trail_search(&input, (4, 3)), 1);
    assert_eq!(recursive_trail_search(&input, (1, 1)), 0);
    assert_eq!(recursive_trail_search(&input, (0, 0)), 0);
    assert_eq!(recursive_trail_search(&input, (6, 5)), 1);
}

#[test]
fn test_find_trailheads() {
    let input = vec![
        vec![b'.', b'.', b'.', b'0', b'.', b'.', b'.'],
        vec![b'.', b'.', b'.', b'1', b'.', b'.', b'.'],
        vec![b'.', b'.', b'.', b'2', b'.', b'.', b'.'],
        vec![b'6', b'5', b'4', b'3', b'4', b'5', b'6'],
        vec![b'7', b'.', b'.', b'.', b'.', b'.', b'7'],
        vec![b'8', b'.', b'.', b'.', b'.', b'.', b'8'],
        vec![b'9', b'.', b'.', b'.', b'.', b'.', b'9'],
    ];
    assert_eq!(find_trailheads(&input), vec![(3, 0)]);
    let input2 = vec![
        vec![b'.', b'.', b'.', b'0', b'.', b'0', b'.'],
        vec![b'.', b'.', b'.', b'1', b'.', b'.', b'.'],
        vec![b'.', b'.', b'.', b'2', b'.', b'.', b'.'],
        vec![b'6', b'5', b'4', b'3', b'4', b'5', b'6'],
        vec![b'7', b'.', b'.', b'.', b'.', b'.', b'7'],
        vec![b'8', b'.', b'.', b'0', b'.', b'.', b'8'],
        vec![b'9', b'.', b'.', b'.', b'.', b'.', b'9'],
    ];
    assert_eq!(find_trailheads(&input2), vec![(3, 0), (5, 0), (3, 5)]);
}

#[test]
fn test_sum_trail_ratings() {
    let input = vec![
        vec![b'.', b'.', b'.', b'0', b'.', b'.', b'.'],
        vec![b'.', b'.', b'.', b'1', b'.', b'.', b'.'],
        vec![b'.', b'.', b'.', b'2', b'.', b'.', b'.'],
        vec![b'6', b'5', b'4', b'3', b'4', b'5', b'6'],
        vec![b'7', b'.', b'.', b'.', b'.', b'.', b'7'],
        vec![b'8', b'.', b'.', b'.', b'.', b'.', b'8'],
        vec![b'9', b'.', b'.', b'.', b'.', b'.', b'9'],
    ];
    assert_eq!(sum_trail_ratings(&input), 2);
    let input2 = vec![
        vec![b'.', b'.', b'.', b'0', b'.', b'0', b'.'],
        vec![b'.', b'.', b'.', b'1', b'.', b'.', b'.'],
        vec![b'.', b'.', b'.', b'2', b'.', b'.', b'.'],
        vec![b'6', b'5', b'4', b'3', b'4', b'5', b'6'],
        vec![b'7', b'.', b'.', b'.', b'.', b'.', b'7'],
        vec![b'8', b'.', b'.', b'0', b'.', b'.', b'8'],
        vec![b'9', b'.', b'.', b'.', b'.', b'.', b'9'],
    ];
    assert_eq!(sum_trail_ratings(&input2), 2);
    let input3 = vec![
        vec![b'.', b'.', b'.', b'0', b'.', b'0', b'.'],
        vec![b'.', b'.', b'0', b'1', b'.', b'.', b'.'],
        vec![b'.', b'.', b'.', b'2', b'.', b'.', b'.'],
        vec![b'6', b'5', b'4', b'3', b'4', b'5', b'6'],
        vec![b'7', b'.', b'.', b'.', b'.', b'.', b'7'],
        vec![b'8', b'.', b'.', b'0', b'.', b'.', b'8'],
        vec![b'9', b'.', b'.', b'.', b'.', b'.', b'9'],
    ];
    assert_eq!(sum_trail_ratings(&input3), 4);
}
