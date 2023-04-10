#![allow(unused)]

use clap::Parser;

// Search for a pattern in file and display lines that contain it
#[derive(Parser)]
struct Cli {
    // pattern to look for
    pattern: String,
    // cross-platform path to file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}


