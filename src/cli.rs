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

    /// View and edit configuration
    Config(Config),
}

#[derive(Args)]
pub struct Init {
    /// Argument to force only authentication setup
    #[arg(short, long = "auth", default_value_t = false)]
    pub authentication: bool,

    /// Argument to force only meta data setup (profile, preferences)
    #[arg(short, long = "meta", default_value_t = false)]
    pub metadata: bool,
}

#[derive(Args)]
pub struct New {
    /// Name of the new repository
    pub name: String,

    /// Description of the new repository
    #[arg(short, long = "desc")]
    pub description: Option<String>,

    /// Make the repository private (overrides config default)
    #[arg(short, long)]
    pub private: Option<bool>,

    /// Make the repository public (overrides config default)
    #[arg(long)]
    pub public: Option<bool>,

    /// License to use (overrides config default)
    /// Options: MIT, Apache-2.0, GPL-3.0, BSD-3-Clause, Unlicense, or None
    #[arg(short, long)]
    pub license: Option<String>,

    /// .gitignore template to use (overrides config default)
    /// Options: Node, Python, Rust, Go, Java, C++, Swift, or None
    #[arg(short, long)]
    pub gitignore: Option<String>,

    /// Initialize with README
    #[arg(long, default_value_t = true)]
    pub readme: bool,
}

#[derive(Args)]
pub struct Config {
    /// View the current configuration
    #[arg(short, long = "view", default_value_t = false)]
    pub view: bool,

    /// Edit the configuration interactively
    #[arg(short, long = "edit", default_value_t = false)]
    pub edit: bool,

    /// Clear configuration to default values
    #[arg(short, long = "clear", default_value_t = false)]
    pub clear: bool,
}
