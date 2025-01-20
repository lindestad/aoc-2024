use itertools::izip;
use regex::Regex;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Button {
    x: usize,
    y: usize,
    name: char,
}

impl Button {
    pub fn new(x: usize, y: usize, name: char) -> Button {
        Button {
            x: x,
            y: y,
            name: name,
        }
    }
}

#[derive(Debug)]
pub struct Prize {
    x: usize,
    y: usize,
}

impl Prize {
    pub fn new(x: usize, y: usize) -> Prize {
        Prize { x: x, y: y }
    }
}

pub fn parse_input(input: &str) -> Vec<(Button, Button, Prize)> {
    let re = Regex::new(r"(?P<name>[^\r\n:]+): X[+=](?P<x>-?\d+), Y[+=](?P<y>-?\d+)").unwrap();

    let mut a_buttons: Vec<Button> = Vec::new();
    let mut b_buttons: Vec<Button> = Vec::new();
    let mut prizes: Vec<Prize> = Vec::new();

    for cap in re.captures_iter(input) {
        let name = cap["name"].to_string();
        let x: usize = cap["x"].parse().unwrap();
        let y: usize = cap["y"].parse().unwrap();

        match name.as_str() {
            "Button A" => a_buttons.push(Button::new(x, y, 'A')),
            "Button B" => b_buttons.push(Button::new(x, y, 'B')),
            "Prize" => prizes.push(Prize::new(x, y)),
            "" => continue,
            _ => panic!("Invalid name: {}", name),
        }
    }
    let mut problems: Vec<(Button, Button, Prize)> = Vec::new();
    for p in izip!(a_buttons, b_buttons, prizes) {
        problems.push(p);
    }
    problems
}

pub fn solve(a: &Button, b: &Button, p: &Prize) -> u64 {
    let p2_offset: i64 = 10000000000000;
    let ax = a.x as i64;
    let ay = a.y as i64;
    let bx = b.x as i64;
    let by = b.y as i64;
    let px = p.x as i64 + p2_offset;
    let py = p.y as i64 + p2_offset;

    let det = ax * by - ay * bx;

    // Determinant zero?
    if det == 0 {
        return 0; // No solution, or or many solutions, not in input set
    }

    let n = (px * by - py * bx) / det;
    let m = (ax * py - ay * px) / det;

    if (ax * n + bx * m, ay * n + by * m) == (px, py) {
        (n * 3 + m) as u64
    } else {
        0
    }
}
