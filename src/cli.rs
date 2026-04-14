use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "azoth", version = "0.1.0", about = "A ai-based agent")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Chat with the agent with non-interactive mode.
    Chat {
        /// The prompt to ask.
        #[arg(short, long)]
        prompt: String,

        /// The model to use.
        #[arg(short, long)]
        model: String,

        /// The provider to use.
        #[arg(short, long)]
        provider: String,
    },

    Provider,

    /// List all models.
    Model,

    /// List all tools.
    Tools,

    /// Manage memory.
    #[command(subcommand)]
    Memory(MemoryCommands),
}

#[derive(Subcommand)]
pub enum MemoryCommands {
    /// List all memory items.
    List,
}
