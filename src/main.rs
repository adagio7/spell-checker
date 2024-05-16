use clap::Parser;

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
    let args = Args::parse();

    println!("Dictionary path: {}", args.dictionary_path);
    println!("Verbose: {}", args.verbose);
}
