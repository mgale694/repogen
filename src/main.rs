use clap::Parser;

mod cli;
mod commands;
mod utils;

use commands::InitHandler;

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
            println!(
                "Creating new repository: name='{}', description='{:?}', private={}",
                new.name, new.description, new.private
            );
        }
        cli::Commands::Config(config) => {
            println!(
                "Config commands invoked: {:?}, {:?}, {:?}",
                config.view, config.edit, config.clear
            );
        }
    }
}
