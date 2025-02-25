fn get_moves(input: &str) -> Vec<char> {
    let mut moves = Vec::new();
    for c in input.chars() {
        match c {
            '<' | '^' | '>' | 'v' => moves.push(c),
            _ => (),
        }
    }
    moves
}

fn get_map(input: &str) -> Vec<Vec<char>> {
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
    (get_map(input), get_moves(input))
}

pub fn print_map(map: Vec<Vec<char>>) {
    println!(" ");
    for line in map.iter() {
        println!("{}", line.iter().collect::<String>());
    }
    println!(" ");
}

pub fn solve_map(map: Vec<Vec<char>>, moves: Vec<char>) -> Vec<Vec<char>> {
    let mut robot_pos: Option<(usize, usize)> = None;

    // Get initial robot position
    'outer: for (y, line) in map.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '@' {
                robot_pos = Some((x, y));
                break 'outer;
            }
        }
    }

    let mut robot_pos = robot_pos.expect("No robot found in input map. (No '@' present).");

    let mut map = map.clone();

    // Clear robot from map to avoid having to keep updating it
    map[robot_pos.1][robot_pos.0] = '.';

    for mv in moves {
        match mv {
            '<' => {
                let mut stack: usize = 0;
                let mut test_pos: usize = robot_pos.0 - 1;
                loop {
                    match map[robot_pos.1][test_pos] {
                        '#' => {
                            // Step back one step
                            test_pos += 1;
                            break;
                        }
                        '.' => {
                            break;
                        }
                        'O' => {
                            map[robot_pos.1][test_pos] = '.';
                            stack += 1;
                            test_pos -= 1;
                        }
                        _ => {
                            panic!("Invalid character in map!");
                        }
                    }
                }
                for _ in 0..stack {
                    map[robot_pos.1][test_pos] = 'O';
                    test_pos += 1;
                }
                robot_pos.0 = test_pos; // Move the robot
            }
            '^' => {
                let mut stack: usize = 0;
                let mut test_pos: usize = robot_pos.1 - 1;
                loop {
                    match map[test_pos][robot_pos.0] {
                        '#' => {
                            // Step back one step
                            test_pos += 1;
                            break;
                        }
                        '.' => {
                            break;
                        }
                        'O' => {
                            map[test_pos][robot_pos.0] = '.';
                            stack += 1;
                            test_pos -= 1;
                        }
                        _ => {
                            panic!("Invalid character in map!");
                        }
                    }
                }
                for _ in 0..stack {
                    map[test_pos][robot_pos.0] = 'O';
                    test_pos += 1;
                }
                robot_pos.1 = test_pos; // Move the robot
            }
            '>' => {
                let mut stack: usize = 0;
                let mut test_pos: usize = robot_pos.0 + 1;
                loop {
                    match map[robot_pos.1][test_pos] {
                        '#' => {
                            // Step back one step
                            test_pos -= 1;
                            break;
                        }
                        '.' => {
                            break;
                        }
                        'O' => {
                            map[robot_pos.1][test_pos] = '.';
                            stack += 1;
                            test_pos += 1;
                        }
                        _ => {
                            panic!("Invalid character in map!");
                        }
                    }
                }
                for _ in 0..stack {
                    map[robot_pos.1][test_pos] = 'O';
                    test_pos -= 1;
                }
                robot_pos.0 = test_pos; // Move the robot
            }
            'v' => {
                let mut stack: usize = 0;
                let mut test_pos: usize = robot_pos.1 + 1;
                loop {
                    match map[test_pos][robot_pos.0] {
                        '#' => {
                            // Step back one step
                            test_pos -= 1;
                            break;
                        }
                        '.' => {
                            break;
                        }
                        'O' => {
                            map[test_pos][robot_pos.0] = '.';
                            stack += 1;
                            test_pos += 1;
                        }
                        _ => {
                            panic!("Invalid character in map!");
                        }
                    }
                }
                for _ in 0..stack {
                    map[test_pos][robot_pos.0] = 'O';
                    test_pos -= 1;
                }
                robot_pos.1 = test_pos; // Move the robot
            }
            _ => {
                panic!("Invalid move character in move input: '{mv}' was passed.")
            }
        }
    }

    map[robot_pos.1][robot_pos.0] = '@';
    map
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_single_move_left_blocked() {
        let input = r"
########
#@.....#
########

<
        ";

        let solution = r"
########
#@.....#
########
        ";

        let map;
        let moves;
        (map, moves) = parse_input(input);

        let result = solve_map(map, moves);
        let solution = parse_input(solution).0;
        assert_eq!(result, solution);
    }

    #[test]
    pub fn test_single_move_left() {
        let input = r"
########
#....@.#
########

<
        ";

        let solution = r"
########
#...@..#
########
        ";

        let map;
        let moves;
        (map, moves) = parse_input(input);

        let result = solve_map(map, moves);
        let solution = parse_input(solution).0;
        assert_eq!(result, solution);
    }

    #[test]
    pub fn test_double_move_left() {
        let input = r"
########
#....@.#
########

<<
        ";

        let solution = r"
########
#..@...#
########
        ";

        let map;
        let moves;
        (map, moves) = parse_input(input);

        let result = solve_map(map, moves);
        let solution = parse_input(solution).0;
        assert_eq!(result, solution);
    }

    #[test]
    pub fn test_single_move_left_boulder() {
        let input = r"
########
#...O@.#
########

<
        ";

        let solution = r"
########
#..O@..#
########
        ";

        let map;
        let moves;
        (map, moves) = parse_input(input);

        let result = solve_map(map, moves);
        let solution = parse_input(solution).0;
        assert_eq!(result, solution);
    }

    pub fn test_single_move_left_2boulder() {
        let input = r"
########
#..OO@.#
########

<
        ";

        let solution = r"
########
#.OO@..#
########
        ";

        let map;
        let moves;
        (map, moves) = parse_input(input);

        let result = solve_map(map, moves);
        let solution = parse_input(solution).0;
        assert_eq!(result, solution);
    }

    pub fn test_single_move_left_3boulder() {
        let input = r"
########
#.OOO@.#
########

<
        ";

        let solution = r"
########
#OOO@..#
########
        ";

        let map;
        let moves;
        (map, moves) = parse_input(input);

        let result = solve_map(map, moves);
        let solution = parse_input(solution).0;
        assert_eq!(result, solution);
    }

    pub fn test_single_move_left_4boulder_blocked() {
        let input = r"
########
#OOOO@.#
########

<
        ";

        let solution = r"
########
#OOOO@.#
########
        ";

        let map;
        let moves;
        (map, moves) = parse_input(input);

        let result = solve_map(map, moves);
        let solution = parse_input(solution).0;
        assert_eq!(result, solution);
    }

    #[test]
    pub fn test_many_complex_moves_left() {
        let input = r"
############
#O..O..O.@.#
############

<<<<<<<<<<<<<<<<<<<
        ";

        let solution = r"
############
#OOO@......#
############
        ";

        let map;
        let moves;
        (map, moves) = parse_input(input);

        let result = solve_map(map, moves);
        let solution = parse_input(solution).0;

        println!("Input map:");
        print_map(parse_input(input).0);
        println!("Result map:");
        print_map(result.clone());
        println!("Solution map:");
        print_map(solution.clone());
        assert_eq!(result, solution);
    }

    #[test]
    pub fn test_many_complex_moves_right() {
        let input = r"
############
#O@.O..O.O.#
############

>>>>>>>>>>>>>>>>
        ";

        let solution = r"
############
#O.....@OOO#
############
        ";

        let map;
        let moves;
        (map, moves) = parse_input(input);

        let result = solve_map(map, moves);
        let solution = parse_input(solution).0;

        println!("Input map:");
        print_map(parse_input(input).0);
        println!("Result map:");
        print_map(result.clone());
        println!("Solution map:");
        print_map(solution.clone());
        assert_eq!(result, solution);
    }

    #[test]
    pub fn test_solve_map_small_input() {
        let input = r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

        let solution = r"
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########
        ";

        let map;
        let moves;
        (map, moves) = parse_input(input);

        let result = solve_map(map, moves);
        let solution = parse_input(solution).0;

        println!("Result map:");
        print_map(result.clone());
        println!("Solution map:");
        print_map(solution.clone());
        assert_eq!(result, solution);
    }

    #[test]
    pub fn test_solve_map_large_input() {
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

        let solution = r"
##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
        ";

        let map;
        let moves;
        (map, moves) = parse_input(input);

        let result = solve_map(map, moves);
        let solution = parse_input(solution).0;

        println!("Result map:");
        print_map(result.clone());
        println!("Solution map:");
        print_map(solution.clone());
        assert_eq!(result, solution);
    }
}
