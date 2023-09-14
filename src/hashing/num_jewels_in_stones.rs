use std::collections::HashMap;

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut counts = HashMap::new();
    stones.chars().for_each(|stone| {
        *counts.entry(stone).or_insert(0) += 1;
    });
    jewels.chars().fold(0, |total_count, jewel| total_count + *counts.entry(jewel).or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let jewels = "aA".to_string();
        let stones = "aAAbbbb".to_string();
        assert_eq!(num_jewels_in_stones(jewels, stones), 3);
    }

    #[test]
    fn example_2() {
        let jewels = "z".to_string();
        let stones = "ZZ".to_string();
        assert_eq!(num_jewels_in_stones(jewels, stones), 0);
    }
}
