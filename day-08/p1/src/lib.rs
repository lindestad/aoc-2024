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

fn find_pairs(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
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
    pairs
}

fn check_inbounds(x: i32, y: i32, width: usize, height: usize) -> bool {
    return x >= 0 && x < width as i32 && y >= 0 && y < height as i32;
}

fn find_antinodes_for_pair(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    width: Option<usize>,
    height: Option<usize>,
) -> Vec<(i32, i32)> {
    let dx = x2 - x1;
    let dy = y2 - y1;

    // If antennas coincide, no well-defined line or "twice as far" points
    if dx == 0 && dy == 0 {
        return vec![];
    }

    // Four t-values that come from the "AP = 2 BP or BP = 2 AP" ratio:
    let t_candidates: [f64; 4] = [2.0, -1.0, 1.0 / 3.0, 2.0 / 3.0];

    let mut antinodes = Vec::new();

    for &t in &t_candidates {
        // Because dx, dy, x1, y1 are integers, we only get integral coordinates
        // if t is an integer (like 2 or -1) or if dx,dy are multiples of 3 (for 1/3, 2/3).
        let (px, py) = if (t - 2.0).abs() < f64::EPSILON {
            // t == 2.0
            (x1 + 2 * dx, y1 + 2 * dy)
        } else if (t + 1.0).abs() < f64::EPSILON {
            // t == -1.0
            (x1 - dx, y1 - dy)
        } else if (t - (1.0 / 3.0)).abs() < f64::EPSILON {
            // t == 1/3
            if dx % 3 == 0 && dy % 3 == 0 {
                (x1 + dx / 3, y1 + dy / 3)
            } else {
                // Not an integer grid point -> skip
                continue;
            }
        } else if (t - (2.0 / 3.0)).abs() < f64::EPSILON {
            // t == 2/3
            // x1 + 2*dx/3, y1 + 2*dy/3 must be integer
            if (2 * dx) % 3 == 0 && (2 * dy) % 3 == 0 {
                (x1 + 2 * dx / 3, y1 + 2 * dy / 3)
            } else {
                // Not an integer grid point -> skip
                continue;
            }
        } else {
            // Should never happen with the array we used
            continue;
        };

        // Check bounds
        if let (Some(w), Some(h)) = (width, height) {
            if !check_inbounds(px, py, w, h) {
                // Out of grid
                continue;
            }
        }

        antinodes.push((px, py));
    }

    // Deduplicate, in case the same point arises from multiple t-values
    antinodes.sort_unstable();
    antinodes.dedup();

    return antinodes;
}

pub fn find_unique_antinodes(grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut antennas: HashSet<(usize, usize)> = HashSet::new();
    let pairs = find_pairs(grid);

    let width = grid[0].len();
    let height = grid.len();

    for (_freq, positions) in &pairs {
        for combo in positions.iter().combinations(2) {
            let (x1, y1) = *combo[0];
            let (x2, y2) = *combo[1];

            let antinode_points = find_antinodes_for_pair(
                x1 as i32,
                y1 as i32,
                x2 as i32,
                y2 as i32,
                Some(width),
                Some(height),
            );

            for (ax, ay) in antinode_points {
                antennas.insert((ax as usize, ay as usize));
            }
        }
    }

    return antennas;
}
