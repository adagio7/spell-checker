use clap::{ arg, command };
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::collections::HashSet as Hashset;

mod utils;
mod algorithms;

use utils::{ load_dictionary, create_spellchecker };
use algorithms::base::SpellChecker;
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
            if spell_checker.search(word, 1)[0] != word {
                println!("Line {} Word {}: Misspelled {}, Suggested: {}", i, j, word, spell_checker.search(word, 1)[0]);
            }
        }
    } 
}

fn main() {
    // TODO: Let user specify the dictionary module_path!
    // TODO: Let user specify the algorithm to use!
    // TODO: Let user toggle default matches

    let matches =
        command!("spell_check")
            .arg(
                arg!(-d --dictionary_path <path> "Path to the dictionary file")
                    .default_value("./dictionaries/google-10k-eng.txt")
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

    // let spell_checker: Box<dyn SpellChecker> = 
    //             create_spellchecker(
    //                 matches.get_one::<String>("mode").unwrap(),
    //                 matches.get_one::<String>("default_matches").unwrap().parse().unwrap(),
    //             ).unwrap();
    //
    // println!("{:?}", spell_checker.get_matches(&dictionary.unwrap(), "helo"));

    let mut spell_checker = BKTree::new();
    spell_checker.load_dictionary(&dictionary.unwrap());

    correct_file("./text/fable1.txt", &spell_checker);

    println!("{:?}", spell_checker.search("helo", 2));
}
