use regex::Regex;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new<T: Into<i64>>(x: T, y: T) -> Point {
        Point {
            x: x.into(),
            y: y.into(),
        }
    }
}

pub fn parse_input(input: &str) -> Vec<(Point, Point)> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    re.captures_iter(input)
        .filter_map(|cap| {
            let px = i64::from_str(&cap[1]).ok()?;
            let py = i64::from_str(&cap[2]).ok()?;
            let vx = i64::from_str(&cap[3]).ok()?;
            let vy = i64::from_str(&cap[4]).ok()?;

            Some((Point::new(px, py), Point::new(vx, vy)))
        })
        .collect()
}

pub fn find_robot_position<T: Into<i64>>(
    start: Point,
    velocity: Point,
    width: T,
    height: T,
    delta_time: T,
) -> Point {
    let width = width.into();
    let height = height.into();
    let delta_time = delta_time.into();
    Point::new(
        (start.x + velocity.x * delta_time).rem_euclid(width),
        (start.y + velocity.y * delta_time).rem_euclid(height),
    )
}

pub fn find_safety_factor<T: Into<i64>>(points: &[Point], width: T, height: T) -> i64 {
    let width = width.into();
    let height = height.into();

    let mut quadrant_counts: [i32; 4] = [0; 4];

    for point in points {
        let x = point.x;
        let y = point.y;

        // Skip the center of the grid
        if x == width / 2 || y == height / 2 {
            continue;
        }

        match (x >= width / 2, y >= height / 2) {
            (true, true) => quadrant_counts[0] += 1,   // Quadrant 1
            (false, true) => quadrant_counts[1] += 1,  // Quadrant 2
            (false, false) => quadrant_counts[2] += 1, // Quadrant 3
            (true, false) => quadrant_counts[3] += 1,  // Quadrant 4
        }
    }
    quadrant_counts.iter().fold(1, |prod, &x| prod * x) as i64
}

pub fn print_robots(points: &[Point], width: i64, height: i64) {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];

    for point in points {
        grid[point.y as usize][point.x as usize] = '#';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

pub fn max_num_points_collinear(points: &[Point]) -> i64 {
    if points.len() < 2 {
        return points.len() as i64;
    }

    let mut max_count = 1;

    for i in 0..points.len() {
        let mut slopes: HashMap<(i64, i64), i32> = HashMap::new();
        let mut duplicate = 1;
        let mut local_max = 0;

        for j in 0..points.len() {
            if i == j {
                continue;
            }

            let dx = points[j].x - points[i].x;
            let dy = points[j].y - points[i].y;

            if dx == 0 && dy == 0 {
                duplicate += 1; // Duplicate points don't change the slope
                continue;
            }

            let gcd = gcd(dx, dy);
            let slope = (dx / gcd, dy / gcd); // Normalize slope by GCD

            let count = slopes.entry(slope).or_insert(0);
            *count += 1;
            local_max = local_max.max(*count);
        }

        max_count = max_count.max(local_max + duplicate);
    }

    max_count as i64
}

//  Greatest Common Divisor (GCD)
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

pub fn max_col_row(points: &[Point], width: i64, height: i64) -> i64 {
    let mut max_x_count = 0;
    let mut max_y_count = 0;

    let mut cols = vec![0; width as usize];
    let mut rows = vec![0; height as usize];

    for point in points {
        if point.x >= 0 && point.x < width && point.y >= 0 && point.y < height {
            cols[point.x as usize] += 1;
            rows[point.y as usize] += 1;
        }
    }

    // Find max column count
    for &col_count in &cols {
        if col_count > max_x_count {
            max_x_count = col_count;
        }
    }

    // Find max row count
    for &row_count in &rows {
        if row_count > max_y_count {
            max_y_count = row_count;
        }
    }

    // Return the highest number of points in a single row or column
    max_x_count.max(max_y_count) as i64
}

pub fn max_consecutive_col_row(points: &[Point], width: i64, height: i64) -> i64 {
    let mut max_row_streak = 0;
    let mut max_col_streak = 0;

    // Sort points by X first, then by Y
    let mut sorted_by_x = points.to_vec();
    sorted_by_x.sort_by_key(|p| (p.y, p.x)); // Sorting ensures contiguous check

    let mut sorted_by_y = points.to_vec();
    sorted_by_y.sort_by_key(|p| (p.x, p.y));

    // Find the longest contiguous row streak
    let mut current_row_streak = 1;
    for i in 1..sorted_by_x.len() {
        if sorted_by_x[i].y == sorted_by_x[i - 1].y && sorted_by_x[i].x == sorted_by_x[i - 1].x + 1
        {
            current_row_streak += 1;
        } else {
            max_row_streak = max_row_streak.max(current_row_streak);
            current_row_streak = 1;
        }
    }
    max_row_streak = max_row_streak.max(current_row_streak); // Final check

    // Find the longest contiguous column streak
    let mut current_col_streak = 1;
    for i in 1..sorted_by_y.len() {
        if sorted_by_y[i].x == sorted_by_y[i - 1].x && sorted_by_y[i].y == sorted_by_y[i - 1].y + 1
        {
            current_col_streak += 1;
        } else {
            max_col_streak = max_col_streak.max(current_col_streak);
            current_col_streak = 1;
        }
    }
    max_col_streak = max_col_streak.max(current_col_streak); // Final check

    // Return the longest sequence found
    max_row_streak.max(max_col_streak) as i64
}
