use crate::algorithms::base::SpellChecker;

use std::collections::{ HashSet, BinaryHeap };

pub struct Levenshtein {
    pub default_matches: usize,
}

impl Levenshtein {
    pub fn new(
        default_matches: usize,
    ) -> Self {
        Levenshtein {
            default_matches,
        }
    }

}

impl SpellChecker for Levenshtein {
    fn distance(
        &self,
        word: &str,
        target: &str,
    ) -> usize {
        /*
        * The Levenshtein distance is a string metric for measuring the difference between two sequences.
        * It is the minimum number of single-character edits (insertions, deletions, or substitutions) required to change one word into the other.
        * Formally, it is implemented as a recursive function, but we optimize it using dynamic programming.
        *
        * Reference: https://en.wikipedia.org/wiki/Levenshtein_distance
        */ 

        // Note that len(word)+1 is used to include the empty string
        let mut dp: Vec<usize> = (0..=target.len()).collect();

        for i in 1..=word.len() {
            let mut prev = dp[0];
            dp[0] = i;

            for j in 1..=target.len() {
                let temp = dp[j];
                dp[j] = if word.chars().nth(i-1) == target.chars().nth(j-1) {
                    prev
                } else {
                    std::cmp::min(
                        prev + 1,
                        std::cmp::min(
                            dp[j] + 1,
                            dp[j-1] + 1
                        )
                    )
                };
                prev = temp;
            }
        }

        dp[target.len()]
    }

    fn get_matches(
        &self,
        dictionary: &HashSet<String>,
        word: &str,
    ) -> Vec<(usize, String)> {
        let mut heap: BinaryHeap<(usize, String)> = BinaryHeap::new();

        for target in dictionary.iter() {
            let distance = self.distance(word, target);
            heap.push((distance, target.to_string()));

            if heap.len() > self.default_matches {
                heap.pop();
            }
        }

        heap.into_sorted_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::base::SpellChecker;
    use crate::algorithms::levenshtein::Levenshtein;
    use std::collections::HashSet;

    #[test]
    fn test_levenshtein_distance_for_empty_string() {
        let dictionary: HashSet<String> = HashSet::from_iter(vec![
            "kitten".to_string(),
            "sitting".to_string(),
            "saturday".to_string(),
            "sunday".to_string(),
        ]);

        let spell_checker: Levenshtein = Levenshtein::new(1);

        assert_eq!(spell_checker.get_matches(&dictionary, "").len(), 1);
        assert_eq!(spell_checker.get_matches(&dictionary, "")[0].0, 6);
    }

    #[test]
    fn test_levenshtein_distance_for_match_equal_zero() {
        let dictionary: HashSet<String> = HashSet::from_iter(vec![
            "kitten".to_string(),
            "sitting".to_string(),
            "saturday".to_string(),
            "sunday".to_string(),
        ]);

        let spell_checker: Levenshtein = Levenshtein::new(1);

        assert_eq!(spell_checker.get_matches(&dictionary, "kitten")[0].0, 0);
        assert_eq!(spell_checker.get_matches(&dictionary, "sitting")[0].0, 0);
        assert_eq!(spell_checker.get_matches(&dictionary, "saturday")[0].0, 0);
        assert_eq!(spell_checker.get_matches(&dictionary, "sunday")[0].0, 0);
    }

    #[test]
    fn test_levenshtein_distance_for_match_equal_one() {
        let dictionary: HashSet<String> = HashSet::from_iter(vec![
            "kitten".to_string(),
            "sitting".to_string(),
            "saturday".to_string(),
            "sunday".to_string(),
        ]);

        let spell_checker: Levenshtein = Levenshtein::new(1);

        assert_eq!(spell_checker.get_matches(&dictionary, "kittens")[0].0, 1);
        assert_eq!(spell_checker.get_matches(&dictionary, "sittin")[0].0, 1);
        assert_eq!(spell_checker.get_matches(&dictionary, "satyrday")[0].0, 1);
        assert_eq!(spell_checker.get_matches(&dictionary, "sundae")[0].0, 1);
    }

    #[test]
    fn test_levenshtein_with_n_matching_terms() {
        let dictionary: HashSet<String> = HashSet::from_iter(vec![
            "kitten".to_string(),
            "sitting".to_string(),
            "saturday".to_string(),
            "sunday".to_string(),
        ]);

        let spell_checker: Levenshtein = Levenshtein::new(2);

        assert_eq!(spell_checker.get_matches(&dictionary, "kittens").len(), 2);
        assert_eq!(spell_checker.get_matches(&dictionary, "sittin").len(), 2);
        assert_eq!(spell_checker.get_matches(&dictionary, "satyrday").len(), 2);
        assert_eq!(spell_checker.get_matches(&dictionary, "sundae").len(), 2);
    }

    #[test]
    fn test_levenshtein_with_n_matching_terms_and_n_greater_than_dictionary() {
        let dictionary: HashSet<String> = HashSet::from_iter(vec![
            "kitten".to_string(),
            "sitting".to_string(),
            "saturday".to_string(),
            "sunday".to_string(),
        ]);

        let spell_checker: Levenshtein = Levenshtein::new(5);

        // Returns all terms in the dictionary
        assert_eq!(spell_checker.get_matches(&dictionary, "kittens").len(), 4);
        assert_eq!(spell_checker.get_matches(&dictionary, "sittin").len(), 4);
        assert_eq!(spell_checker.get_matches(&dictionary, "satyrday").len(), 4);
        assert_eq!(spell_checker.get_matches(&dictionary, "sundae").len(), 4);
    }

    #[test]
    fn test_levenshtein_top_2_matches() {
        let dictionary: HashSet<String> = HashSet::from_iter(vec![
            "kitten".to_string(),
            "sitting".to_string(),
            "saturday".to_string(),
            "sunday".to_string(),
        ]);

        let spell_checker: Levenshtein = Levenshtein::new(2);

        assert_eq!(spell_checker.get_matches(&dictionary, "kittens")[0].0, 1);
        assert_eq!(spell_checker.get_matches(&dictionary, "kittens")[0].1, "kitten".to_string());

        assert_eq!(spell_checker.get_matches(&dictionary, "kittens")[1].0, 3);
        assert_eq!(spell_checker.get_matches(&dictionary, "kittens")[1].1, "sitting".to_string());
    }
}

