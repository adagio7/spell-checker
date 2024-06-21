use std::fs::File;
use std::io::{ BufRead, BufReader, Result };
use std::collections::HashSet;

use crate::algorithms::bk_tree::BKTree;
use crate::algorithms::base::SpellChecker;
use crate::algorithms::levenshtein::Levenshtein;
use crate::algorithms::lcs::Lcs;
use crate::algorithms::hamming::Hamming;

pub fn load_dictionary(filename: &str) -> Result<HashSet<String>> {
    // Open the file in read-only mode
    // File is expected to be a list of words separated by newlines
    
    let file = File::open(filename)
                    .expect("Could not open file");

    let reader = BufReader::new(file);
    let mut dictionary = HashSet::new();

    for line in reader.lines() {
        dictionary.insert(line?);
    }

    Ok(dictionary)
}

pub fn bk_factory(
        algorithm: &str,
        top_matches: usize,
    ) -> BKTree {
    // Create a spellchecker based on the algorithm provided

    let spell_checker = match algorithm {
        "levenshtein" => Box::new(Levenshtein::new(top_matches)) as Box<dyn SpellChecker>,
        "lcs" => Box::new(Lcs::new(top_matches)) as Box<dyn SpellChecker>,
        "hamming" => Box::new(Hamming::new(top_matches)) as Box<dyn SpellChecker>,

        _ => panic!("Algorithm not found")
    };

    BKTree::new(spell_checker)
}

pub fn capitalize_first_letter(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn filter_alphabet(word: &str) -> String {
    // Filter out non-alphabetic characters from the word
    word
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_alphabet() {
        assert_eq!(filter_alphabet("hello"), "hello");
        assert_eq!(filter_alphabet("hello!"), "hello");
        assert_eq!(filter_alphabet("hello123"), "hello");
        assert_eq!(filter_alphabet("hello!@#123"), "hello");
    }

    #[test]
    fn test_capitalize_first_lette() {
        assert_eq!(capitalize_first_letter("hello"), "Hello");
        assert_eq!(capitalize_first_letter("Hello"), "Hello");
        assert_eq!(capitalize_first_letter("hELLO"), "HELLO");
    }

    #[test]
    #[should_panic]
    fn test_bk_factory_unknown_checker() {
        bk_factory("unknown", 3);
    }
}
