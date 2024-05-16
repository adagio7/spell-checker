use clap::Parser;
use std::collections::HashSet as Hashset;

mod utils;
mod algorithms;

use algorithms::base::SpellChecker;
use algorithms::levenshtein::Levenshtein;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    dictionary_path: String, 

    #[arg(short, long)]
    verbose: bool,

    // #[arg(subcommand)]
    // cmd: Command,
}

fn main() {
    // let args = Args::parse();

    let dictionary: Result<Hashset<String>, std::io::Error> = utils::load_dictionary("./dictionaries/google-10k-eng.txt");

    let spell_checker = Levenshtein{
        default_matches: 5,
        dictionary: dictionary.unwrap()
    };
    println!("{:?}", spell_checker.find_suggestions("helo"));
}
