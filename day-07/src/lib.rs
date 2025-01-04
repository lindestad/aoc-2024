use std::collections::HashSet;

pub fn parse_input(input: &str) -> (Vec<i128>, Vec<Vec<i128>>) {
    let lines = input.lines();
    let mut keys: Vec<i128> = Vec::new();
    let mut values: Vec<Vec<i128>> = Vec::new();
    for line in lines {
        // input in for key: value1 value2 value3 ...
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(": ");
        let key = parts.next().unwrap().parse::<i128>().unwrap();
        keys.push(key);
        let values_str = parts.next().unwrap();
        let values_str = values_str.split(" ");
        let mut values_vec: Vec<i128> = Vec::new();
        for value in values_str {
            values_vec.push(value.parse::<i128>().unwrap());
        }
        values.push(values_vec);
    }
    return (keys, values);
}

pub fn can_make_target(target: i128, values: &[i128]) -> bool {
    if values.is_empty() {
        return false;
    }
    if values.len() == 1 {
        return values[0] == target;
    }

    let mut current_results = HashSet::new();
    current_results.insert(values[0]);

    for &val in &values[1..] {
        let mut next_results = HashSet::new();
        for &r in &current_results {
            // Only + and *
            next_results.insert(r.saturating_add(val));
            next_results.insert(r.saturating_mul(val));
        }
        current_results = next_results;
    }
    return current_results.contains(&target);
}
