pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
    let mut counts = [0; 1001];
    nums.iter().for_each(|&num| counts[num as usize] += 1);
    counts
        .iter()
        .enumerate()
        .rev()
        .find(|(_, &count)| count == 1)
        .map(|(num, _)| num as i32)
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = vec![5, 7, 3, 9, 4, 9, 8, 3, 1];
        let expected = 8;
        assert_eq!(largest_unique_number(input), expected);
    }

    #[test]
    fn example_2() {
        let input = vec![9, 9, 8, 8];
        let expected = -1;
        assert_eq!(largest_unique_number(input), expected);
    }
}
