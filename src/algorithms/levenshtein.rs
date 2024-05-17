use crate::algorithms::base::SpellChecker;

use std::collections::{ HashSet, BinaryHeap };

pub struct Levenshtein {
    pub default_matches: usize,
    pub dictionary: HashSet<String>,
}

impl Levenshtein {
    pub fn new(
        default_matches: usize,
        dictionary: HashSet<String>,
    ) -> Self {
        Levenshtein {
            default_matches,
            dictionary,
        }
    }
}

impl SpellChecker for Levenshtein {
    fn find_suggestions(
        &self,
        word: &str,
    ) -> Vec<(usize, String)> {
        /*
        * The Levenshtein distance is a string metric for measuring the difference between two sequences.
        * It is the minimum number of single-character edits (insertions, deletions, or substitutions) required to change one word into the other.
        * Formally, it is implemented as a recursive function, but we optimize it using dynamic programming.
        *
        * Reference: https://en.wikipedia.org/wiki/Levenshtein_distance
        */ 
        let mut top_matches: BinaryHeap<(usize, String)> = BinaryHeap::new();

        for potential_match in &self.dictionary {
            // Note that len(word)+1 is used to include the empty string
            let mut dp = vec![vec![0; word.len() + 1]; potential_match.len() + 1];

            // Initialize the first row and column to the number of characters in the word
            for i in 0..=word.len() {
                dp[0][i] = i;
            }

            for i in 0..=potential_match.len() {
                dp[i][0] = i;
            }

            for i in 1..=potential_match.len() {
                for j in 1..=word.len() {
                    // If chars match, no cost incurred else +1
                    let cost = if word.chars().nth(j-1) == potential_match.chars().nth(i-1) {
                        0
                    } else {
                        1
                    };

                    // Compare cheapest operation from deleting, inserting, or replacing
                    dp[i][j] = std::cmp::min(
                        dp[i-1][j] + 1,
                        std::cmp::min(
                            dp[i][j-1] + 1,
                            dp[i-1][j-1] + cost
                        )
                    );
                }
            }
            top_matches.push(
                (
                    dp[potential_match.len()][word.len()],
                    potential_match.clone()
                )
            );

            // Ensure only top k matches are stored
            if top_matches.len() > self.default_matches {
                top_matches.pop();
            }
        }

        top_matches.into_sorted_vec()
    }
}
