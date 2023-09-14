use std::cmp::max;

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let mut curr_sum = nums.iter().take(k).sum::<i32>();
    let mut max_sum = curr_sum;
    for i in k..nums.len() {
        curr_sum += nums[i] - nums[i - k];
        max_sum = max(curr_sum, max_sum)
    }
    (max_sum as f64) / (k as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;
        let result = 12.75;
        assert_eq!(find_max_average(nums, k), result);
    }

    #[test]
    fn example_2() {
        let nums = vec![5];
        let k = 1;
        let result = 5.0;
        assert_eq!(find_max_average(nums, k), result);
    }

    #[test]
    fn example_3() {
        let nums = vec![0, 1, 1, 3, 3];
        let k = 4;
        let result = 2.0;
        assert_eq!(find_max_average(nums, k), result);
    }
}
