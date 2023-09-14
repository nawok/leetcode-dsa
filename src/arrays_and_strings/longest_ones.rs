use std::cmp::max;

pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut max_len = 0;
    let mut curr_zeros = 0;

    let mut left = 0;
    for right in 0..nums.len() {
        if nums[right] == 0 {
            curr_zeros += 1;
        }

        while curr_zeros > k {
            if nums[left] == 0 {
                curr_zeros -= 1;
            }
            left += 1;
        }

        let curr_len = right - left + 1;
        max_len = max(curr_len, max_len);
    }

    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let k = 2;
        let result = 6;
        assert_eq!(longest_ones(nums, k), result);
    }

    #[test]
    fn example_2() {
        let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        let k = 3;
        let result = 10;
        assert_eq!(longest_ones(nums, k), result);
    }
}
