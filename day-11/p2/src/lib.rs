use std::collections::{HashMap, HashSet};

// Old approach
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
        let mut largest_stone = 0;
        stones.split(" ").for_each(|stone| {
            let stone_num = stone.parse::<u64>().unwrap();
            if stone_num > largest_stone {
                largest_stone = stone_num;
            }
        });
        stones = update_stones(&stones);
    }
    stones
}

pub fn count_stones(stones: &str) -> usize {
    stones.split(" ").count()
}

// New approach

/// Count how many digits (base-10) a nonnegative integer has.
fn digit_count(mut x: u64) -> usize {
    if x == 0 {
        return 1;
    }
    let mut count = 0;
    while x > 0 {
        x /= 10;
        count += 1;
    }
    count
}

/// Split an even-digit number into two halves (string-based approach).
fn split_stone(x: u64) -> (u64, u64) {
    let s = x.to_string();
    let half = s.len() / 2;
    let left_str = &s[..half];
    let right_str = &s[half..];
    let left_num = left_str.parse::<u64>().unwrap();
    let right_num = right_str.parse::<u64>().unwrap();
    (left_num, right_num)
}

/// Apply exactly one "blink" / transformation step to a single stone,
/// returning the child stones.
fn next_stones(n: u64) -> Vec<u64> {
    if n == 0 {
        // rule #1
        vec![1]
    } else {
        let dcount = digit_count(n);
        if dcount % 2 == 0 {
            // rule #2 (even digit count): split
            let (l, r) = split_stone(n);
            vec![l, r]
        } else {
            // rule #3 (odd digit count, nonzero): multiply
            vec![n.saturating_mul(2024)]
        }
    }
}

/// Recursively compute how many stones come from stone `n` after `t` blinks.
/// Uses memoization + cycle detection to avoid re-expanding the same subproblems.
fn expand(
    n: u64,
    t: usize,
    memo: &mut HashMap<(u64, usize), u64>,
    in_progress: &mut HashSet<(u64, usize)>,
) -> u64 {
    // Base case
    if t == 0 {
        return 1;
    }

    // If it's already in memo, return cached value.
    if let Some(&cached) = memo.get(&(n, t)) {
        return cached;
    }

    // Cycle detection: if we revisit (n, t) before finishing,
    // we've detected a cycle. Does not seem to occur so left as a panic.
    if !in_progress.insert((n, t)) {
        panic!("Cycle detected at stone = {n}, t = {t} -- implement cycle skipping here if this ever panics.");
    }

    // Expand to child stones, then sum their expansions.
    let children = next_stones(n);
    let mut sum = 0;
    for &child in &children {
        sum += expand(child, t - 1, memo, in_progress);
    }

    // Mark (n,t) done; store in memo and remove from "in progress"
    memo.insert((n, t), sum);
    in_progress.remove(&(n, t));

    sum
}

/// Public function to compute how many stones you'll have after `blink_count` blinks,
/// starting from an initial collection of stones.

pub fn calculatestones(stones: &[u64], blink_count: usize) -> u64 {
    let mut memo = HashMap::<(u64, usize), u64>::new();
    let mut in_progress = HashSet::<(u64, usize)>::new();

    let mut total = 0;
    for &initial_stone in stones {
        total += expand(initial_stone, blink_count, &mut memo, &mut in_progress);
    }
    total
}
