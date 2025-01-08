pub fn update_stones(stones: &str) -> String {
    let stone_vec = stones.split(" ").collect::<Vec<&str>>();
    let mut new_stones = String::new();

    for stone in stone_vec {
        // Rule 1
        // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
        if stone == "0" {
            new_stones.push_str("1 ");
        }
        // Rule 2
        // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
        else if stone.len() % 2 == 0 {
            let half = stone.len() / 2;
            new_stones.push_str(&stone[..half]);
            new_stones.push_str(" ");
            // check if &stone[half..] has leading zeroes
            let right_half = &stone[half..];
            let trimmed_right_half = right_half.trim_start_matches('0');
            if trimmed_right_half.is_empty() {
                new_stones.push_str("0");
            } else {
                new_stones.push_str(trimmed_right_half);
            }
            new_stones.push_str(" ");
        }
        // Rule 3
        // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
        else {
            let new_stone = stone.parse::<u64>().unwrap() * 2024;
            new_stones.push_str(&new_stone.to_string());
            new_stones.push_str(" ");
        }
    }
    new_stones.pop(); // remove trailing space
    new_stones
}

pub fn blink(stones: &str, target_blinks: usize) -> String {
    let mut stones = stones.to_string();
    for _ in 0..target_blinks {
        stones = update_stones(&stones);
    }
    stones
}

pub fn count_stones(stones: &str) -> usize {
    stones.split(" ").count()
}
