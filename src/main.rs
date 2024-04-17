// INTRO HERE

// Read the cli args using this crate
use std::env;
use clap::Parser;
mod cli;
mod database;
mod embeddings;
mod llm;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let args = cli::Cli::parse();
    match args.command {
        cli::Commands::Ask { query } => {
            let context = database::retrieve(&query).await?;
            let answer = llm::answer_with_context(&query, context).await?;
            println!("Answer: {}", answer);
        },
        cli::Commands::Remember { context } => {
            database::insert(&content).await?;
        }
    }
    Ok(())



}
