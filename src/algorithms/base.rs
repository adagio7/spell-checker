use std::collections::HashSet;

pub trait SpellChecker {
    fn get_matches(
        &self,
        dictionary: &HashSet<String>,
        word: &str,
    ) -> Vec<(usize, String)>;

    fn distance(
        &self,
        word: &str,
        target: &str,
    ) -> usize;
}
