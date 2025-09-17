use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use anyhow::{Context, Result};
use clap::Parser;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .init();

    let content =
        File::open(&args.path).with_context(|| format!("Could not read file {:?}", args.path))?;

    let reader = BufReader::new(content);

    find_matches(reader, &args.pattern, &mut std::io::stdout());

    Ok(())
}
 
fn find_matches(mut content: impl BufRead, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        let line = line.unwrap();
        if line.contains(pattern){
            writeln!(writer, "{}", line).unwrap();
        }
    }
}

fn answer() -> u32 {
    42
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}

#[test]
fn test_find_matches() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet".as_bytes(), "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
