use std::cmp::min;
use std::collections::HashMap;

pub fn max_number_of_balloons(text: String) -> i32 {
    let mut counts = HashMap::new();
    text.chars().for_each(|c| {
        *counts.entry(c).or_insert(0) += 1;
    });

    *counts.entry('l').or_insert(0) /= 2;
    *counts.entry('o').or_insert(0) /= 2;

    let mut min_count = text.len() + 1;
    "balloon".chars().for_each(|c| {
        min_count = min(min_count, *counts.entry(c).or_default());
    });

    min_count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "nlaebolko".to_string();
        let expected = 1;
        assert_eq!(max_number_of_balloons(input), expected);
    }

    #[test]
    fn example_2() {
        let input = "loonbalxballpoon".to_string();
        let expected = 2;
        assert_eq!(max_number_of_balloons(input), expected);
    }

    #[test]
    fn example_3() {
        let input = "leetcode".to_string();
        let expected = 0;
        assert_eq!(max_number_of_balloons(input), expected);
    }
}
