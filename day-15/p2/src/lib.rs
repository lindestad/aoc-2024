pub fn get_moves(input: &str) -> Vec<char> {
    let mut moves = Vec::new();
    for c in input.chars() {
        match c {
            '<' | '^' | '>' | 'v' => moves.push(c),
            _ => (),
        }
    }
    moves
}

pub fn get_map(input: &str) -> Vec<Vec<char>> {
    let mut m: Vec<Vec<char>> = Vec::new();

    let mut started_parsing: bool = false;

    for line in input.lines() {
        if !started_parsing {
            if line.starts_with('#') {
                started_parsing = true;
            } else {
                continue; // Possible empty lines before the map
            }
        }
        if !line.starts_with('#') {
            break;
        }
        m.push(line.chars().collect());
    }
    m
}

pub fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    (expand_map(&get_map(input)), get_moves(input))
}

pub fn print_map(map: &[Vec<char>]) {
    println!(" ");
    for line in map.iter() {
        println!("{}", line.iter().collect::<String>());
    }
    println!(" ");
}

pub fn expand_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut out_map: Vec<Vec<char>> = Vec::new();
    for row in map.iter() {
        let mut new_row: Vec<char> = Vec::new();
        for c in row.iter() {
            match c {
                '#' => new_row.extend_from_slice(&['#', '#']),
                'O' => new_row.extend_from_slice(&['[', ']']),
                '.' => new_row.extend_from_slice(&['.', '.']),
                '@' => new_row.extend_from_slice(&['@', '.']),
                _ => panic!("Invalid character in input map: '{c}' was passed."),
            }
        }
        out_map.push(new_row);
    }
    out_map
}

// Wrapper to keep track of recursion depth internally
pub fn blocked(map: &Vec<Vec<char>>, pos: (usize, usize), direction: char) -> bool {
    blocked_internal(map, pos, direction, 0)
}

fn blocked_internal(
    map: &Vec<Vec<char>>,
    pos: (usize, usize),
    direction: char,
    depth: usize,
) -> bool {
    match direction {
        '<' => match map[pos.1][pos.0 - 1] {
            '#' => true,
            '.' => false,
            ']' => {
                assert!(map[pos.1][pos.0 - 2] == '[');
                blocked_internal(map, (pos.0 - 2, pos.1), direction, depth + 1) // Check neighbor
            }
            '[' => {
                panic!(
                    "Bugged box in map: {}{}",
                    map[pos.1][pos.0 - 1],
                    map[pos.1][pos.0]
                );
            }

            _ => panic!("Invalid character found: {}", map[pos.1][pos.0 - 1]),
        },
        '^' => {
            match map[pos.1][pos.0] {
                '[' => {
                    assert!(map[pos.1][pos.0 + 1] == ']');
                    // Check space above box
                    blocked_internal(map, (pos.0, pos.1 - 1), direction, depth + 1)
                        || blocked_internal(map, (pos.0 + 1, pos.1 - 1), direction, depth + 1)
                }
                ']' => {
                    assert!(map[pos.1][pos.0 - 1] == '[');
                    // Check space above box
                    blocked_internal(map, (pos.0, pos.1 - 1), direction, depth + 1)
                        || blocked_internal(map, (pos.0 - 1, pos.1 - 1), direction, depth + 1)
                }
                '.' | '@' => {
                    if depth == 0 {
                        blocked_internal(map, (pos.0, pos.1 - 1), direction, depth + 1)
                    } else {
                        false
                    }
                }
                '#' => {
                    if depth == 0 {
                        panic!("Trying to check from boundary and up, is this intentional? Passed position {:?}, direction '{}'.", pos, direction);
                    } else {
                        true
                    }
                }
                _ => panic!("Invalid character found in map at position {:?}.", pos),
            }
        }
        '>' => {
            match map[pos.1][pos.0 + 1] {
                '#' => true,
                '.' => false,
                '[' => {
                    assert!(map[pos.1][pos.0 + 2] == ']');
                    blocked_internal(map, (pos.0 + 2, pos.1), direction, depth + 1)
                    // Check neighbor
                }
                ']' => {
                    panic!(
                        "Bugged box in map: {}{}",
                        map[pos.1][pos.0 + 1],
                        map[pos.1][pos.0]
                    );
                }

                _ => panic!("Invalid character found: {}", map[pos.1][pos.0 + 1]),
            }
        }
        'v' => {
            match map[pos.1][pos.0] {
                '[' => {
                    assert!(map[pos.1][pos.0 + 1] == ']');
                    // Check space below box
                    blocked_internal(map, (pos.0, pos.1 + 1), direction, depth + 1)
                        || blocked_internal(map, (pos.0 + 1, pos.1 + 1), direction, depth + 1)
                }
                ']' => {
                    assert!(map[pos.1][pos.0 - 1] == '[');
                    // Check space below box
                    blocked_internal(map, (pos.0, pos.1 + 1), direction, depth + 1)
                        || blocked_internal(map, (pos.0 - 1, pos.1 + 1), direction, depth + 1)
                }
                '.' | '@' => {
                    if depth == 0 {
                        blocked_internal(map, (pos.0, pos.1 + 1), direction, depth + 1)
                    } else {
                        false
                    }
                }
                '#' => {
                    if depth == 0 {
                        panic!("Trying to check from boundary and down, is this intentional? Passed position {:?}, direction '{}'.", pos, direction);
                    } else {
                        true
                    }
                }
                _ => panic!("Invalid character found in map at position {:?}.", pos),
            }
        }
        _ => {
            panic!("Invalid input direction given: '{direction}', must be one of ['<', '^', '>', 'v'].");
        }
    }
}

