pub fn check_if_pangram(sentence: String) -> bool {
    let mut alphabet = [false; 26];
    for c in sentence.to_lowercase().chars() {
        let idx = (c as u8 - b'a') as usize;
        alphabet[idx] = true;
    }
    alphabet.iter().all(|&x| x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "thequickbrownfoxjumpsoverthelazydog".to_string();
        let expected = true;
        assert_eq!(check_if_pangram(input), expected);
    }

    #[test]
    fn example_2() {
        let input = "leetcode".to_string();
        let expected = false;
        assert_eq!(check_if_pangram(input), expected);
    }
}
