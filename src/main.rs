use clap::Parser;

mod cli;
mod commands;
mod utils;

fn main() {
    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Init(init) => {
            // Display the cool title
            utils::display_title();

            if init.authentication {
                // Skip the profile and preferences setup
                // commands::handle_auth_only();
                println!("Authentication only setup is not yet implemented.");
            } else if init.metadata {
                // Skip the authentication setup
                // commands::handle_meta_only();
                println!("Metadata only setup is not yet implemented.");
            } else {
                commands::handle_init();
            }
        }
        cli::Commands::New(new) => {
            println!(
                "Creating new repository: name='{}', description='{:?}', private={}",
                new.name, new.description, new.private
            );
        }
    }
}
