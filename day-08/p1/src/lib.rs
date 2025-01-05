use std::collections::HashSet;

pub fn find_characters(input: &str) -> HashSet<char> {
    let mut characters = HashSet::new();
    for c in input.chars() {
        if !(c == '.' || c == '\n' || c == '\r') {
            characters.insert(c);
        }
    }
    return characters;
}
