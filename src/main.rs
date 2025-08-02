// src/main.rs
/*
 * Main executable for PrimeToken
 */

use clap::Parser;
use primetoken::{Result, run}; 
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser)]
#[command(version, about = "PrimeToken - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

pub fn run(verbose: bool, input: Option<String>, output: Option<String>) -> Result<()> {
    let input_content = if let Some(ref input_path) = input {
        let path = Path::new(input_path);
        if !path.exists() {
            return Err(anyhow::anyhow!("Input file '{}' not found.", input_path));
        }

        let content = fs::read_to_string(path)?;
        if content.trim().is_empty() {
            return Err(anyhow::anyhow!("Input file '{}' is empty.", input_path));
        }

        content
    } else {
        return Err(anyhow::anyhow!("No input file specified."));
    };

    // Placeholder: process input_content
    let output_content = format!("Processed content: {}", &input_content[..20.min(input_content.len())]);

    if let Some(output_path) = output {
        let mut file = fs::File::create(output_path)?;
        file.write_all(output_content.as_bytes())?;
        if verbose {
            println!("Output written successfully.");
        }
    } else if verbose {
        println!("No output file specified. Result:\n{}", output_content);
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose, args.input, args.output)
}