fn robot_position(map: &[Vec<char>]) -> (usize, usize) {
    let mut robot_pos: Option<(usize, usize)> = None;

    // Get robot position
    'outer: for (y, line) in map.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '@' {
                robot_pos = Some((x, y));
                break 'outer;
            }
        }
    }

    robot_pos.expect("No robot found in input map. (No '@' present).")
}

fn pos_at_direction(pos: (usize, usize), direction: char) -> (usize, usize) {
    match direction {
        '<' => (pos.0 - 1, pos.1),
        '^' => (pos.0, pos.1 - 1),
        '>' => (pos.0 + 1, pos.1),
        'v' => (pos.0, pos.1 + 1),
        _ => panic!("Invalid direction: {direction}."),
    }
}

fn find_connected_boxes(
    mut box_stack: Vec<((usize, usize), (usize, usize))>,
    map: &[Vec<char>],
    direction: char,
    pos: (usize, usize),
) -> Vec<((usize, usize), (usize, usize))> {
    match direction {
        '<' => match map[pos.1][pos.0 - 1] {
            ']' => {
                assert!(map[pos.1][pos.0 - 2] == '[');
                box_stack.push(((pos.0 - 1, pos.1), (pos.0 - 2, pos.1)));
                box_stack.extend(find_connected_boxes(
                    box_stack,
                    map,
                    direction,
                    (pos.0 - 3, pos.1),
                ));
                return box_stack;
            }
            '[' => {
                panic!(
                    "Bugged box in map: {}{}",
                    map[pos.1][pos.0 - 1],
                    map[pos.1][pos.0]
                );
            }

            _ => return box_stack,
        },
        '^' => {
            match map[pos.1][pos.0] {
                '[' => {
                    assert!(map[pos.1][pos.0 + 1] == ']');
                    // Check space above box
                    match map[pos.1 - 1][pos.0] {
                        ']' => {
                            // Box left facing
                            assert!(map[pos.1 - 1][pos.0 - 1] == '[');

                            //Check the section above, Distinct possible cases:
                            //    [][]  |    []..  |    ..[]  |    .[].  |    ....
                            // 1: .[].  | 2: .[].  | 3: .[].  | 4: .[].  | 5: .[].
                            //     ^          ^          ^          ^          ^

                            let above: String = [
                                map[pos.1 - 1][pos.0 - 2],
                                map[pos.1 - 1][pos.0 - 1],
                                map[pos.1 - 1][pos.0],
                                map[pos.1 - 1][pos.0 + 1],
                            ]
                            .iter()
                            .collect();

                            match &above as &str {
                                "[][]" => {
                                    // Left []
                                    box_stack
                                        .push(((pos.0 - 2, pos.1 - 1), (pos.0 - 1, pos.1 - 1)));
                                    box_stack.extend(find_connected_boxes(
                                        box_stack,
                                        map,
                                        direction,
                                        (pos.0 - 1, pos.1 - 1),
                                    ));

                                    // Right []
                                    box_stack.push(((pos.0, pos.1 - 1), (pos.0 + 1, pos.1 - 1)));
                                    box_stack.extend(find_connected_boxes(
                                        box_stack,
                                        map,
                                        direction,
                                        (pos.0, pos.1 - 1),
                                    ));
                                    return *box_stack;
                                }
                                "[].." | "[].#" | "[]##" => {
                                    // Left []
                                    box_stack
                                        .push(((pos.0 - 2, pos.1 - 1), (pos.0 - 1, pos.1 - 1)));
                                    box_stack.extend(find_connected_boxes(
                                        box_stack,
                                        map,
                                        direction,
                                        (pos.0 - 1, pos.1 - 1),
                                    ));
                                    return *box_stack;
                                }
                                "..[]" | "#.[]" | "##[]" => {
                                    // Right []
                                    box_stack.push(((pos.0, pos.1 - 1), (pos.0 + 1, pos.1 - 1)));
                                    box_stack.extend(find_connected_boxes(
                                        box_stack,
                                        map,
                                        direction,
                                        (pos.0, pos.1 - 1),
                                    ));
                                    return *box_stack;
                                }
                                ".[]." | "#[]." | ".[]#" | "#[]#" => {
                                    // Center []
                                    box_stack.push(((pos.0 - 1, pos.1 - 1), (pos.0, pos.1 - 1)));
                                    box_stack.extend(find_connected_boxes(
                                        box_stack,
                                        map,
                                        direction,
                                        (pos.0, pos.1 - 1),
                                    ));
                                    return *box_stack;
                                }
                                _ => return *box_stack,
                            }
                        }
                        '[' => {
                            // Box right facing
                            assert!(map[pos.1 - 1][pos.0 + 1] == ']');

                            //Check the section above, Distinct possible cases:
                            //    [][]  |    []..  |    ..[]  |    .[].  |    ....
                            // 1: .[].  | 2: .[].  | 3: .[].  | 4: .[].  | 5: .[].
                            //     ^          ^          ^          ^          ^

                            let above: String = [
                                map[pos.1 - 1][pos.0 - 1],
                                map[pos.1 - 1][pos.0],
                                map[pos.1 - 1][pos.0 + 1],
                                map[pos.1 - 1][pos.0 + 2],
                            ]
                            .iter()
                            .collect();

                            match &above as &str {
                                "[][]" => {
                                    // Left []
                                    box_stack.push(((pos.0 - 1, pos.1 - 1), (pos.0, pos.1 - 1)));
                                    box_stack.extend(find_connected_boxes(
                                        box_stack,
                                        map,
                                        direction,
                                        (pos.0, pos.1 - 1),
                                    ));

                                    // Right []
                                    box_stack
                                        .push(((pos.0 + 1, pos.1 - 1), (pos.0 + 2, pos.1 - 1)));
                                    box_stack.extend(find_connected_boxes(
                                        box_stack,
                                        map,
                                        direction,
                                        (pos.0 + 1, pos.1 - 1),
                                    ));
                                    return *box_stack;
                                }
                                "[].." | "[].#" | "[]##" => {
                                    // Left []
                                    box_stack
                                        .push(((pos.0 - 2, pos.1 - 1), (pos.0 - 1, pos.1 - 1)));
                                    box_stack.extend(find_connected_boxes(
                                        box_stack,
                                        map,
                                        direction,
                                        (pos.0 - 1, pos.1 - 1),
                                    ));
                                    return *box_stack;
                                }
                                "..[]" | "#.[]" | "##[]" => {
                                    // Right []
                                    box_stack.push(((pos.0, pos.1 - 1), (pos.0 + 1, pos.1 - 1)));
                                    box_stack.extend(find_connected_boxes(
                                        box_stack,
                                        map,
                                        direction,
                                        (pos.0, pos.1 - 1),
                                    ));
                                    return *box_stack;
                                }
                                ".[]." | "#[]." | ".[]#" | "#[]#" => {
                                    // Center []
                                    box_stack.push(((pos.0 - 1, pos.1 - 1), (pos.0, pos.1 - 1)));
                                    box_stack.extend(find_connected_boxes(
                                        box_stack,
                                        map,
                                        direction,
                                        (pos.0, pos.1 - 1),
                                    ));
                                    return *box_stack;
                                }
                                _ => return *box_stack,
                            }
                        }
                        _ => return *box_stack,
                    }
                }
                ']' => {
                    assert!(map[pos.1][pos.0 - 1] == '[');
                    // Check space above box
                    todo!()
                }

                _ => return *box_stack,
            }
        }
        '>' => match map[pos.1][pos.0 + 1] {
            '[' => {
                assert!(map[pos.1][pos.0 + 2] == ']');
                box_stack.push(((pos.0 + 1, pos.1), (pos.0 + 2, pos.1)));
                box_stack.extend(find_connected_boxes(
                    box_stack,
                    map,
                    direction,
                    (pos.0 + 3, pos.1),
                ));
                return *box_stack;
            }
            ']' => {
                panic!(
                    "Bugged box in map: {}{}",
                    map[pos.1][pos.0 - 1],
                    map[pos.1][pos.0]
                );
            }

            _ => *box_stack,
        },
        'v' => {
            todo!();
        }
        _ => {
            panic!("Invalid input direction given: '{direction}', must be one of ['<', '^', '>', 'v'].");
        }
    }
}

