pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut sum = 0;
    for num in nums {
        sum += num;
        result.push(sum);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![1, 3, 6, 10];
        assert_eq!(running_sum(nums), expected);
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 1, 1, 1, 1];
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(running_sum(nums), expected);
    }

    #[test]
    fn example_3() {
        let nums = vec![3, 1, 2, 10, 1];
        let expected = vec![3, 4, 6, 16, 17];
        assert_eq!(running_sum(nums), expected);
    }
}
