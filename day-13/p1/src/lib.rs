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

pub fn solve(a: &Button, b: &Button, p: &Prize) -> usize {
    let mut steps = 0;
    let mut x = p.x as i32;
    let mut y = p.y as i32;
    let mut solutions: Vec<usize> = Vec::new();
    while (x >= 0) && (y >= 0) {
        if (x % b.x as i32 == 0) && (y % b.y as i32 == 0) && (x / b.x as i32 == y / b.y as i32) {
            if (((p.x as i32 - x) % a.x as i32) == 0) && (((p.y as i32 - y) % a.y as i32) == 0) {
                let press_a = steps;
                let press_b = x as usize / b.x;
                if press_a <= 100 && press_b <= 100 {
                    solutions.push(press_a * 3 + press_b);
                }
            }
        }
        steps += 1;
        x -= a.x as i32;
        y -= a.y as i32;
    }
    if solutions.len() > 0 {
        return *solutions.iter().min().unwrap();
    } else {
        return 0;
    }
}
