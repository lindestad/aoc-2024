use std::collections::HashSet;

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
                blocked_internal(map, (pos.0 - 1, pos.1), direction, depth + 1) // Check neighbor
            }
            '[' => {
                assert!(map[pos.1][pos.0] == ']');
                blocked_internal(map, (pos.0 - 1, pos.1), direction, depth + 1) // Check neighbor
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
                    // Check neighbor
                    blocked_internal(map, (pos.0 + 1, pos.1), direction, depth + 1)
                }
                ']' => {
                    assert!(map[pos.1][pos.0] == '[');
                    // Check neighbor
                    blocked_internal(map, (pos.0 + 1, pos.1), direction, depth + 1)
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

#[allow(clippy::type_complexity)]
fn find_connected_boxes(
    box_stack: &mut HashSet<((usize, usize), (usize, usize))>,
    map: &[Vec<char>],
    direction: char,
    pos: (usize, usize),
) {
    if box_stack.is_empty() {
        if map[pos.1][pos.0] == '[' && map[pos.1][pos.0 + 1] == ']' {
            box_stack.insert(((pos.0, pos.1), (pos.0 + 1, pos.1)));
        } else if map[pos.1][pos.0 - 1] == '[' && map[pos.1][pos.0] == ']' {
            box_stack.insert(((pos.0 - 1, pos.1), (pos.0, pos.1)));
        } else {
            return;
        }
    }
    match direction {
        '<' => match map[pos.1][pos.0 - 1] {
            ']' => {
                assert!(map[pos.1][pos.0 - 2] == '[');
                box_stack.insert(((pos.0 - 2, pos.1), (pos.0 - 1, pos.1)));
                find_connected_boxes(box_stack, map, direction, (pos.0 - 2, pos.1));
            }
            '[' => {
                assert!(map[pos.1][pos.0] == ']');

                box_stack.insert(((pos.0 - 1, pos.1), (pos.0, pos.1)));

                // Just use code from left side:
                find_connected_boxes(box_stack, map, direction, (pos.0 - 1, pos.1))
            }

            _ => (),
        },
        '^' => {
            match map[pos.1][pos.0] {
                '[' => {
                    assert!(map[pos.1][pos.0 + 1] == ']');

                    //Check the section above, Distinct possible cases:
                    //    [][]  |    []..  |    ..[]  |    .[].  |    ....
                    // 1: .[].  | 2: .[].  | 3: .[].  | 4: .[].  | 5: .[].
                    //     ^          ^          ^          ^          ^

                    assert!(map[pos.1][pos.0 + 1] == ']');
                    // Try the default window above (starting one column left)
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
                            box_stack.insert(((pos.0 - 1, pos.1 - 1), (pos.0, pos.1 - 1)));
                            find_connected_boxes(box_stack, map, direction, (pos.0 - 1, pos.1 - 1));

                            // Right []
                            box_stack.insert(((pos.0 + 1, pos.1 - 1), (pos.0 + 2, pos.1 - 1)));
                            find_connected_boxes(box_stack, map, direction, (pos.0 + 1, pos.1 - 1))
                        }
                        "[].." | "[].#" | "[]##" | "[].[" | "[]#[" => {
                            // Left []
                            box_stack.insert(((pos.0 - 1, pos.1 - 1), (pos.0, pos.1 - 1)));
                            find_connected_boxes(box_stack, map, direction, (pos.0 - 1, pos.1 - 1))
                        }
                        "..[]" | "#.[]" | "##[]" | "].[]" | "]#[]" => {
                            // Right []
                            box_stack.insert(((pos.0 + 1, pos.1 - 1), (pos.0 + 2, pos.1 - 1)));
                            find_connected_boxes(box_stack, map, direction, (pos.0 + 1, pos.1 - 1))
                        }
                        ".[]." | "#[]." | ".[]#" | "#[]#" | "][][" | "][]." | "][]#" | ".[]["
                        | "#[][" => {
                            // Center []
                            box_stack.insert(((pos.0, pos.1 - 1), (pos.0 + 1, pos.1 - 1)));
                            find_connected_boxes(box_stack, map, direction, (pos.0, pos.1 - 1))
                        }
                        // Explicitly check for expected cases for bug testing purposes
                        "]..[" | "]..." | "...[" | "####" | ".###" | "..##" | "...#" | "...."
                        | "#..." | "##.." | "###." | "]###" | "###[" | "#..[" | "]..#" | "#..#" => {
                        }
                        _ => panic!("Unexpected pattern encountered: {}", &above),
                    }
                }
                ']' => {
                    assert!(map[pos.1][pos.0 - 1] == '[');

                    // Just use code from left side:
                    find_connected_boxes(box_stack, map, direction, (pos.0 - 1, pos.1))
                }

                _ => (),
            }
        }
        '>' => match map[pos.1][pos.0 + 1] {
            '[' => {
                assert!(map[pos.1][pos.0 + 2] == ']');
                box_stack.insert(((pos.0 + 1, pos.1), (pos.0 + 2, pos.1)));
                find_connected_boxes(box_stack, map, direction, (pos.0 + 2, pos.1));
            }
            ']' => {
                assert!(map[pos.1][pos.0] == '[');

                // Just use code from left side:
                find_connected_boxes(box_stack, map, direction, (pos.0 + 1, pos.1))
            }

            _ => (),
        },
        'v' => {
            match map[pos.1][pos.0] {
                '[' => {
                    assert!(map[pos.1][pos.0 + 1] == ']');

                    //Check the section below, Distinct possible cases:
                    //     v          v          v          v          v
                    // 1: .[].  | 2: .[].  | 3: .[].  | 4: .[].  | 5: .[].
                    //    [][]  |    []..  |    ..[]  |    .[].  |    ....

                    let below: String = [
                        map[pos.1 + 1][pos.0 - 1],
                        map[pos.1 + 1][pos.0],
                        map[pos.1 + 1][pos.0 + 1],
                        map[pos.1 + 1][pos.0 + 2],
                    ]
                    .iter()
                    .collect();

                    match &below as &str {
                        "[][]" => {
                            // Left []
                            box_stack.insert(((pos.0 - 1, pos.1 + 1), (pos.0, pos.1 + 1)));
                            find_connected_boxes(box_stack, map, direction, (pos.0 - 1, pos.1 + 1));

                            // Right []
                            box_stack.insert(((pos.0 + 1, pos.1 + 1), (pos.0 + 2, pos.1 + 1)));
                            find_connected_boxes(box_stack, map, direction, (pos.0 + 1, pos.1 + 1))
                        }
                        "[].." | "[].#" | "[]##" | "[].[" | "[]#[" => {
                            // Left []
                            box_stack.insert(((pos.0 - 1, pos.1 + 1), (pos.0, pos.1 + 1)));
                            find_connected_boxes(box_stack, map, direction, (pos.0 - 1, pos.1 + 1))
                        }
                        "..[]" | "#.[]" | "##[]" | "].[]" | "]#[]" => {
                            // Right []
                            box_stack.insert(((pos.0 + 1, pos.1 + 1), (pos.0 + 2, pos.1 + 1)));
                            find_connected_boxes(box_stack, map, direction, (pos.0 + 1, pos.1 + 1))
                        }
                        ".[]." | "#[]." | ".[]#" | "#[]#" | "][][" | "][]." | "][]#" | ".[]["
                        | "#[][" => {
                            // Center []
                            box_stack.insert(((pos.0, pos.1 + 1), (pos.0 + 1, pos.1 + 1)));
                            find_connected_boxes(box_stack, map, direction, (pos.0, pos.1 + 1))
                        }
                        // Explicitly check for expected cases for bug testing purposes
                        "]..[" | "]..." | "...[" | "####" | ".###" | "..##" | "...#" | "...."
                        | "#..." | "##.." | "###." | "]###" | "###[" | "#..[" | "]..#" | "#..#" => {
                        }
                        _ => panic!("Unexpected pattern encountered: {}", &below),
                    }
                }
                ']' => {
                    assert!(map[pos.1][pos.0 - 1] == '[');

                    // Just use code from left side:
                    find_connected_boxes(box_stack, map, direction, (pos.0 - 1, pos.1))
                }

                _ => (),
            }
        }
        _ => {
            panic!("Invalid input direction given: '{direction}', must be one of ['<', '^', '>', 'v'].");
        }
    }
}

