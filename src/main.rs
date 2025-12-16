use anyhow::{Context, Result};
use clap::{Parser, CommandFactory};
use colored::*;
use humansize::{format_size, BINARY};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

// Import our new modules
mod args;
mod generator;
mod utils;

use args::Args;
use generator::Generator;

fn print_header() {
    println!("{}{}", "d8888b. db    db .d8888. d888888b".truecolor(190, 110, 50),"  .d88b.  db    db d88888b db    db  .d8b.  ".magenta());
    println!("{}{}", "88  `8D 88    88 88'  YP `  88  '".truecolor(190, 110, 50)," .8P  Y8. 88    88 88'     88    88 d8' `8b ".magenta());
    println!("{}{}", "88oobY' 88    88 `8bo.      88   ".truecolor(190, 110, 50)," 88    88 Y8    8P 88ooooo Y8    8P 88ooo88 ".magenta());
    println!("{}{}", "88`8b   88    88   `Y8b.    88   ".truecolor(190, 110, 50)," 88    88 `8b  d8' 88      `8b  d8' 88   88 ".magenta());
    println!("{}{}", "88 `88. 88b  d88 db   8D    88   ".truecolor(190, 110, 50)," `8b  d8'  `8bd8'  88.      `8bd8'  88   88 ".magenta());
    println!("{}{}", "88   YD  Y8888P' `8888Y'    YP   ".truecolor(190, 110, 50),"  `Y88P'     YP    Y88888P    YP    YP   YP ".magenta());
    println!("{}", format!("---  Fast and versatile name generator for custom brute force attacks v{}  ---", args::VERSION).bold());
    println!();
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    print_header();

    // 1. Validation
    if args.input_file.is_some() && args.words.is_some() {
        eprintln!("{} {} Error: Only one input method is allowed: file or list", "[✗]".red(), "ERROR:".red());
        std::process::exit(1);
    }
    if args.input_file.is_none() && args.words.is_none() {
        eprintln!("{} {} Input or output file invalid (provide -i or -p)", "[✗]".red(), "ERROR:".red());
        Args::command().print_help()?;
        std::process::exit(1);
    }

    if Path::new(&args.output_file).exists() && !args.split {
         eprintln!("{} {} Output file {} already exists", "[✗]".red(), "ERROR:".red(), args.output_file);
         std::process::exit(1);
    }
    
    // 2. Parse Range
    let (min_len, max_len) = if let Some(r) = &args.range {
        let parts: Vec<&str> = r.split('-').collect();
        if parts.len() != 2 {
            eprintln!("{} {} Range format invalid. Use 8-12", "[✗]".red(), "ERROR:".red());
            std::process::exit(1);
        }
        (parts[0].parse::<usize>().unwrap_or(0), parts[1].parse::<usize>().unwrap_or(usize::MAX))
    } else {
        (0, usize::MAX)
    };

    // 3. Prepare Input Words
    let raw_words = if let Some(file_path) = &args.input_file {
        if !Path::new(file_path).exists() {
             eprintln!("{} {} Input file {} does not exist", "[✗]".red(), "ERROR:".red(), file_path);
             std::process::exit(1);
        }
        fs::read_to_string(file_path)?
    } else {
        args.words.clone().unwrap()
    };

    let cleaned_words: Vec<String> = raw_words
        .replace('\n', " ")
        .replace('\t', " ")
        .split_whitespace()
        .map(|s| utils::remove_accents(s).to_lowercase())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let mut sorted_words = cleaned_words;
    sorted_words.sort();

    println!();
    println!("{} {} Creating dictionary", "[✓]".green(), "[OK!]".green());
    println!();
    println!("Number of input words: \t{}", sorted_words.len());
    
    let size_per_word_mb = if args.minimal { 134 } else { 288 };
    println!("Approximate maximum size: \t{}MB", sorted_words.len() * size_per_word_mb);
    if let Some(r) = &args.range {
         println!("Only passwords between {} and {} characters will be generated.", r.split('-').next().unwrap(), r.split('-').last().unwrap());
    }
    println!();
    
    std::thread::sleep(std::time::Duration::from_secs(2));

    // 4. Generation Loop
    let characters = if args.minimal { "-_.*+" } else { "-_!,.%*+$" };
    let verbose = args.verbose || args.debug;

    let start_total = Instant::now();

    if args.split {
        for word in &sorted_words {
            let fname = format!("{}_{}", word, args.output_file);
            if Path::new(&fname).exists() {
                 eprintln!("{} {} Output file {} already exists", "[✗]".red(), "ERROR:".red(), fname);
                 std::process::exit(1);
            }
            let file = File::create(&fname).context(format!("Failed to create {}", fname))?;
            println!("{} Generating combinations for {}", "[✓]".green(), word);
            let mut gen = Generator::new(file, min_len, max_len, verbose);
            gen.generate_for_word(word, args.minimal, characters)?;
        }
    } else {
        let file = File::create(&args.output_file).context(format!("Failed to create {}", args.output_file))?;
        let mut gen = Generator::new(file, min_len, max_len, verbose);
        
        for word in &sorted_words {
            println!("{} Generating combinations for {}", "[✓]".green(), word);
            gen.generate_for_word(word, args.minimal, characters)?;
        }
    }

    // 5. Final Stats
    println!();
    
    let mut total_lines = 0;
    let mut total_size_str = String::new();

    if args.split {
         println!("Words: \t(Check output directory)");
    } else {
        let metadata = fs::metadata(&args.output_file)?;
        let file = File::open(&args.output_file)?;
        let reader = BufReader::new(file);
        total_lines = reader.lines().count();
        total_size_str = format_size(metadata.len(), BINARY); 

        println!("Words: \t{}", total_lines.to_string().bold());
        println!("Size: \t{}", total_size_str);
    }

    let duration = start_total.elapsed();
    let hours = duration.as_secs() / 3600;
    let mins = (duration.as_secs() % 3600) / 60;
    let secs = duration.as_secs() % 60;

    println!("Total time: \t\t{}h {}m {}s", hours, mins, secs);
    println!();
    if !args.split {
        println!("{} File {} successfully generated {}", "[✓]".green(), fs::canonicalize(&args.output_file)?.display(), "".normal());
        println!();
        println!("Some of the words generated:");
        println!();
        
        let output = std::process::Command::new("shuf")
            .arg("-n").arg("64")
            .arg(&args.output_file)
            .output();
            
        if let Ok(o) = output {
             let s = String::from_utf8_lossy(&o.stdout);
             println!("{}", s);
        }
    } else {
        println!("{} Files generated successfully", "[✓]".green());
    }

    Ok(())
}