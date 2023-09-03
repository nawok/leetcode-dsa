pub fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut min_sum = 0;
    let mut sum = 0;
    for num in nums {
        sum += num;
        min_sum = min_sum.min(sum);
    }
    1 - min_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![-3, 2, -3, 4, 2];
        let expected = 5;
        assert_eq!(min_start_value(nums), expected);
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 2];
        let expected = 1;
        assert_eq!(min_start_value(nums), expected);
    }

    #[test]
    fn example_3() {
        let nums = vec![1, -2, -3];
        let expected = 5;
        assert_eq!(min_start_value(nums), expected);
    }
}
