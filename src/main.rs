use clap::{ arg, command };
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::collections::HashSet as Hashset;

mod utils;
mod algorithms;

use utils::{ bk_factory, filter_alphabet, load_dictionary };
use algorithms::bk_tree::BKTree;

fn correct_file(file_path: &str, spell_checker: &BKTree) {
    // Goes through the file and identify the spell errors
    
    let file = File::open(file_path)
                    .expect("Could not open file");

    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let words = line.split_whitespace();

        for (j, word) in words.enumerate() {
            let cleaned_word = filter_alphabet(word);
            let results = spell_checker.search(&cleaned_word, 1);

            if !results.is_empty() && !results.contains(&cleaned_word.to_string()) {
                println!("Line {} Word {}: Misspelled {}, Suggested: {}", i, j, cleaned_word, results[0]);
            }
        }
    } 
}

fn main() {
    let matches =
        command!("spell_check")
            .arg(
                arg!(-d --dictionary_path <path> "Path to the dictionary file")
                    .default_value("./dictionaries/google-10k-eng.txt")
            )
            .arg(
                arg!(-t --text_path <path> "Path to the text file to spell check")
            )
            .arg(
                arg!(-v --verbose "Prints debug information verbosely")
            )
            .arg(
                arg!(-n --default_matches <n> "Number of default matches to return")
                    .default_value("5")
            )
            .arg(
                arg!(-m --mode <mode> "Mode to run the spell checker in")
                    // Use the ALGORITHMS keys as possible values
                    // .value_parser(ALGORITHMS.keys().copied().collect::<Vec<&str>>())
                    .default_value("levenshtein")
            )
            .get_matches();


    let dictionary: Result<Hashset<String>, std::io::Error> = load_dictionary(matches.get_one::<String>("dictionary_path").unwrap());

    let mut spell_checker = bk_factory(
        matches.get_one::<String>("mode").unwrap(),
        matches.get_one::<String>("default_matches").unwrap().parse().unwrap(),
    );

    spell_checker.load_dictionary(&dictionary.unwrap());

    correct_file(
        matches.get_one::<String>("text_path").unwrap(),
        &spell_checker,
    );
}
