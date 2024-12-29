use day_03::find_valid_mul_regex;

#[test]
fn test_valid_mul() {
    assert_eq!(find_valid_mul_regex("mul(2,3)"), 6);
    assert_eq!(find_valid_mul_regex("mul(10,20)mul(5,5)"), 225);
    assert_eq!(find_valid_mul_regex("randommul(4,8)random"), 32);
    assert_eq!(find_valid_mul_regex("prefixmul(1,2)suffix"), 2);
}

#[test]
fn test_invalid_mul() {
    assert_eq!(find_valid_mul_regex("mul(3,)"), 0, "Missing second number");
    assert_eq!(find_valid_mul_regex("mul(,7)"), 0, "Missing first number");
    assert_eq!(
        find_valid_mul_regex("mul(4,8!"),
        0,
        "Malformed closing parenthesis"
    );
    assert_eq!(
        find_valid_mul_regex("mul 2,3)"),
        0,
        "Missing opening parenthesis"
    );
}

#[test]
fn test_mixed_valid_invalid() {
    assert_eq!(
        find_valid_mul_regex("mul(2,3)mul(,4)mul(5,)"),
        6,
        "Only the first instruction is valid"
    );
    assert_eq!(
        find_valid_mul_regex("mul(2,3)randommul(4,5)garbage"),
        26,
        "Two valid instructions"
    );
    assert_eq!(
        find_valid_mul_regex("!!mul(6,7)*mul(8,9)~"),
        114,
        "Two valid instructions"
    );
}

#[test]
fn test_edge_cases() {
    assert_eq!(find_valid_mul_regex(""), 0, "Empty input");
    assert_eq!(
        find_valid_mul_regex("mul(0,0)"),
        0,
        "Valid but multiplies zero"
    );
    assert_eq!(
        find_valid_mul_regex("mul(123456789,1)"),
        123456789,
        "Handles large numbers"
    );
    assert_eq!(
        find_valid_mul_regex("mul(1,2)mul(2,3)mul(3,4)"),
        20,
        "Multiple valid instructions"
    );
}

#[test]
fn test_complex_input() {
    let input = "where()(< }when()mul(678,62)%mul(747,584)from()-select(59,725)\
                 mul(570,425)<~^:$where()!@}where()mul(542,816):don't()select()\
                 from()#(;!select()mul(541,668)mul(557,427)what()<#";
    assert_eq!(
        find_valid_mul_regex(input),
        678 * 62 + 747 * 584 + 570 * 425 + 542 * 816 + 541 * 668 + 557 * 427,
        "Handles complex mixed input"
    );
}
