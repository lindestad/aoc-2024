use std::collections::HashMap;

pub fn str_to_vec_u8(input: &str) -> Vec<u8> {
    input
        .chars()
        .filter(|&c| c != '\n' && c != '\r')
        .map(|c| c as u8)
        .collect()
}

pub fn str_to_vec_u64(input: &str) -> Vec<u64> {
    input
        .chars()
        .filter(|&c| c != '\n' && c != '\r')
        .map(|c| c as u64)
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

fn swap_ranges(vec: &mut Vec<u64>, range1: std::ops::Range<usize>, range2: std::ops::Range<usize>) {
    assert_eq!(range1.len(), range2.len());
    for (i, j) in range1.zip(range2) {
        vec.swap(i, j);
    }
}

pub fn strip_period(blocks: &mut Vec<u64>) {
    for elem in blocks.iter_mut() {
        if *elem == '.' as u64 {
            *elem = u64::MAX;
        }
    }
}

pub fn find_windows(blocks: &[u64]) -> HashMap<u64, (usize, usize)> {
    let mut windows: HashMap<u64, (usize, usize)> = HashMap::new();
    let mut i: usize = 0;

    while i < blocks.len() {
        if blocks[i] == u64::MAX {
            i += 1; // Skip free space
            continue;
        }

        let key = blocks[i];
        let start = i; // Start of the window

        // Move `i` to the end of the current contiguous run of `key`
        while i < blocks.len() && blocks[i] == key {
            i += 1;
        }

        let end = i - 1; // End of the window

        windows.insert(key, (start, end));
    }

    windows
}

pub fn find_first_slot(blocks: &[u64], slot_size: usize) -> Option<(usize, usize)> {
    // Find first contiguous slot of size `slot_size`
    // Where the slot is empty (u64::MAX)
    let mut start = 0;
    let mut end = 0;
    let mut slot_found = false;

    for i in 0..blocks.len() {
        if blocks[i] == u64::MAX {
            if !slot_found {
                start = i;
                slot_found = true;
            }
            end = i;
        } else {
            if slot_found {
                if end - start + 1 >= slot_size {
                    return Some((start, start + slot_size - 1));
                }
                slot_found = false;
            }
        }
    }
    if slot_found && end - start + 1 >= slot_size {
        return Some((start, start + slot_size - 1));
    }

    None
}

pub fn move_blocks_left2(blocks: &mut Vec<u64>) {
    let windows = find_windows(&blocks);
    let mut keys: Vec<_> = windows.keys().collect();
    keys.sort_by(|a, b| b.cmp(a)); // Sort keys in descending order
    for key in keys {
        let window = windows.get(key).unwrap();
        if let Some(slot) = find_first_slot(&blocks, window.1 - window.0 + 1) {
            if slot.1 <= window.0 {
                swap_ranges(blocks, slot.0..slot.1 + 1, window.0..window.1 + 1)
            }
        } else {
            continue;
        };
    }
}

pub fn calculate_checksum(input: &str) -> u64 {
    let disk_map = str_to_vec_u8(input);
    let mut blocks = disk_map_to_blocks(disk_map);
    move_blocks_left2(&mut blocks);
    let mut sum: u64 = 0;
    for i in 0..blocks.len() {
        let block = blocks[i];
        if block != u64::MAX {
            sum += block * (i as u64);
        }
    }
    sum
}
