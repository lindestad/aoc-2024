use day_06::*;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");
    let mut grid: Vec<Vec<char>> = parse_input(&input);
    let (mut x, mut y, mut direction) = extract_guard_pos(&grid);
    clear_guard_from_grid(&mut grid);
    let mut guard_positions: HashSet<(usize, usize)> = HashSet::new();
    guard_positions.insert((x, y));
    loop {
        (x, y, direction) = next_guard_position(&grid, (x, y), direction);
        if direction == 'Q' {
            break;
        }
        guard_positions.insert((x, y));
    }
    println!("Day 06: Part 1");
    println!(
        "Number of unique guard positions: {}",
        guard_positions.len()
    );

    // Part 2
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input.txt");
    let mut grid: Vec<Vec<char>> = parse_input(&input);
    let (starting_x, starting_y, starting_direction) = extract_guard_pos(&grid);
    clear_guard_from_grid(&mut grid);
    let mut obstacle_positions: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..grid[0].len() {
        for j in 0..grid.len() {
            // Track unique guard positions and directions
            let mut guard_pos_and_permutation: HashSet<(usize, usize, char)> = HashSet::new();
            guard_pos_and_permutation.insert((x, y, direction));
            if grid[j][i] == '#' {
                continue;
            } else if ['^', 'v', '<', '>'].contains(&grid[j][i]) {
                continue;
            } else {
                grid[j][i] = '#'; // insert obstacle to test
            }
            let (mut x, mut y, mut direction) = (starting_x, starting_y, starting_direction);
            loop {
                (x, y, direction) = next_guard_position(&grid, (x, y), direction);
                if direction == 'Q' {
                    grid[j][i] = '.'; // remove obstacle
                    break;
                }
                if guard_pos_and_permutation.contains(&(x, y, direction)) {
                    obstacle_positions.insert((i, j));
                    grid[j][i] = '.'; // remove obstacle
                    break;
                }
                guard_pos_and_permutation.insert((x, y, direction));
            }
        }
    }
    println!("Day 06: Part 2");
    println!(
        "Number of unique guard positions with obstacles: {}",
        obstacle_positions.len()
    );
}
