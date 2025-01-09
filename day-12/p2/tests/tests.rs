use p2::*;
use std::collections::HashSet;

#[test]
fn test_example_1() {
    let input = "AAAA\nBBCD\nBBCC\nEEEC\n\r";
    let solution = 80;
    assert_eq!(calculate_price(input), solution);
}

#[test]
fn test_example_2() {
    let input = "EEEEEn\nEXXXX\nEEEEE\nEXXXX\nEEEEE";
    let solution = 236;
    assert_eq!(calculate_price(input), solution);
}

#[test]
fn test_example_3() {
    let input = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";
    let solution = 1206;
    assert_eq!(calculate_price(input), solution);
}

#[test]
fn test_example_4() {
    let input = "AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA\n";
    let solution = 368;
    assert_eq!(calculate_price(input), solution);
}

#[test]
fn test_single_cell_region() {
    // A single 'A' cell. Area=1.
    // It's just one square, so its boundary is a 1x1 bounding box = 4 edges.
    // But as a single rectangle, those 4 edges form exactly 4 sides (no collinearity merges).
    // Under Part Two logic: cost = area (1) * sides (4) = 4.
    let input = "A";
    let grid = str_to_vec(input);
    let mut visited = HashSet::new();
    let cost = calculate_single_price(0, 0, &grid, &mut visited);
    assert_eq!(cost, 4, "Expected 4 for a 1x1 region");
}

#[test]
fn test_2x2_all_a() {
    // 2x2 block of 'A':
    // AA
    // AA
    //
    // The area is 4.
    // It's one big 2-wide-by-2-tall rectangle, which has 4 sides total:
    //   - top horizontal
    //   - bottom horizontal
    //   - left vertical
    //   - right vertical
    // So cost = 4 (area) * 4 (sides) = 16.
    let input = "\
AA
AA
";
    let grid = str_to_vec(input);
    let mut visited = HashSet::new();
    let cost = calculate_single_price(0, 0, &grid, &mut visited);
    assert_eq!(cost, 16, "Expected 16 for a 2x2 region");
}

#[test]
fn test_3x1_row_of_a() {
    // A single row of 'A': "AAA"
    //
    // The area is 3.
    // As a 3-wide-by-1-tall rectangle, it has 4 sides total:
    //   - top horizontal
    //   - bottom horizontal
    //   - left vertical
    //   - right vertical
    // So cost = 3 (area) * 4 (sides) = 12.
    let input = "AAA";
    let grid = str_to_vec(input);
    let mut visited = HashSet::new();
    let cost = calculate_single_price(0, 0, &grid, &mut visited);
    assert_eq!(cost, 12, "Expected 12 for a 3x1 region");
}

#[test]
fn test_small_with_two_regions() {
    // A 2x2 grid but with half 'A' and half 'B', like:
    // AB
    // AB
    //
    // Region 1 (all 'A'): (0,0) and (0,1).
    //   - That's a 2-tall-by-1-wide rectangle => area=2, sides=4 => cost=8.
    // Region 2 (all 'B'): (1,0) and (1,1).
    //   - Same shape => area=2, sides=4 => cost=8.
    //
    // If we call calculate_single_price(0,0), we only get Region 1’s cost = 8.
    // But that’s correct for that one region. So we expect 8 from that call.
    let input = "\
AB
AB
";
    let grid = str_to_vec(input);
    let mut visited = HashSet::new();
    // We'll compute the region that starts at (0,0) => the 'A' region
    let cost_a_region = calculate_single_price(0, 0, &grid, &mut visited);
    assert_eq!(
        cost_a_region, 8,
        "Expected 8 for the 'A' region in a 2x2 split"
    );

    // Next, if we want to test the 'B' region in isolation, do the same from (1,0).
    let cost_b_region = calculate_single_price(1, 0, &grid, &mut visited);
    assert_eq!(
        cost_b_region, 8,
        "Expected 8 for the 'B' region in a 2x2 split"
    );
}

#[test]
fn test_single_area() {
    let input = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";
    let grid = str_to_vec(input);
    let mut visited = HashSet::new();
    let (x, y) = (1, 1);
    let solution = 12;
    assert_eq!(find_single_area(x, y, &grid, &mut visited), solution);
}
