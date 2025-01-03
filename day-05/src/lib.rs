pub fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut pairs: Vec<(i32, i32)> = Vec::new();
    let mut values: Vec<Vec<i32>> = Vec::new();

    let mut lines = input.lines();

    // Collect pairs until the blank line
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }

        let parts: Vec<&str> = trimmed.split('|').collect();
        if parts.len() == 2 {
            if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                pairs.push((a, b));
            }
        }
    }

    // Collect the remaining comma-separated values
    while let Some(line) = lines.next() {
        if line == "" {
            continue;
        }
        values.push(
            line.split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect(),
        );
    }

    return (pairs, values);
}

pub fn check_validity(pairs: &[(i32, i32)], values: &[i32]) -> bool {
    if values.len() < 2 {
        return true;
    }
    for (a, b) in pairs {
        let a_index = values.iter().position(|&x| x == *a);
        let b_index = values.iter().position(|&x| x == *b);

        match (a_index, b_index) {
            (Some(a_index), Some(b_index)) => {
                if a_index > b_index {
                    return false;
                }
            }
            _ => {}
        }
    }
    return true;
}

pub fn return_middle_if_valid(pairs: &[(i32, i32)], values: &[i32]) -> Option<i32> {
    if check_validity(pairs, values) {
        let middle_index = values.len() / 2;
        return Some(values[middle_index]);
    }
    return None;
}

pub fn sort_values(pairs: &[(i32, i32)], values: &mut Vec<i32>) {
    if values.len() < 2 {
        return;
    }
    while check_validity(pairs, values) == false {
        for (a, b) in pairs {
            let a_index = values.iter().position(|&x| x == *a);
            let b_index = values.iter().position(|&x| x == *b);

            match (a_index, b_index) {
                (Some(a_index), Some(b_index)) => {
                    if a_index > b_index {
                        values.swap(a_index, b_index);
                    }
                }
                _ => {}
            }
        }
    }
    return;
}
