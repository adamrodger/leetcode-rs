use std::collections::HashMap;

pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut id_to_word = HashMap::new();
    let mut word_to_id = HashMap::new();

    let mut chars = pattern.chars();
    let mut words = s.split_ascii_whitespace();

    loop {
        match (chars.next(), words.next()) {
            (Some(c), Some(word)) => {
                if id_to_word.entry(c).or_insert(word) != &word
                    || word_to_id.entry(word).or_insert(c) != &c
                {
                    return false;
                }
            }
            (None, None) => return true,
            _ => return false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abba", "dog cat cat dog")]
    #[case("a", "dog")]
    #[case("e", "elephant")]
    #[case("deadbeef", "d e a d b e e f")]
    pub fn valid(#[case] pattern: String, #[case] s: String) {
        assert!(word_pattern(pattern, s))
    }

    #[rstest]
    #[case("abc", "dog cat dog")]
    #[case("ab", "dog")]
    #[case("ab", "dog dog")]
    #[case("abba", "dog dog dog dog")]
    pub fn invalid(#[case] pattern: String, #[case] s: String) {
        assert!(!word_pattern(pattern, s))
    }
}
