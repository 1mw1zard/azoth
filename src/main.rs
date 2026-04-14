use anyhow::Context;
use clap::Parser;
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::openai;

mod cli;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load `.env` from the current directory (or parents); ignore if missing.
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Chat {
            prompt,
            model,
            provider,
        } => {
            let client = openai::CompletionsClient::from_env();

            let model = std::env::var("OPENAI_MODEL").context("OPENAI_MODEL not set")?;

            let comedian_agent = client
                .agent(model)
                .preamble("You are a comedian. You are funny and you like to make people laugh.")
                .build();

            let response = comedian_agent.prompt(prompt).await?;

            println!("{response}");
        }
        _ => {}
    }

    Ok(())
}
