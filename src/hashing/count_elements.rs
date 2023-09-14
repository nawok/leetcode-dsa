use std::collections::HashSet;

pub fn count_elements(arr: Vec<i32>) -> i32 {
    let mut count = 0;
    let set = arr.iter().collect::<HashSet<_>>();
    for num in arr.iter() {
        if set.contains(&(num + 1)) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = vec![1, 2, 3];
        let expected = 2;
        assert_eq!(count_elements(input), expected);
    }

    #[test]
    fn example_2() {
        let input = vec![1, 1, 3, 3, 5, 5, 7, 7];
        let expected = 0;
        assert_eq!(count_elements(input), expected);
    }
}
