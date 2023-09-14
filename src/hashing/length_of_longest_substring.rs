use std::cmp::max;
use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_length = 0;
    let mut current_chars = HashMap::new();
    let mut left = 0;

    for (right, c) in s.chars().enumerate() {
        while current_chars.contains_key(&c) {
            let left_char = s.chars().nth(left).unwrap();
            current_chars.remove(&left_char);
            left += 1;
        }
        *current_chars.entry(c).or_insert(0) += 1;
        max_length = max(right - left + 1, max_length);
    }
    max_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s = "abcabcbb".to_string();
        assert_eq!(length_of_longest_substring(s), 3);
    }

    #[test]
    fn example_2() {
        let s = "bbbbb".to_string();
        assert_eq!(length_of_longest_substring(s), 1);
    }

    #[test]
    fn example_3() {
        let s = "pwwkew".to_string();
        assert_eq!(length_of_longest_substring(s), 3);
    }

    #[test]
    fn example_4() {
        let s = "".to_string();
        assert_eq!(length_of_longest_substring(s), 0);
    }

    #[test]
    fn example_5() {
        let s = " ".to_string();
        assert_eq!(length_of_longest_substring(s), 1);
    }

    #[test]
    fn example_6() {
        let s = "dvdf".to_string();
        assert_eq!(length_of_longest_substring(s), 3);
    }
}
