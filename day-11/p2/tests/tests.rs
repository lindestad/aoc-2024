use p2::*;

// Check that old tests still hold
#[test]
fn test_update_stones() {
    assert_eq!(update_stones("125 17"), "253000 1 7");
}

#[test]
fn test_blink1() {
    assert_eq!(blink("125 17", 1), "253000 1 7");
}

#[test]
fn test_blink2() {
    assert_eq!(blink("125 17", 2), "253 0 2024 14168");
}

#[test]
fn test_blink3() {
    assert_eq!(blink("125 17", 3), "512072 1 20 24 28676032");
}

#[test]
fn test_blink4() {
    assert_eq!(blink("125 17", 4), "512 72 2024 2 0 2 4 2867 6032");
}

#[test]
fn test_blink5() {
    assert_eq!(
        blink("125 17", 5),
        "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32"
    );
}

#[test]
fn test_blink6() {
    assert_eq!(
        blink("125 17", 6),
        "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"
    );
}

// New tests
#[test]
fn test_calculatestone_0_15() {
    let stones = vec![0];
    let blink_count = 15;

    let old_result = count_stones(&blink("0", blink_count));
    let total_stones = calculatestones(&stones, blink_count);
    assert_eq!(old_result, total_stones as usize);
}

#[test]
fn test_calculatestone_0_30() {
    let stones = vec![0];
    let blink_count = 30;

    let old_result = count_stones(&blink("0", blink_count));
    let total_stones = calculatestones(&stones, blink_count);
    assert_eq!(old_result, total_stones as usize);
}

#[test]
fn test_calculatestone_0_35() {
    let stones = vec![0];
    let blink_count = 35;

    let old_result = count_stones(&blink("0", blink_count));
    let total_stones = calculatestones(&stones, blink_count);
    assert_eq!(old_result, total_stones as usize);
}

#[test]
fn test_calculatestone_7_15() {
    let stones = vec![7];
    let blink_count = 15;

    let old_result = count_stones(&blink("7", blink_count));
    let total_stones = calculatestones(&stones, blink_count);
    assert_eq!(old_result, total_stones as usize);
}

#[test]
fn test_calculatestone_7_30() {
    let stones = vec![7];
    let blink_count = 30;

    let old_result = count_stones(&blink("7", blink_count));
    let total_stones = calculatestones(&stones, blink_count);
    assert_eq!(old_result, total_stones as usize);
}

#[test]
fn test_calculatestone_7_35() {
    let stones = vec![7];
    let blink_count = 35;

    let old_result = count_stones(&blink("7", blink_count));
    let total_stones = calculatestones(&stones, blink_count);
    assert_eq!(old_result, total_stones as usize);
}

#[test]
fn test_calculatestone_382091382_15() {
    let stones = vec![382091382];
    let blink_count = 15;

    let old_result = count_stones(&blink("382091382", blink_count));
    let total_stones = calculatestones(&stones, blink_count);
    assert_eq!(old_result, total_stones as usize);
}
