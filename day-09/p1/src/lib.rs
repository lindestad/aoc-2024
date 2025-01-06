pub fn str_to_vec_u8(input: &str) -> Vec<u8> {
    input
        .chars()
        .filter(|&c| c != '\n' && c != '\r')
        .map(|c| c as u8)
        .collect()
}

pub fn disk_map_to_blocks(disk_map: Vec<u8>) -> Vec<u64> {
    let mut blocks: Vec<u64> = Vec::new();
    for i in 0..disk_map.len() {
        let mut character: u64 = u64::MAX;
        if i % 2 == 0 {
            character = (i / 2) as u64;
        }
        let sec_length = disk_map[i] - 48;
        let mut block: Vec<u64> = Vec::new();
        for _ in 0..sec_length {
            block.push(character);
        }
        blocks.append(&mut block);
    }
    blocks
}

pub fn blocks_to_str(blocks: Vec<u64>) -> String {
    blocks
        .iter()
        .map(|&c| {
            if c == u64::MAX {
                '.'.to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

pub fn str_to_blocks_str(input: &str) -> String {
    let disk_map = str_to_vec_u8(input);
    blocks_to_str(disk_map_to_blocks(disk_map))
}

pub fn move_blocks_left(blocks: &mut Vec<u64>) {
    let mut left_piv = 0;
    let mut right_piv = blocks.len();

    while left_piv < right_piv {
        if blocks[left_piv] == u64::MAX {
            if blocks[right_piv - 1] != u64::MAX {
                blocks.swap(left_piv, right_piv - 1);
                left_piv += 1;
                right_piv -= 1;
            } else {
                right_piv -= 1;
            }
        } else {
            left_piv += 1;
        }
    }
}

pub fn calculate_checksum(input: &str) -> u64 {
    let disk_map = str_to_vec_u8(input);
    let mut blocks = disk_map_to_blocks(disk_map);
    move_blocks_left(&mut blocks);
    let mut sum: u64 = 0;
    for i in 0..blocks.len() {
        let block = blocks[i];
        if block != u64::MAX {
            sum += block * (i as u64);
        }
    }
    sum
}
