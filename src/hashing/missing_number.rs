pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut missing = nums.len() as i32;
    nums.iter().enumerate().for_each(|(idx, num)| missing ^= idx as i32 ^ num);
    missing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = vec![3, 0, 1];
        let expected = 2;
        assert_eq!(missing_number(input), expected);
    }

    #[test]
    fn example_2() {
        let input = vec![0, 1];
        let expected = 2;
        assert_eq!(missing_number(input), expected);
    }

    #[test]
    fn example_3() {
        let input = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let expected = 8;
        assert_eq!(missing_number(input), expected);
    }

    #[test]
    fn example_4() {
        let input = vec![0];
        let expected = 1;
        assert_eq!(missing_number(input), expected);
    }
}
