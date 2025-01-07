pub fn parse_input_to_grid(input: &str) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();
    let mut row = Vec::new();
    for c in input.chars() {
        if c == '\n' {
            grid.push(row);
            row = Vec::new();
        } else if c != '\r' {
            row.push(c as u8);
        }
    }
    if !row.is_empty() {
        grid.push(row);
    }
    grid
}

fn check_inbounds(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}

pub fn recursive_trail_search(grid: &[Vec<u8>], position: (usize, usize)) -> usize {
    let mut trail_count = 0;
    let directions = vec![
        (0, 1),  // up
        (1, 0),  // right
        (0, -1), // down
        (-1, 0), // left
    ];
    let grid_dimensions = (grid[0].len(), grid.len());
    for direction in directions {
        let new_position = (
            position.0 as i32 + direction.0,
            position.1 as i32 + direction.1,
        );
        if check_inbounds(
            new_position.0,
            new_position.1,
            grid_dimensions.0,
            grid_dimensions.1,
        ) {
            let new_position = (new_position.0 as usize, new_position.1 as usize);
            if grid[new_position.1][new_position.0] == grid[position.1][position.0] + 1 {
                if grid[new_position.1][new_position.0] == b'9' {
                    trail_count += 1;
                } else {
                    trail_count += recursive_trail_search(grid, new_position);
                }
            }
        }
    }
    trail_count
}

pub fn find_trailheads(grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == b'0' {
                trailheads.push((x, y));
            }
        }
    }
    trailheads
}

pub fn sum_trail_ratings(grid: &[Vec<u8>]) -> usize {
    let trailheads = find_trailheads(grid);
    let mut total_score = 0;

    for trailhead in trailheads {
        total_score += recursive_trail_search(grid, trailhead);
    }
    total_score
}
