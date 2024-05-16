use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

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
