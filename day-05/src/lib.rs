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
