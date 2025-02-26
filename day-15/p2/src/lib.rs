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

pub fn blocked(map: &Vec<Vec<char>>, pos: (usize, usize), direction: char) -> bool {
    match direction {
        '<' => match map[pos.1][pos.0 - 1] {
            '#' => true,
            '.' => false,
            ']' => {
                assert!(map[pos.1][pos.0 - 2] == '[');
                blocked(map, (pos.0 - 2, pos.1), direction) // Check neighbor
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
                    todo!();
                }
                ']' => {
                    todo!();
                }
                '.' | '@' => {
                    todo!();
                }
                '#' => panic!("Trying to check from bondary and up, is this intentional? Passed position {:?}, direction '{}'.", pos, direction),
                _ => panic!("Invalid character found in map at position {:?}.", pos),
            }
        }
        '>' => {
            match map[pos.1][pos.0 + 1] {
                '#' => true,
                '.' => false,
                '[' => {
                    assert!(map[pos.1][pos.0 + 2] == ']');
                    blocked(map, (pos.0 + 2, pos.1), direction) // Check neighbor
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
            todo!();
        }
        _ => {
            panic!("Invalid input direction given: '{direction}', must be one of ['<', '^', '>', 'v'].");
        }
    }
}

pub fn robot_position(map: &[Vec<char>]) -> (usize, usize) {
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
}
