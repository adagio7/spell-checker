use clap::{ arg, command };
use std::collections::HashSet as Hashset;

mod utils;
mod algorithms;

use utils::{ load_dictionary, create_spellchecker };
use algorithms::base::SpellChecker;

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

    let spell_checker: Box<dyn SpellChecker> = 
                create_spellchecker(
                    matches.get_one::<String>("mode").unwrap(),
                    matches.get_one::<String>("default_matches").unwrap().parse().unwrap(),
                ).unwrap();

    println!("{:?}", spell_checker.get_matches(&dictionary.unwrap(), "helo"));
}
