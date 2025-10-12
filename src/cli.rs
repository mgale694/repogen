use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // TODO: Call this login instead?
    /// Initialize the repogen configuration
    Init(Init),

    /// Create a new repository on GitHub
    New(New),
    // View and edit configuration
    // Config(Config),
}

#[derive(Args)]
pub struct Init {}

#[derive(Args)]
pub struct New {
    /// Name of the new repository
    pub name: String,

    /// Description of the new repository
    #[arg(short, long = "desc")]
    pub description: Option<String>,

    /// Make the repository private
    #[arg(short, long, default_value_t = false)]
    pub private: bool,
}
