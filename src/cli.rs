// File - cli.rs
use clap::{Parser, Subcommand};


// This block is from the `clap` crate to create a cli tool with
// that derives debug and implements parser.
#[derive(Debug, Parser)]
#[command(name = "ask")]
#[command(about = "shellm ask is a llm-powered cli to create shell commands")]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


// This block defines an enum which represents the subcommands of our
// CLI application
#[derive(Debug, Subcommand)]
pub enum Commands {
    // Ask a question
    Ask {
        query: String,
    },
    // Tell shellm to remember something
    Remember {
        // The content to remember
        content: String,
    },
}
