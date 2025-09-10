//Create a program that is a simple calculator that can compute statistical analysis on a dataset of numbers provided by user.

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to the CSV file containing the dataset.
    #[arg(short, long, value_name = "FILE")]
    file: std::path::PathBuf,
}

pub fn totlfn2() {
    let cli = Cli::parse();
    println!("Analyzing data from file: {:?}", cli.file);
}
