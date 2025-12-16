use clap::Parser;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(author, version = VERSION, about = "Fast and versatile name generator (Rust Port)", long_about = None)]
pub struct Args {
    /// Input file containing the base words
    #[arg(short = 'i', long = "inputFile")]
    pub input_file: Option<String>,

    /// Quoted list of space-separated words
    #[arg(short = 'p')]
    pub words: Option<String>,

    /// Output file for the dictionary
    #[arg(short = 'o', long = "outputFile")]
    pub output_file: String,

    /// Splits the output into one file per input word
    #[arg(short = 's')]
    pub split: bool,

    /// Range of characters to use. Format: 8-12 or 10-10
    #[arg(short = 'r')]
    pub range: Option<String>,

    /// Minimal mode. Generates fewer combinations per word
    #[arg(short = 'm')]
    pub minimal: bool,

    /// Verbose mode. Displays the created combinations
    #[arg(short = 'v')]
    pub verbose: bool,

    /// Debug mode
    #[arg(short = 'd', long = "debug")]
    pub debug: bool,
}