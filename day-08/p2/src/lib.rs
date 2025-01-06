use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn input_to_vec(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    let mut row = Vec::new();
    for c in input.chars() {
        if c == '\n' {
            grid.push(row);
            row = Vec::new();
        } else if c != '\r' {
            row.push(c);
        }
    }
    if !row.is_empty() {
        grid.push(row);
    }
    return grid;
}

pub fn find_pairs(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut pairs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '.' {
                continue;
            }
            let mut pair = Vec::new();
            pair.push((x, y));

            // Insert or update the HashMap
            pairs.entry(c).or_insert(Vec::new()).extend(pair);
        }
    }
    return pairs;
}

fn check_inbounds(x: i32, y: i32, width: usize, height: usize) -> bool {
    return x >= 0 && x < width as i32 && y >= 0 && y < height as i32;
}

pub fn find_antinodes_for_pair(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    width: usize,
    height: usize,
) -> Vec<(i32, i32)> {
    let dx = x2 - x1;
    let dy = y2 - y1;

    if dx == 0 && dy == 0 {
        return vec![];
    }

    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    // positive
    let mut i = 0;
    loop {
        let x = x1 + dx * i;
        let y = y1 + dy * i;
        if !check_inbounds(x, y, width, height) {
            break;
        }
        antinodes.push((x, y));
        i += 1;
    }
    // negative
    let mut i = 1;
    loop {
        let x = x1 - dx * i;
        let y = y1 - dy * i;
        if !check_inbounds(x, y, width, height) {
            break;
        }
        antinodes.push((x, y));
        i += 1;
    }
    return antinodes;
}

pub fn find_unique_antinodes(grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let pairs = find_pairs(grid);

    let width = grid[0].len();
    let height = grid.len();

    for (_freq, positions) in &pairs {
        for combo in positions.iter().combinations(2) {
            let (x1, y1) = *combo[0];
            let (x2, y2) = *combo[1];

            let antinode_points =
                find_antinodes_for_pair(x1 as i32, y1 as i32, x2 as i32, y2 as i32, width, height);

            for (ax, ay) in antinode_points {
                antinodes.insert((ax as usize, ay as usize));
            }
        }
    }

    return antinodes;
}
