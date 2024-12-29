use regex::Regex;

pub fn find_valid_mul_no_regex(input: &str) -> i32 {
    if !input.is_ascii() {
        panic!("Input is not ASCII");
    }

    let mut result = 0;

    input
        .as_bytes()
        .windows(4)
        .enumerate()
        .filter_map(|(i, window)| {
            if window == b"mul(" {
                let mut val_a = String::new();
                let mut val_b = String::new();
                let mut parsing_a = true;

                for offset in (i + 4).. {
                    if let Some(&next_char) = input.as_bytes().get(offset) {
                        if next_char.is_ascii_digit() {
                            if parsing_a {
                                val_a.push(next_char as char);
                            } else {
                                val_b.push(next_char as char);
                            }
                        } else if next_char == b',' && parsing_a {
                            parsing_a = false; // Switch to parsing second number
                        } else if next_char == b')' && !parsing_a {
                            break; // End of valid instruction
                        } else {
                            return None; // Invalid character, skip this window
                        }
                    } else {
                        return None; // Out of bounds, skip this window
                    }
                }

                if let (Ok(a), Ok(b)) = (val_a.parse::<i32>(), val_b.parse::<i32>()) {
                    return Some(a * b); // Valid mul(x, y), return the product
                }
            }
            None
        })
        .for_each(|val| result += val);

    return result;
}

pub fn find_valid_mul_regex(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;

    for cap in re.captures_iter(input) {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        result += a * b;
    }

    return result;
}

pub fn strip_dont_sections(input: &str) -> String {
    let mut result = String::new();
    let mut skip = false;
    let mut i = 0;
    while i < input.len() {
        if input[i..].starts_with("don't()") {
            skip = true;
            i += "don't()".len();
            result.push_str("don't()");
        } else if input[i..].starts_with("do()") {
            skip = false;
            i += "do()".len();
            result.push_str("do()");
        } else {
            if !skip {
                // Push the current character, can't index input[i] directly
                // because Rust doesn't allow indexing into strings as they are
                // not guaranteed to be valid UTF-8
                result.push(input[i..].chars().next().unwrap());
            }
            i += 1;
        }
    }

    return result;
}
