use std::collections::HashMap;

pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut losses = HashMap::new();
    matches.iter().for_each(|m| {
        losses.entry(m[0]).or_insert(0);
        *losses.entry(m[1]).or_insert(0) += 1;
    });

    let mut never_lost = vec![];
    let mut lost_once = vec![];
    losses.iter().for_each(|(k, v)| {
        match v {
            0 => never_lost.push(*k),
            1 => lost_once.push(*k),
            _ => (),
        }
    });

    never_lost.sort();
    lost_once.sort();
    vec![never_lost, lost_once]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];
        let expected = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];
        assert_eq!(find_winners(matches), expected);
    }

    #[test]
    fn example_2() {
        let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
        let expected = vec![vec![1, 2, 5, 6], vec![]];
        assert_eq!(find_winners(matches), expected);
    }
}
