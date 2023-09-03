pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();

    if k == 0 {
        return nums;
    }

    let mut avgs = vec![-1; n];
    if 2 * k + 1 > n {
        return avgs;
    }

    let mut window_sum = 0i64;
    nums.iter()
        .take(2 * k + 1)
        .for_each(|&i| window_sum += i as i64);
    avgs[k] = (window_sum / (2 * k + 1) as i64) as i32;

    for i in 2 * k + 1..n {
        window_sum += nums[i] as i64 - nums[i - 2 * k - 1] as i64;
        avgs[i - k] = (window_sum / (2 * k + 1) as i64) as i32;
    }

    avgs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![7, 4, 3, 9, 1, 8, 5, 2, 6];
        let k = 3;
        let result = vec![-1, -1, -1, 5, 4, 4, -1, -1, -1];
        assert_eq!(get_averages(nums, k), result);
    }

    #[test]
    fn example_2() {
        let nums = vec![100000];
        let k = 0;
        let result = vec![100000];
        assert_eq!(get_averages(nums, k), result);
    }

    #[test]
    fn example_3() {
        let nums = vec![8];
        let k = 100000;
        let result = vec![-1];
        assert_eq!(get_averages(nums, k), result);
    }

    #[test]
    fn case_100000_times_100000() {
        let nums = vec![100000; 100000];
        let k = 100000;
        let result = vec![-1; 100000];
        assert_eq!(get_averages(nums, k), result);
    }
}
