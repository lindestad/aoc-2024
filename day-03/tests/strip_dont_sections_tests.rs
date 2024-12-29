use day_03::strip_dont_sections;

#[test]
fn test_strip_dont_sections_basic() {
    assert_eq!(
        strip_dont_sections("don't()mul(2,3)"),
        "don't()",
        "Text inside 'don't()' block is removed, but 'don't()' itself is preserved"
    );
    assert_eq!(
        strip_dont_sections("mul(2,3)don't()mul(4,5)do()mul(6,7)"),
        "mul(2,3)don't()do()mul(6,7)",
        "Only the section inside 'don't()' block is removed"
    );
    assert_eq!(
        strip_dont_sections("don't()mul(4,5)do()"),
        "don't()do()",
        "Handles removal when no text follows 'do()'"
    );
}

#[test]
fn test_strip_dont_sections_multiple() {
    assert_eq!(
        strip_dont_sections("mul(2,3)don't()mul(4,5)do()mul(6,7)don't()mul(8,9)do()"),
        "mul(2,3)don't()do()mul(6,7)don't()do()",
        "Handles multiple 'don't()' and 'do()' sections"
    );
    assert_eq!(
        strip_dont_sections("don't()mul(2,3)don't()mul(4,5)do()"),
        "don't()don't()do()",
        "Handles nested 'don't()' blocks"
    );
}

#[test]
fn test_strip_dont_sections_edge_cases() {
    assert_eq!(strip_dont_sections(""), "", "Empty input remains unchanged");
    assert_eq!(
        strip_dont_sections("mul(2,3)"),
        "mul(2,3)",
        "No 'don't()' or 'do()' leaves input unchanged"
    );
    assert_eq!(
        strip_dont_sections("don't()"),
        "don't()",
        "'don't()' with no following text is preserved"
    );
    assert_eq!(
        strip_dont_sections("don't()mul(2,3)"),
        "don't()",
        "Entire section within 'don't()' is removed, but 'don't()' itself is preserved"
    );
    assert_eq!(
        strip_dont_sections("mul(2,3)don't()mul(4,5)"),
        "mul(2,3)don't()",
        "No 'do()' following 'don't()', removes text up to end"
    );
}
