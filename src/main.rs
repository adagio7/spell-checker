use clap::{ Parser, arg, command };
use std::collections::HashSet as Hashset;

mod utils;
mod algorithms;

use utils::load_dictionary;
use algorithms::base::SpellChecker;
use algorithms::levenshtein::Levenshtein;

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
            .subcommand(
                command!("levenshtein").about("Uses the Levenshtein algorithm to spellcheck")
            )
            .get_matches();


    let dictionary: Result<Hashset<String>, std::io::Error> = load_dictionary(matches.get_one::<String>("dictionary_path").unwrap());

    let spell_checker = Levenshtein{
        default_matches: 5,
        dictionary: dictionary.unwrap()
    };
    println!("{:?}", spell_checker.find_suggestions("helo"));
}
