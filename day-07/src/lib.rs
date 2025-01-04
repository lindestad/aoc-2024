use num_rational::Ratio;
use std::collections::HashMap;

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

pub fn can_make_sum_memoized(
    target: Ratio<i128>,
    values: &[Ratio<i128>],
    memo: &mut HashMap<(Ratio<i128>, Vec<Ratio<i128>>), bool>,
) -> bool {
    // Check the memoization cache
    if let Some(&result) = memo.get(&(target.clone(), values.to_vec())) {
        return result;
    }

    // Base cases
    if values.is_empty() {
        return false;
    }
    if values.len() == 1 {
        let result = values[0] == target;
        memo.insert((target.clone(), values.to_vec()), result);
        return result;
    }

    let (first, rest) = values.split_first().unwrap();

    // Recursive cases: Try all operations
    let mut result = can_make_sum_memoized(target.clone() - first.clone(), rest, memo)
        || can_make_sum_memoized(target.clone() + first.clone(), rest, memo)
        || can_make_sum_memoized(target.clone() * first.clone(), rest, memo);

    // Safe division: Avoid division by zero
    if *first != Ratio::new(0, 1) {
        result = result || can_make_sum_memoized(target.clone() / first.clone(), rest, memo);
    }

    // Store the result in the memoization cache
    memo.insert((target.clone(), values.to_vec()), result);
    result
}

pub fn can_make_sum(target: Ratio<i128>, values: Vec<Ratio<i128>>) -> bool {
    let mut memo = HashMap::new();
    can_make_sum_memoized(target, &values, &mut memo)
}
