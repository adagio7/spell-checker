use std::iter::zip;
use std::collections::{ HashSet, BinaryHeap };

use crate::algorithms::base::SpellChecker;

struct Hamming {
    pub default_matches: usize,
}

impl Hamming {
    pub fn new(
        default_matches: usize
    ) -> Self {
        // Note that usize by default can't be negative, so no check needed
        Hamming { default_matches }
    }
}

impl SpellChecker for Hamming {
    /// The hamming distance is a string metric for measuring the difference between two sequences.
    /// It is the minimum number of single-character edits (insertions, deletions, or substitutions) required to change one word into the other.

    fn distance(
        &self,
        word: &str,
        target: &str
    ) -> usize {
        if word.len() != target.len() {
            return usize::MAX;
        }

        zip(word.chars(), target.chars())
            .filter(|(a, b)| a != b)
            .count()
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
    fn test_hamming_distance() {
        let hamming = Hamming::new(3);

        assert_eq!(hamming.distance("kitten", "sittin"), 2);
        assert_eq!(hamming.distance("kitten", "kittes"), 1);
        assert_eq!(hamming.distance("kitten", "k1tt3n"), 2);
    }

    #[test]
    fn test_hamming_distance_same_word() {
        let hamming = Hamming::new(3);

        assert_eq!(hamming.distance("kitten", "kitten"), 0);
        assert_eq!(hamming.distance("sitting", "sitting"), 0);
        assert_eq!(hamming.distance("hello my friend, I'm back", "hello my friend, I'm back"), 0);
    }

    #[test]
    fn test_hamming_distance_different_length() {
        let hamming = Hamming::new(3);
        
        assert_eq!(hamming.distance("kitten", "sittingg"), usize::MAX);
        assert_eq!(hamming.distance("kitten", "sittins"), usize::MAX);
        assert_eq!(hamming.distance("kitten", "k1tt3n5"), usize::MAX);
        assert_eq!(hamming.distance("kitten", "wrejfgnjwkenfkjewn"), usize::MAX);
    }

    #[test]
    fn test_hamming_get_matches() {
        let hamming = Hamming::new(3);
        let dictionary: HashSet<String> = HashSet::from_iter(vec![
            "kitten".to_string(),
            "sittin".to_string(),
            "nettik".to_string(),
        ]);

        let matches = hamming.get_matches(&dictionary, "kitten");
        assert_eq!(
            matches, 
            vec![(0, "kitten".to_string()), (2, "sittin".to_string()), (4, "nettik".to_string())]
        );
    }

    #[test]
    fn test_hamming_get_matches_empty_dictionary() {
        let hamming = Hamming::new(3);
        let dictionary: HashSet<String> = HashSet::new();

        let matches = hamming.get_matches(&dictionary, "kitten");
        assert_eq!(matches, vec![]);
    }

    #[test]
    fn test_hamming_get_matches_no_matches() {
        let hamming = Hamming::new(0);
        let dictionary: HashSet<String> = HashSet::from_iter(vec![
            "sitting".to_string(),
            "kittens".to_string(),
        ]);

        let matches = hamming.get_matches(&dictionary, "kitten");
        assert_eq!(matches, vec![]);
    }

    #[test]
    fn test_hamming_get_matches_less_than_default() {
        let hamming = Hamming::new(3);
        let dictionary: HashSet<String> = HashSet::from_iter(vec![
            "kitten".to_string(),
            "sittin".to_string(),
        ]);

        let matches = hamming.get_matches(&dictionary, "kitten");
        assert_eq!(matches, vec![(0, "kitten".to_string()), (2, "sittin".to_string())]);
    }

    #[test]
    fn test_hamming_get_matches_more_than_default() {
        let hamming = Hamming::new(1);
        let dictionary: HashSet<String> = HashSet::from_iter(vec![
            "kitten".to_string(),
            "sitting".to_string(),
            "kittens".to_string(),
        ]);

        let matches = hamming.get_matches(&dictionary, "kitten");
        assert_eq!(matches, vec![(0, "kitten".to_string())]);
    }
}
