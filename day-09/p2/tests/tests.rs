use p2::*;
use std::collections::HashMap;

#[test]
fn test_checksum() {
    let input = "2333133121414131402";
    let output = calculate_checksum(&input);
    let expected_output = 2858;
    println!("output:\n{}", output);
    println!("expected_output:\n{}", expected_output);
    assert_eq!(output, expected_output);
}

#[test]
fn test_move_blocks_left2() {
    let input = "2333133121414131402";
    let disk_map = str_to_vec_u8(&input);
    let mut blocks = disk_map_to_blocks(disk_map);
    move_blocks_left2(&mut blocks);
    let expected_output = "00992111777.44.333....5555.6666.....8888..";
    let output = blocks_to_str(blocks);
    dbg!(&output);
    dbg!(&expected_output);
    println!("output:\n{}", output);
    println!("expected_output:\n{}", expected_output);
    assert_eq!(output, expected_output);
}

#[test]
fn test_find_windows() {
    let input_str = "00...111...2...333.44.5555.6666.777.888899";
    let mut input = str_to_vec_u64(&input_str);
    strip_period(&mut input);
    println!("input:");
    for line in input.iter() {
        println!("{}", line);
    }
    let output = find_windows(&mut input);
    let expected_output: HashMap<u64, (usize, usize)> = HashMap::from_iter(vec![
        (48, (0, 1)),
        (49, (5, 7)),
        (50, (11, 11)),
        (51, (15, 17)),
        (52, (19, 20)),
        (53, (22, 25)),
        (54, (27, 30)),
        (55, (32, 34)),
        (56, (36, 39)),
        (57, (40, 41)),
    ]);
    println!("output:");
    for line in expected_output.iter() {
        println!("key: {}, start: {}, end: {}", line.0, line.1 .0, line.1 .1);
    }

    println!("output:\n{:?}", output);
    println!("expected_output:\n{:?}", expected_output);
    assert_eq!(output, expected_output);
}

#[test]
fn test_find_first_slot1() {
    let input_str = "00...111...2...333.44.5555.6666.777.888899";
    let mut input = str_to_vec_u64(&input_str);
    strip_period(&mut input);
    let output = find_first_slot(&input, 3);
    let expected_output = Some((2, 4));
    println!("output:\n{:?}", output);
    println!("expected_output:\n{:?}", expected_output);
    assert_eq!(output, expected_output);
}