#[allow(clippy::ptr_arg)] // Incorrect clippy suggestion as can't use &map.clone() on slice
fn single_move(map: &mut Vec<Vec<char>>, pos: (usize, usize), direction: char) -> (usize, usize) {
    if blocked(&map.clone(), pos, direction) {
        return pos;
    }

    let mut box_set: HashSet<((usize, usize), (usize, usize))> = HashSet::new(); // Vec<(Position of '[', Position of ']')>
    find_connected_boxes(
        &mut box_set,
        map,
        direction,
        pos_at_direction(pos, direction),
    );

    // Clear the relevant boxes
    for bbox in box_set.iter() {
        // Left brace
        map[bbox.0 .1][bbox.0 .0] = '.';

        // Right brace
        map[bbox.1 .1][bbox.1 .0] = '.';
    }

    // Write new boxes
    for bbox in box_set.iter() {
        // Left brace
        let new_pos_left_brace = pos_at_direction(bbox.0, direction);
        map[new_pos_left_brace.1][new_pos_left_brace.0] = '[';

        // Right brace
        let new_pos_right_brace = pos_at_direction(bbox.1, direction);
        map[new_pos_right_brace.1][new_pos_right_brace.0] = ']';
    }

    pos_at_direction(pos, direction) // New robot position
}

