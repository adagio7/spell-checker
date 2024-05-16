pub trait SpellChecker {
    fn find_suggestions(
        &self,
        word: &str,
    ) -> Vec<(usize, String)>;
}
