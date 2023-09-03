pub fn reverse_string(s: &mut Vec<char>) {
    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
        let mut input = vec!['h', 'e', 'l', 'l', 'o'];
        let result = vec!['o', 'l', 'l', 'e', 'h'];
        reverse_string(&mut input);
        assert_eq!(input, result);
    }

    #[test]
    fn hannah() {
        let mut input = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        let result = vec!['h', 'a', 'n', 'n', 'a', 'H'];
        reverse_string(&mut input);
        assert_eq!(input, result);
    }
}
