#![allow(unused)]

use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}

fn search_pattern(reader: BufReader<File>, pattern: String){
    let mut str = String::new();

    let mut lines_iter = reader.lines().map(|l| l.unwrap());

    for line in lines_iter{

        let words = line.split(" ");
        let pattern_lowercase = pattern.to_ascii_lowercase();

        for word in words{
            let word_lowercase = word.to_ascii_lowercase();
            if word_lowercase == pattern_lowercase{
                println!("Found:'{}'",word_lowercase)
            }
        }

    }  
}

fn main() {
    // Example: cargo run -- hello test.txt
    let args = Cli::parse();

    let mut file = File::open(&args.path).unwrap();

    let mut reader = BufReader::new(file);

    search_pattern(reader, args.pattern);
}