use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

use crate::algorithms::base::SpellChecker;
use crate::algorithms::levenshtein::Levenshtein;

pub fn load_dictionary(filename: &str) -> Result<HashSet<String>> {
    let file = File::open(filename)
                    .expect("Could not open file");

    let reader = BufReader::new(file);
    let mut dictionary = HashSet::new();

    for line in reader.lines() {
        dictionary.insert(line?);
    }

    Ok(dictionary)
}

pub fn create_spellchecker(
        algorithm: &str,
        top_matches: usize,
        dictionary: HashSet<String>
    ) -> Option<Box<dyn SpellChecker>>{
    match algorithm {
        "levenshtein" => Some(Box::new(Levenshtein::new(top_matches, dictionary)) as Box<dyn SpellChecker>),
        _ => panic!("Algorithm not found")
    }

}
