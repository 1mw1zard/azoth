use anyhow::Context;
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::openai;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load `.env` from the current directory (or parents); ignore if missing.
    dotenvy::dotenv().ok();

    // Use Chat Completions API (`/v1/chat/completions`). Default `openai::Client` uses the
    // Responses API (`/v1/responses`), which most OpenAI-compatible gateways do not implement (404).
    let client = openai::CompletionsClient::from_env();

    let model = std::env::var("OPENAI_MODEL").context("OPENAI_MODEL not set")?;

    let comedian_agent = client
        .agent(model)
        .preamble("You are a comedian. You are funny and you like to make people laugh.")
        .build();

    let response = comedian_agent
        .prompt("What is the meaning of life?")
        .await?;

    println!("{response}");

    Ok(())
}
