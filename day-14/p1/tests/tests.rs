use p1::*;

#[test]
fn test_parse_input() {
    let input = "p=1,2 v=3,4\np=5,6 v=7,8";
    let expected = vec![
        (Point::new(1, 2), Point::new(3, 4)),
        (Point::new(5, 6), Point::new(7, 8)),
    ];
    assert_eq!(parse_input(input), expected);
}

#[test]
fn test_find_robot_position() {
    let start = Point::new(1, 2);
    let velocity = Point::new(3, 4);
    let width = 10;
    let height = 10;
    let delta_time = 1;
    let expected = Point::new(4, 6);
    assert_eq!(
        find_robot_position(start, velocity, width, height, delta_time),
        expected
    );
}

#[test]
fn test_find_robot_position_wraps() {
    let start = Point::new(9, 9);
    let velocity = Point::new(1, 1);
    let width = 10;
    let height = 10;
    let delta_time = 1;
    let expected = Point::new(0, 0);
    assert_eq!(
        find_robot_position(start, velocity, width, height, delta_time),
        expected
    );
}

#[test]
fn test_find_robot_position_wraps_multiple() {
    let start = Point::new(9, 9);
    let velocity = Point::new(1, 1);
    let width = 10;
    let height = 10;
    let delta_time = 10;
    let expected = Point::new(9, 9);
    assert_eq!(
        find_robot_position(start, velocity, width, height, delta_time),
        expected
    );
}

#[test]
fn test_find_safety_factor() {
    let input = r"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
    let points = parse_input(input);
    let width = 11;
    let height = 7;
    let delta_time = 100;
    let guard_positions = points
        .iter()
        .map(|(start, velocity)| find_robot_position(*start, *velocity, width, height, delta_time))
        .collect::<Vec<_>>();
    let expected = 12;
    assert_eq!(
        find_safety_factor(&guard_positions, width, height),
        expected
    );
}
