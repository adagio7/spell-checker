use crate::algorithms::base::SpellChecker;

pub struct Levenshtein;

impl SpellChecker for Levenshtein {
    fn find_suggestions(&self, word: &str) -> Vec<String> {
        vec!["hello".to_string()]
    }
}
