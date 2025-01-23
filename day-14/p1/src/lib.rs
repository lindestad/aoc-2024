use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
