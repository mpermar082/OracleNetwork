// src/main.rs
/*
 * Main executable for OracleNetwork
 * 
 * This is the entry point for the OracleNetwork application.
 * It uses the clap crate to parse command line arguments and the oraclenetwork crate to run the application logic.
 */

use clap::Parser;
use oraclenetwork::{Result, run};

#[derive(Parser)]
#[command(version, about = "OracleNetwork - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Path to the input file
    #[arg(short, long, default_value = "")]
    input: Option<String>,
    
    /// Path to the output file
    #[arg(short, long, default_value = "")]
    output: Option<String>,
}

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Cli::parse();
    
    // Run the application logic
    run(args.verbose, args.input, args.output)
}