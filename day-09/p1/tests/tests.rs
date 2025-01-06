use p1::*;

#[test]
fn test_disk_map_to_blocks1() {
    let input = "2333133121414131402";
    let output = str_to_blocks_str(&input);
    let expected_output = "00...111...2...333.44.5555.6666.777.888899";
    println!("output:\n{}", output);
    println!("expected_output:\n{}", expected_output);
    assert_eq!(output, expected_output);
}

#[test]
fn test_disk_map_to_blocks2() {
    let input = "12345";
    let output = str_to_blocks_str(&input);
    let expected_output = "0..111....22222";
    println!("output:\n{}", output);
    println!("expected_output:\n{}", expected_output);
    assert_eq!(output, expected_output);
}

#[test]
fn test_move_blocks_left() {
    let input = "2333133121414131402";
    let disk_map = str_to_vec_u8(&input);
    let mut blocks = disk_map_to_blocks(disk_map);
    move_blocks_left(&mut blocks);
    let expected_output = "0099811188827773336446555566..............";
    let output = blocks_to_str(blocks);
    println!("output:\n{}", output);
    println!("expected_output:\n{}", expected_output);
    assert_eq!(output, expected_output);
}

#[test]
fn test_checksum() {
    let input = "2333133121414131402";
    let output = calculate_checksum(&input);
    let expected_output = 1928;
    println!("output:\n{}", output);
    println!("expected_output:\n{}", expected_output);
    assert_eq!(output, expected_output);
}
