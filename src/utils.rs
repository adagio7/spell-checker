use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

use crate::algorithms::base::SpellChecker;
use crate::algorithms::levenshtein::Levenshtein;
// use crate::algorithms::

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

pub fn create_spellchecker(
        algorithm: &str,
        top_matches: usize,
    ) -> Option<Box<dyn SpellChecker>>{
    // Create a spellchecker based on the algorithm provided

    match algorithm {
        "levenshtein" => Some(Box::new(Levenshtein::new(top_matches)) as Box<dyn SpellChecker>),

        _ => panic!("Algorithm not found")
    }
}
