pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; nums.len()];
    let mut left = 0;
    let mut right = nums.len() - 1;

    for last_index in (0..nums.len()).rev() {
        let left_squared = nums[left] * nums[left];
        let right_squared = nums[right] * nums[right];
        let larger_squared;
        if left_squared > right_squared {
            larger_squared = left_squared;
            left += 1;
        } else {
            larger_squared = right_squared;
            right = right.saturating_sub(1);
        }
        result[last_index] = larger_squared;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![-4, -1, 0, 3, 10];
        let result = vec![0, 1, 9, 16, 100];
        assert_eq!(sorted_squares(nums), result);
    }

    #[test]
    fn example_2() {
        let nums = vec![-7, -3, 2, 3, 11];
        let result = vec![4, 9, 9, 49, 121];
        assert_eq!(sorted_squares(nums), result);
    }

    #[test]
    fn single_1() {
        let nums = vec![1];
        let result = vec![1];
        assert_eq!(sorted_squares(nums), result);
    }
}
