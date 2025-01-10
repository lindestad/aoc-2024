use p2::*;
use std::collections::HashSet;

#[test]
fn test_corner_count1() {
    let input = "\
AAAA
";
    let grid = str_to_vec(input);
    let solution = 4;
    let calculated = count_corners(&grid);
    assert_eq!(calculated, solution);
}

#[test]
fn test_corner_count2() {
    let input = "\
AAAA
AAAA
";
    let grid = str_to_vec(input);
    let solution = 4;
    let calculated = count_corners(&grid);
    assert_eq!(calculated, solution);
}

#[test]
fn test_corner_count3() {
    let input = "\
AAAB
";
    let grid = str_to_vec(input);
    let solution = 8;
    let calculated = count_corners(&grid);
    assert_eq!(calculated, solution);
}

#[test]
fn test_corner_count4() {
    let input = "\
AAAB
AAAB
";
    let grid = str_to_vec(input);
    let solution = 8;
    let calculated = count_corners(&grid);
    assert_eq!(calculated, solution);
}

#[test]
fn test_corner_count5() {
    let input = "\
AA
AA
AA
BB
";
    let grid = str_to_vec(input);
    let solution = 8;
    let calculated = count_corners(&grid);
    assert_eq!(calculated, solution);
}

#[test]
fn test_corner_count6() {
    let input = "\
AAAA
AAAA
AAAA
BBAA
";
    let grid = str_to_vec(input);
    let solution = 10;
    let calculated = count_corners(&grid);
    assert_eq!(calculated, solution);
}

#[test]
fn test_corner_count7() {
    let input = "\
AAAA
ABAA
ABAA
AAAA
";
    let grid = str_to_vec(input);
    let solution = 12;
    let calculated = count_corners(&grid);
    assert_eq!(calculated, solution);
}
