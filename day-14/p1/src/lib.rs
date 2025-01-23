use regex::Regex;
use std::str::FromStr;

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

pub fn find_robot_position()