#[allow(clippy::ptr_arg)] // Incorrect clippy suggestion as can't use &map.clone() on slice
pub fn solve_maze(map: &Vec<Vec<char>>, moves: Vec<char>) -> Vec<Vec<char>> {
    let rpos = robot_position(map);
    let mut map = map.clone();
    map[rpos.1][rpos.0] = '.'; // Clear robot from map

    let mut rpos = rpos;
    for direction in moves {
        rpos = single_move(&mut map, rpos, direction);
    }

    map[rpos.1][rpos.0] = '@'; // Put the robot back
    map
}

pub fn gps(map: Vec<Vec<char>>) -> i64 {
    let mut answer = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '[' {
                answer += (100 * y) + x;
            }
        }
    }

    answer as i64
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
    fn test_check_blocked_left5() {
        let input = r"
##############
##..........##
##.[][][]...##
##..........##
##############
        ";
        let map = get_map(input);
        let direction = '<';
        let start_positions = [(6, 2), (7, 2)];
        for rpos in start_positions {
            let result = blocked(&map, rpos, direction);
            assert_eq!(result, false);
        }
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
    fn test_check_blocked_right4() {
        let input = r"
##############
##..........##
##...[][][].##
##..........##
##############
        ";
        let map = get_map(input);
        let direction = '>';
        let start_positions = [(5, 2), (6, 2)];
        for rpos in start_positions {
            let result = blocked(&map, rpos, direction);
            assert_eq!(result, false);
        }
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

    #[test]
    fn test_find_connect_boxes1() {
        let input = r"
##############
##..........##
##....[]....##
##..........##
##############
        ";
        let map = get_map(input);
        let correct_boxes: HashSet<((usize, usize), (usize, usize))> =
            HashSet::from([((6, 2), (7, 2))]);
        for direction in ['<', '^', '>', 'v'] {
            for pos in [(6, 2), (7, 2)] {
                // Start from left and right bracket
                let mut boxes: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
                find_connected_boxes(&mut boxes, &map, direction, pos);
                assert_eq!(boxes, correct_boxes);
            }
        }
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_find_connect_boxes2() {
        let input = r"
##############
##...[].....##
##..[][][]..##
##.....[]...##
##############
        ";
        let map = get_map(input);
        let correct_boxes: Vec<HashSet<((usize, usize), (usize, usize))>> = vec![
            HashSet::from([((6, 2), (7, 2)), ((4, 2), (5, 2))]),
            HashSet::from([((6, 2), (7, 2)), ((5, 1), (6, 1))]),
            HashSet::from([((6, 2), (7, 2)), ((8, 2), (9, 2))]),
            HashSet::from([((6, 2), (7, 2)), ((7, 3), (8, 3))]),
        ];
        for (idx, direction) in ['<', '^', '>', 'v'].iter().enumerate() {
            for pos in [(6, 2), (7, 2)] {
                // Start from left and right bracket
                let mut boxes: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
                find_connected_boxes(&mut boxes, &map, *direction, pos);
                assert_eq!(boxes, correct_boxes[idx]);
            }
        }
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_find_connect_boxes3() {
        let input = r"
##############
##.....[]...##
##..[][][]..##
##...[].....##
##############
        ";
        let map = get_map(input);
        let correct_boxes: Vec<HashSet<((usize, usize), (usize, usize))>> = vec![
            HashSet::from([((6, 2), (7, 2)), ((4, 2), (5, 2))]),
            HashSet::from([((6, 2), (7, 2)), ((7, 1), (8, 1))]),
            HashSet::from([((6, 2), (7, 2)), ((8, 2), (9, 2))]),
            HashSet::from([((6, 2), (7, 2)), ((5, 3), (6, 3))]),
        ];
        for (idx, direction) in ['<', '^', '>', 'v'].iter().enumerate() {
            for pos in [(6, 2), (7, 2)] {
                // Start from left and right bracket
                let mut boxes: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
                find_connected_boxes(&mut boxes, &map, *direction, pos);
                assert_eq!(boxes, correct_boxes[idx]);
            }
        }
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_find_connect_boxes4() {
        let input = r"
##############
##....[]....##
##.....[]...##
##[][][][][]##
##...[].....##
##..[]......##
##############
        ";
        let map = get_map(input);
        let correct_boxes: Vec<HashSet<((usize, usize), (usize, usize))>> = vec![
            HashSet::from([((6, 3), (7, 3)), ((4, 3), (5, 3)), ((2, 3), (3, 3))]),
            HashSet::from([((6, 3), (7, 3)), ((7, 2), (8, 2)), ((6, 1), (7, 1))]),
            HashSet::from([((6, 3), (7, 3)), ((8, 3), (9, 3)), ((10, 3), (11, 3))]),
            HashSet::from([((6, 3), (7, 3)), ((5, 4), (6, 4)), ((4, 5), (5, 5))]),
        ];
        for (idx, direction) in ['<', '^', '>', 'v'].iter().enumerate() {
            for pos in [(6, 3), (7, 3)] {
                // Start from left and right bracket
                let mut boxes: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
                find_connected_boxes(&mut boxes, &map, *direction, pos);

                println!("Testing direction: '{direction}'.");
                assert_eq!(boxes, correct_boxes[idx]);
            }
        }
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_find_connect_boxes5() {
        let input = r"
##############
##....[][]..##
##.....[]...##
##[][][][][]##
##...[].....##
##..[][]....##
##############
        ";
        let map = get_map(input);
        let correct_boxes: Vec<HashSet<((usize, usize), (usize, usize))>> = vec![
            HashSet::from([((6, 3), (7, 3)), ((4, 3), (5, 3)), ((2, 3), (3, 3))]),
            HashSet::from([
                ((6, 3), (7, 3)),
                ((7, 2), (8, 2)),
                ((6, 1), (7, 1)),
                ((8, 1), (9, 1)),
            ]),
            HashSet::from([((6, 3), (7, 3)), ((8, 3), (9, 3)), ((10, 3), (11, 3))]),
            HashSet::from([
                ((6, 3), (7, 3)),
                ((5, 4), (6, 4)),
                ((4, 5), (5, 5)),
                ((6, 5), (7, 5)),
            ]),
        ];
        for (idx, direction) in ['<', '^', '>', 'v'].iter().enumerate() {
            for pos in [(6, 3), (7, 3)] {
                // Start from left and right bracket
                let mut boxes: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
                find_connected_boxes(&mut boxes, &map, *direction, pos);

                println!("Testing direction: '{direction}'.");
                assert_eq!(boxes, correct_boxes[idx]);
            }
        }
    }

    #[test]
    fn test_single_move1() {
        let input = r"
##############
##..........##
##....[]....##
##..........##
##############
        ";
        let map = get_map(input);
        print_map(&map);
        let tests = vec![
            // For a left push, the robot must be immediately to the right of the box.
            (
                '<',
                vec![(8, 2)], // starting at (8,2) so that pos_at_direction((8,2), '<') is (7,2)
                r"
##############
##..........##
##...[].....##
##..........##
##############
                ",
            ),
            // For an upward push, the robot must be immediately below the box.
            (
                '^',
                vec![(6, 3), (7, 3)], // try both columns in case the box was started from its left or right cell
                r"
##############
##....[]....##
##..........##
##..........##
##############
                ",
            ),
            // For a right push, the robot must be immediately to the left of the box.
            (
                '>',
                vec![(5, 2)], // so that pos_at_direction((5,2), '>') is (6,2)
                r"
##############
##..........##
##.....[]...##
##..........##
##############
                ",
            ),
            // For a downward push, the robot must be immediately above the box.
            (
                'v',
                vec![(6, 1), (7, 1)], // try both positions
                r"
##############
##..........##
##..........##
##....[]....##
##############
                ",
            ),
        ];
        for (direction, start_positions, expected_str) in tests {
            let expected_map = get_map(expected_str);
            for pos in start_positions {
                let mut result = map.clone();
                println!("ran");
                print_map(&map);
                single_move(&mut result, pos, direction);
                if result != expected_map {
                    println!("Direction '{}' starting at {:?} produced:", direction, pos);
                    print_map(&result);
                    println!("Expected:");
                    print_map(&expected_map);
                }
                assert_eq!(result, expected_map);
            }
        }
    }

    #[test]
    fn test_single_move2() {
        let input = r"
######
##[]##
######
        ";
        let map = get_map(input);
        let answers = [
            r"
######
##[]##
######
        ",
            r"
######
##[]##
######
        ",
            r"
######
##[]##
######
        ",
            r"
######
##[]##
######
        ",
        ];
        let directions = vec!['<', '^', '>', 'v'];
        let start_positions = [(2, 1), (3, 1)];
        for pos in start_positions {
            for (answer, direction) in answers.iter().zip(directions.clone()) {
                let expected_map = get_map(answer);
                let mut result = map.clone();
                single_move(&mut result, pos, direction);
                if result.clone() != expected_map {
                    println!("Correct: ");
                    print_map(&expected_map);
                    println!("\nActual:");
                    print_map(&result.clone());
                }
                assert_eq!(result, expected_map);
            }
        }
    }

    #[test]
    fn test_single_move_left1() {
        let input = r"
##############
##..........##
##..[][]....##
##..........##
##############
        ";
        let map = get_map(input);
        let answer = r"
##############
##..........##
##.[][].....##
##..........##
##############
        ";
        let direction = '<';
        let start_pos = (8, 2);
        let expected_map = get_map(answer);
        let mut result = map.clone();
        single_move(&mut result, start_pos, direction);
        if result.clone() != expected_map {
            println!("Correct: ");
            print_map(&expected_map);
            println!("\nActual:");
            print_map(&result.clone());

            assert_eq!(result, expected_map);
        }
    }

    #[test]
    fn test_single_move_left2() {
        let input = r"
##############
##..........##
##[][][]....##
##..........##
##############
        ";
        let map = get_map(input);
        let answer = r"
##############
##..........##
##[][][]....##
##..........##
##############
        ";
        let direction = '<';
        let start_positions = [(8, 2)];
        for pos in start_positions {
            let expected_map = get_map(answer);
            let mut result = map.clone();
            single_move(&mut result, pos, direction);
            if result.clone() != expected_map {
                println!("Correct: ");
                print_map(&expected_map);
                println!("\nActual:");
                print_map(&result.clone());
            }
            assert_eq!(result, expected_map);
        }
    }

    #[test]
    fn test_single_move_left3() {
        let input = r"
##############
##..........##
##.[][][]...##
##..........##
##############
        ";
        let map = get_map(input);
        let answer = r"
##############
##..........##
##[][][]....##
##..........##
##############
        ";
        let direction = '<';
        let start_positions = [(9, 2)];
        for pos in start_positions {
            let expected_map = get_map(answer);
            let mut result = map.clone();
            single_move(&mut result, pos, direction);
            if result.clone() != expected_map {
                print_map(&map);
                println!("{}", map[pos.1][pos.0]);
                println!("Correct: ");
                print_map(&expected_map);
                println!("\nActual:");
                print_map(&result.clone());
            }
            assert_eq!(result, expected_map);
        }
    }

    #[test]
    fn test_single_move_right1() {
        let input = r"
##############
##..........##
##....[][]..##
##..........##
##############
        ";
        let map = get_map(input);
        let answer = r"
##############
##..........##
##.....[][].##
##..........##
##############
        ";
        let direction = '>';
        let start_positions = [(5, 2)];
        for pos in start_positions {
            let expected_map = get_map(answer);
            let mut result = map.clone();
            single_move(&mut result, pos, direction);
            if result.clone() != expected_map {
                println!("Correct: ");
                print_map(&expected_map);
                println!("\nActual:");
                print_map(&result.clone());
            }
            assert_eq!(result, expected_map);
        }
    }

    #[test]
    fn test_single_move_right2() {
        let input = r"
##############
##..........##
##....[][][]##
##..........##
##############
        ";
        let map = get_map(input);
        let answer = r"
##############
##..........##
##....[][][]##
##..........##
##############
        ";
        let direction = '>';
        let start_positions = [(5, 2)];
        for pos in start_positions {
            let expected_map = get_map(answer);
            let mut result = map.clone();
            single_move(&mut result, pos, direction);
            if result.clone() != expected_map {
                println!("Correct: ");
                print_map(&expected_map);
                println!("\nActual:");
                print_map(&result.clone());
            }
            assert_eq!(result, expected_map);
        }
    }

    #[test]
    fn test_single_move_right3() {
        let input = r"
##############
##..........##
##...[][][].##
##..........##
##############
        ";
        let map = get_map(input);
        let answer = r"
##############
##..........##
##....[][][]##
##..........##
##############
        ";
        let direction = '>';
        let start_positions = [(4, 2)];
        for pos in start_positions {
            let expected_map = get_map(answer);
            let mut result = map.clone();
            single_move(&mut result, pos, direction);
            if result.clone() != expected_map {
                print_map(&map);
                println!("{}", map[pos.1][pos.0]);
                println!("Correct: ");
                print_map(&expected_map);
                println!("\nActual:");
                print_map(&result.clone());
            }
            assert_eq!(result, expected_map);
        }
    }

    #[test]
    fn test_single_move_right4() {
        let input = r"
####################
##[]..[]....[]..[]##
##[]..........[]..##
##......@[][].[].[]#
##....[]..[]..[]..##
##..##....[]......##
##...[].......[]..##
##.....[]..[].[][]##
##........[]......##
####################
        ";
        let map = get_map(input);
        let answer = r"
####################
##[]..[]....[]..[]##
##[]..........[]..##
##.......@[][][].[]#
##....[]..[]..[]..##
##..##....[]......##
##...[].......[]..##
##.....[]..[].[][]##
##........[]......##
####################
        ";
        let direction = '>';
        let pos = robot_position(&map);
        let expected_map = get_map(answer);
        let mut result = map.clone();
        result[pos.1][pos.0] = '.';
        single_move(&mut result, pos, direction);
        let new_pos = pos_at_direction(pos, direction);
        result[new_pos.1][new_pos.0] = '@';
        if result.clone() != expected_map {
            print_map(&map);
            println!("{}", map[pos.1][pos.0]);
            println!("Correct: ");
            print_map(&expected_map);
            println!("\nActual:");
            print_map(&result.clone());
        }
        assert_eq!(result, expected_map);
    }

    #[test]
    fn test_single_move_up1() {
        let input = r"
##############
##..........##
##...[].....##
##....[]....##
##..........##
##############
        ";
        let map = get_map(input);
        let answer = r"
##############
##...[].....##
##....[]....##
##..........##
##..........##
##############
        ";
        let direction = '^';
        let start_positions = [(6, 4), (7, 4)];
        for pos in start_positions {
            let expected_map = get_map(answer);
            let mut result = map.clone();
            single_move(&mut result, pos, direction);
            if result.clone() != expected_map {
                println!("Correct: ");
                print_map(&expected_map);
                println!("\nActual:");
                print_map(&result.clone());
            }
            assert_eq!(result, expected_map);
        }
    }

    #[test]
    fn test_single_move_up2() {
        let input = r"
##############
##....[]....##
##...[].....##
##....[]....##
##..........##
##############
        ";
        let map = get_map(input);
        let answer = r"
##############
##....[]....##
##...[].....##
##....[]....##
##..........##
##############
        ";
        let direction = '^';
        let start_positions = [(6, 4), (7, 4)];
        for pos in start_positions {
            let expected_map = get_map(answer);
            let mut result = map.clone();
            single_move(&mut result, pos, direction);
            if result.clone() != expected_map {
                println!("Correct: ");
                print_map(&expected_map);
                println!("\nActual:");
                print_map(&result.clone());
            }
            assert_eq!(result, expected_map);
        }
    }

    #[test]
    fn test_single_move_up3() {
        let input = r"
##############
##...[].....##
##.....[]...##
##....[]....##
##..........##
##############
        ";
        let map = get_map(input);
        let answer = r"
##############
##...[][]...##
##....[]....##
##..........##
##..........##
##############
        ";
        let direction = '^';
        let start_positions = [(6, 4), (7, 4)];
        for pos in start_positions {
            let expected_map = get_map(answer);
            let mut result = map.clone();
            single_move(&mut result, pos, direction);
            if result.clone() != expected_map {
                println!("Correct: ");
                print_map(&expected_map);
                println!("\nActual:");
                print_map(&result.clone());
            }
            assert_eq!(result, expected_map);
        }
    }

    #[test]
    fn test_single_move_up4() {
        let input = r"
##############
##...[].[]..##
##.....[]...##
##....[]....##
##..........##
##############
        ";
        let map = get_map(input);
        let answer = r"
##############
##...[].[]..##
##.....[]...##
##....[]....##
##..........##
##############
        ";
        let direction = '^';
        let start_positions = [(6, 4), (7, 4)];
        for pos in start_positions {
            let expected_map = get_map(answer);
            let mut result = map.clone();
            single_move(&mut result, pos, direction);
            if result.clone() != expected_map {
                println!("Correct: ");
                print_map(&expected_map);
                println!("\nActual:");
                print_map(&result.clone());
            }
            assert_eq!(result, expected_map);
        }
    }

    #[test]
    fn test_single_move_up5() {
        let input = r"
##############
##..........##
##......[]..##
##.....[]...##
##....[]....##
##..........##
##############
        ";
        let map = get_map(input);
        let answer = r"
##############
##......[]..##
##.....[]...##
##....[]....##
##..........##
##..........##
##############
        ";
        let direction = '^';
        let start_positions = [(6, 5), (7, 5)];
        for pos in start_positions {
            let expected_map = get_map(answer);
            let mut result = map.clone();
            single_move(&mut result, pos, direction);
            if result.clone() != expected_map {
                println!("Correct: ");
                print_map(&expected_map);
                println!("\nActual:");
                print_map(&result.clone());
            }
            assert_eq!(result, expected_map);
        }
    }

    #[test]
    fn test_single_move_up6() {
        let input = r"
##############
##..........##
##...[].[]..##
##.....[]...##
##....[]....##
##..........##
##############
        ";
        let map = get_map(input);
        let answer = r"
##############
##......[]..##
##...[][]...##
##....[]....##
##..........##
##..........##
##############
        ";
        let direction = '^';
        let start_positions = [(6, 5), (7, 5)];
        for pos in start_positions {
            let expected_map = get_map(answer);
            let mut result = map.clone();
            single_move(&mut result, pos, direction);
            if result.clone() != expected_map {
                println!("Correct: ");
                print_map(&expected_map);
                println!("\nActual:");
                print_map(&result.clone());
            }
            assert_eq!(result, expected_map);
        }
    }

    #[test]
    fn test_single_move_down1() {
        let input = r"
####################
##[]..[]......[][]##
##[]...........[].##
##...........@[][]##
##..........[].[].##
##..##[]..[].[]...##
##...[]...[]..[]..##
##.....[]..[].[][]##
##........[]......##
####################
";

        let direction = 'v';
        let map = get_map(input);

        let solution = r"
####################
##[]..[]......[][]##
##[]...........[].##
##............[][]##
##...........@.[].##
##..##[]..[][]....##
##...[]...[].[]...##
##.....[]..[].[][]##
##........[]..[]..##
####################
";
        let solution = get_map(solution);

        let mut result = map.clone();

        let rpos = robot_position(&map);
        result[rpos.1][rpos.0] = '.';
        let new_rpos = single_move(&mut result, rpos, direction);
        result[new_rpos.1][new_rpos.0] = '@';

        if result.clone() != solution {
            println!("Correct: ");
            print_map(&solution);
            println!("\nActual:");
            print_map(&result.clone());
        }
        assert_eq!(result, solution);
    }

    #[test]
    fn test_solve1() {
        let input = r"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
        ";
        let map = expand_map(&get_map(input));
        let moves = get_moves(input);
        let result = solve_maze(&map, moves);

        let solution = r"
##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############
        ";
        let solution = get_map(solution);
        println!("Result:");
        print_map(&result);
        println!("\nCorrect:");
        print_map(&solution);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_solve2() {
        let input = r"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<
        ";
        let map = expand_map(&get_map(input));
        let moves = get_moves(input);
        let result = solve_maze(&map, moves);

        let solution = r"
##############
##......##..##
##..........##
##...[][]@..##
##....[]....##
##..........##
##############
        ";
        let solution = get_map(solution);
        println!("Result:");
        print_map(&result);
        println!("\nCorrect:");
        print_map(&solution);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_solve3() {
        let input = r"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
        ";
        let map = expand_map(&get_map(input));
        let moves = get_moves(input);
        let result = solve_maze(&map, moves);

        let solution = r"
##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############
        ";
        let solution = get_map(solution);
        println!("Result:");
        print_map(&result);
        println!("\nCorrect:");
        print_map(&solution);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_solve4() {
        let input = r"
####################
##[]..[]....[]..[]##
##[]..........[]..##
##......@[][].[].[]#
##....[]..[]..[]..##
##..##....[]......##
##...[].......[]..##
##.....[]..[].[][]##
##........[]......##
####################

>
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let result = solve_maze(&map, moves);

        let solution = r"
####################
##[]..[]....[]..[]##
##[]..........[]..##
##.......@[][][].[]#
##....[]..[]..[]..##
##..##....[]......##
##...[].......[]..##
##.....[]..[].[][]##
##........[]......##
####################
        ";
        let solution = get_map(solution);
        println!("Result:");
        print_map(&result);
        println!("\nCorrect:");
        print_map(&solution);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_solve5() {
        let input = r"
####################
##[]..[]......[][]##
##[]........@..[].##
##..........[][][]##
##...........[][].##
##..##[]..[]......##
##...[]...[]..[]..##
##.....[]..[].[][]##
##........[]......##
####################

v
        ";
        let map = get_map(input);
        let moves = get_moves(input);
        let result = solve_maze(&map, moves);

        let solution = r"
####################
##[]..[]......[][]##
##[]...........[].##
##..........@.[][]##
##..........[].[].##
##..##[]..[].[]...##
##...[]...[]..[]..##
##.....[]..[].[][]##
##........[]......##
####################
        ";
        let solution = get_map(solution);
        println!("Result:");
        print_map(&result);
        println!("\nCorrect:");
        print_map(&solution);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_solve_large() {
        let input = r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
        let map = expand_map(&get_map(input));
        let moves = get_moves(input);

        let solution = r"
####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################
";

        let solution = get_map(solution);
        let result = solve_maze(&map, moves);

        println!("Result:");
        print_map(&result);
        println!("\nCorrect:");
        print_map(&solution);

        assert_eq!(result, solution);
    }
}
