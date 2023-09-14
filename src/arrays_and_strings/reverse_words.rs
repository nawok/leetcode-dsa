pub fn reverse_words(s: String) -> String {
    s.chars().rev().collect::<String>().split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "Let's take LeetCode contest".to_string();
        let expected = "s'teL ekat edoCteeL tsetnoc".to_string();
        assert_eq!(reverse_words(input), expected);
    }

    #[test]
    fn example_2() {
        let input = "God Ding".to_string();
        let expected = "doG gniD".to_string();
        assert_eq!(reverse_words(input), expected);
    }
}
