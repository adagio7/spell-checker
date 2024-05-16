use clap::Parser;
use std::path::Path;
use std::collections::HashSet as Hashset;

mod utils;

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

    let dictionary: Result<Hashset<String>, std::io::Error> = utils::load_dictionary("./src/dictionaries/google-10k-eng.txt");
    println!("{:?}", dictionary);
}
