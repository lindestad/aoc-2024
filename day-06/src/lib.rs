pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        result.push(row);
    }
    return result;
}

pub fn extract_guard_pos(grid: &Vec<Vec<char>>) -> (usize, usize, char) {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '^' || *c == 'v' || *c == '<' || *c == '>' {
                return (x, y, *c);
            }
        }
    }
    panic!("No guard found in grid");
}

pub fn clear_guard_from_grid(grid: &mut Vec<Vec<char>>) {
    for row in grid.iter_mut() {
        for c in row.iter_mut() {
            if *c == '^' || *c == 'v' || *c == '<' || *c == '>' {
                *c = '.';
            }
        }
    }
}

fn check_if_in_bounds((x, y): (usize, usize), (width, height): (usize, usize)) -> bool {
    return x < width && y < height;
}

// returns the new position and direction of the guard
// if out of bounds, returns (0, 0, 'Q') - indicating the serach should stop
pub fn next_guard_position(
    grid: &Vec<Vec<char>>,
    (x, y): (usize, usize),
    direction: char,
) -> (usize, usize, char) {
    let (grid_width, grid_height) = (grid[0].len(), grid.len());
    let mut new_x: usize;
    let mut new_y: usize;
    let mut new_direction: char = direction;
    for _ in 0..4 {
        (new_x, new_y) = (x, y);
        // handle edge cases caused by clamping to usize
        if (new_x == 0 && new_direction == '<') || (new_y == 0 && new_direction == '^') {
            return (0, 0, 'Q');
        }
        match new_direction {
            '^' => new_y = new_y.saturating_sub(1), // avoid usize underflow
            'v' => new_y += 1,
            '<' => new_x = new_x.saturating_sub(1), // avoid usize underflow
            '>' => new_x += 1,
            _ => panic!("Invalid direction"),
        }
        // Check if the new position is out of bounds
        if !check_if_in_bounds((new_x, new_y), (grid_width, grid_height)) {
            return (0, 0, 'Q');
        }
        if grid[new_y][new_x] == '.' {
            return (new_x, new_y, new_direction);
        } else if grid[new_y][new_x] == '#' {
            match new_direction {
                '^' => new_direction = '>',
                '>' => new_direction = 'v',
                'v' => new_direction = '<',
                '<' => new_direction = '^',
                _ => panic!("Invalid direction"),
            }
        }
    }
    panic!("Rotated four times without finding a valid direction");
}
