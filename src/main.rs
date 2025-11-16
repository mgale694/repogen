use clap::Parser;

mod cli;
mod commands;
mod utils;

use commands::{InitHandler, NewHandler};

fn main() {
    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Init(init) => {
            // Display the cool title
            utils::display_title();

            if init.authentication {
                // Authentication-only setup
                InitHandler::handle_auth_only();
            } else if init.metadata {
                // Metadata-only setup (profile and preferences)
                InitHandler::handle_meta_only();
            } else {
                // Full initialization workflow
                InitHandler::handle_init();
            }
        }
        cli::Commands::New(new) => {
            // Create new repository on GitHub
            match NewHandler::new(new) {
                Ok(mut handler) => {
                    if let Err(e) = handler.create_repository() {
                        eprintln!("\n❌ Error creating repository: {}", e);
                        eprintln!("\n💡 Make sure you have:");
                        eprintln!("   1. Authenticated with GitHub (run: repogen init --auth)");
                        eprintln!("   2. A valid GitHub token with 'repo' scope");
                        eprintln!("   3. Internet connection");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("\n❌ Failed to initialize: {}", e);
                    eprintln!("\n💡 Try running: repogen init");
                    std::process::exit(1);
                }
            }
        }
        cli::Commands::Config(config) => {
            utils::display_title();

            if config.view {
                commands::handle_config_view();
            } else if config.edit {
                commands::handle_config_edit();
            } else if config.clear {
                commands::handle_config_clear();
            } else {
                // Default to view if no flag is provided
                commands::handle_config_view();
            }
        }
    }
}
