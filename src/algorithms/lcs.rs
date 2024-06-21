use std::cmp::max;
use std::collections::{ HashSet, BinaryHeap };

use crate::algorithms::base::SpellChecker;

pub struct Lcs {
    pub default_matches: usize,
}

impl Lcs {
    pub fn new(
        default_matches: usize
    ) -> Self {
        // Note that usize by default can't be negative, so no check needed
        Lcs { default_matches }
    }
}

impl SpellChecker for Lcs {
    /// The Longest Common Subsequence (LCS) is a string metric for measuring the difference between two sequences.
    /// It measures the length of the longest subsequence common to both sequences.
    fn distance(
        &self,
        word: &str,
        target: &str
    ) -> usize {
        let mut dp = vec![vec![0; target.len() + 1]; word.len() + 1];

        for (i, c1) in word.chars().enumerate() {
            for (j, c2) in target.chars().enumerate() {
                if c1 == c2 {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = max(dp[i + 1][j], dp[i][j + 1]);
                }
            }
        }

        word.len() + target.len() - 2 * dp[word.len()][target.len()]
    }

    fn get_matches(
        &self,
        dictionary: &HashSet<String>,
        word: &str,
    ) -> Vec<(usize, String)> {
        let mut heap: BinaryHeap<(usize, String)> = BinaryHeap::new();

        for entry in dictionary.iter() {
            let dist = self.distance(word, entry);
            heap.push((dist, entry.clone()));

            if heap.len() > self.default_matches {
                heap.pop();
            }
        }
        heap.into_sorted_vec()
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs_distance() {
        let lcs = Lcs::new(3);

        assert_eq!(lcs.distance("kitten", "sitting"), 5);
        assert_eq!(lcs.distance("kitten", "sittin"), 4);
        assert_eq!(lcs.distance("cat", "cut"), 2);
        assert_eq!(lcs.distance("abcdef", "acbcf"), 3);
    }

    #[test]
    fn test_no_common_lcs_distance() {
        let lcs = Lcs::new(3);

        assert_eq!(lcs.distance("abc", "xyz"), 6);
        
    }

    #[test]
    fn test_lcs_distance_same_word() {
        let lcs = Lcs::new(3);

        assert_eq!(lcs.distance("kitten", "kitten"), 0);
        assert_eq!(lcs.distance("sitting", "sitting"), 0);
        assert_eq!(lcs.distance("hello my friend, I'm back", "hello my friend, I'm back"), 0);
    }

    #[test]
    fn test_lcs_distance_empty_string() {
        let lcs = Lcs::new(3);

        assert_eq!(lcs.distance("", ""), 0);
        assert_eq!(lcs.distance("kitten", ""), 6);
        assert_eq!(lcs.distance("", "kitten"), 6);
    }

    #[test]
    fn test_lcs_get_matches() {
        let lcs = Lcs::new(2);
        let dictionary: HashSet<String> = HashSet::from_iter(vec![
            "kitten".to_string(),
            "sitting".to_string(),
            "cat".to_string(),
            "cut".to_string(),
            "abcdef".to_string(),
        ]);

        assert_eq!(lcs.get_matches(&dictionary, "kitten"), vec![(0, "kitten".to_string()), (5, "sitting".to_string())]);
        assert_eq!(lcs.get_matches(&dictionary, "sittin"), vec![(1, "sitting".to_string()), (4, "kitten".to_string())]);
        assert_eq!(lcs.get_matches(&dictionary, "catt"), vec![(1, "cat".to_string()), (3, "cut".to_string())]);
        assert_eq!(lcs.get_matches(&dictionary, "acbcf"), vec![(3, "abcdef".to_string()), (6, "cat".to_string())]);
    }

    #[test]
    fn test_lcs_get_zero_matches() {
        let lcs = Lcs::new(0);

        let dictionary: HashSet<String> = HashSet::from_iter(vec![
            "kitten".to_string(),
            "sitting".to_string(),
            "cat".to_string(),
            "cut".to_string(),
            "abcdef".to_string(),
        ]);

        assert_eq!(lcs.get_matches(&dictionary, "kitten"), vec![]);
    }
}
