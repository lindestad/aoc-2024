use std::collections::HashSet;

pub fn str_to_vec(input: &str) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            if c != '\r' && c != '\n' {
                row.push(c);
            }
        }
        if !row.is_empty() {
            output.push(row);
        }
    }
    output
}

fn check_inbounts(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

pub fn find_single_area(
    x: usize,
    y: usize,
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    let letter = grid[y][x];
    let mut area = 0;
    let mut stack = vec![(x, y)];
    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        area += 1;
        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if check_inbounts(new_x, new_y, grid[0].len() as i32, grid.len() as i32)
                && grid[new_y as usize][new_x as usize] == letter
            {
                stack.push((new_x as usize, new_y as usize));
            }
        }
    }
    area
}

pub fn calculate_single_price(
    x: usize,
    y: usize,
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> i32 {
    let letter = grid[y][x];
    let mut area = 0;
    let mut fence = 0;
    let mut stack = vec![(x, y)];
    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        area += 1;
        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if check_inbounts(new_x, new_y, grid[0].len() as i32, grid.len() as i32)
                && grid[new_y as usize][new_x as usize] == letter
            {
                stack.push((new_x as usize, new_y as usize));
            } else {
                fence += 1;
            }
        }
    }
    area * fence
}

pub fn calculate_price(input: &str) -> i32 {
    let grid = str_to_vec(input);
    let mut visited = HashSet::new();
    let mut price = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !visited.contains(&(x, y)) {
                price += calculate_single_price(x, y, &grid, &mut visited);
            }
        }
    }
    price
}