fn single_move(
    mut map: &Vec<Vec<char>>,
    pos: (usize, usize),
    direction: char,
) -> (&Vec<Vec<char>>, (usize, usize)) {
    if blocked(&map.clone(), pos, direction) {
        return (map, pos);
    }

    let mut box_stack: Vec<((usize, usize), (usize, usize))> = Vec::new(); // Vec<(Position of '[', Position of ']')>

    let p = pos_at_direction(pos, direction);
    match map[p.1][p.0] {
        '[' => box_stack.push(((p.0, p.1), (p.0 + 1, p.1))),
        ']' => box_stack.push(((p.0, p.1), (p.0 - 1, p.1))),
        '.' => return (map, p),
        '#' => return (map, pos),
        _ => panic!(
            "Invalid character in map: '{}' at position {:?}",
            map[p.1][p.0], p
        ),
    }

    (map, pos)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_left1() {
        let input = r"
##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############

<
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, false);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_left2() {
        let input = r"
##############
##......##..##
##..........##
##..[][][]@.##
##....[]....##
##..........##
##############

<
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, false);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_left3() {
        let input = r"
##############
##......##..##
##..........##
##[][][][]@.##
##....[]....##
##..........##
##############

<
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_left4() {
        let input = r"
##############
##......##..##
##..........##
##@.[][][]..##
##....[]....##
##..........##
##############

<
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_right1() {
        let input = r"
##############
##......##..##
##..........##
##...@[][][]##
##....[]....##
##..........##
##############

>
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_right2() {
        let input = r"
##############
##......##..##
##..........##
##...@[][]..##
##....[]....##
##..........##
##############

>
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, false);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_right3() {
        let input = r"
##############
##......##..##
##..........##
##....[][].@##
##....[]....##
##..........##
##############

>
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_above1() {
        let input = r"
##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############

^
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, false);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_above2() {
        let input = r"
##############
##......##..##
##.......[].##
##....[][]@.##
##....[]....##
##..........##
##############

^
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_above3() {
        let input = r"
##############
##......#...##
##.......[].##
##....[][]@.##
##....[]....##
##..........##
##############

^
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, false);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_above4() {
        let input = r"
##############
##......[]..##
##.......[].##
##....[][]@.##
##....[]....##
##..........##
##############

^
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_above5() {
        let input = r"
##############
##......[][]##
##.......[].##
##....[][]@.##
##....[]....##
##..........##
##############

^
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_above6() {
        let input = r"
##############
##........[]##
##.......[].##
##....[][]@.##
##....[]....##
##..........##
##############

^
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_above7() {
        let input = r"
##############
##......[]@.##
##.......[].##
##....[][]..##
##....[]....##
##..........##
##############

^
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_below1() {
        let input = r"
##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##......##..##
##############

v
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, false);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_below2() {
        let input = r"
##############
##......##..##
##.......[].##
##....[][]@.##
##....[].[].##
##......##..##
##############

v
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_below3() {
        let input = r"
##############
##......#...##
##.......[].##
##....[][]@.##
##....[].[].##
##......#...##
##############

v
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, false);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_below4() {
        let input = r"
##############
##......[]..##
##.......[].##
##....[][]@.##
##....[].[].##
##......[]..##
##############

v
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_below5() {
        let input = r"
##############
##......[][]##
##.......[].##
##....[][]@.##
##....[].[].##
##......[][]##
##############

v
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_below6() {
        let input = r"
##############
##........[]##
##.......[].##
##....[][]@.##
##....[].[].##
##........[]##
##############

v
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // More idiomatic in this case
    fn test_check_blocked_below7() {
        let input = r"
##############
##....[]....##
##..........##
##......[]..##
##.......[].##
##....[][]@.##
##############

v
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let rpos = robot_position(&map);

        let result = blocked(&map, rpos, moves[0]);
        assert_eq!(result, true);
    }
}
