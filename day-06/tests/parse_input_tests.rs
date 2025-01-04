use day_06::parse_input;

#[test]
fn test_parse_input_test() {
    let input = "...#....\n..##.#.#\n#.......\n##.#..#.\n#...##..\n";
    let expected_output = vec![
        vec!['.', '.', '.', '#', '.', '.', '.', '.'],
        vec!['.', '.', '#', '#', '.', '#', '.', '#'],
        vec!['#', '.', '.', '.', '.', '.', '.', '.'],
        vec!['#', '#', '.', '#', '.', '.', '#', '.'],
        vec!['#', '.', '.', '.', '#', '#', '.', '.'],
    ];
    let output = parse_input(input);
    assert_eq![output, expected_output];
}
