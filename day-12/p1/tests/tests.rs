use p1::*;

#[test]
fn test_example_1() {
    let input = "AAAA\nBBCD\nBBCC\nEEEC\n\r";
    let solution = 140;
    assert_eq!(calculate_price(input), solution);
}

#[test]
fn test_example_2() {
    let input = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO";
    let solution = 772;
    assert_eq!(calculate_price(input), solution);
}

#[test]
fn test_example_3() {
    let input = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";
    let solution = 1930;
    assert_eq!(calculate_price(input), solution);
}
