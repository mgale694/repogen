use clap::Parser;

mod cli;
mod config;
mod init;
mod launch;

fn main() {
    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Init(init) => {
            // Display the cool title
            launch::display_title();

            if init.authentication {
                // Skip the profile and preferences setup
                // init::handle_auth_only();
                println!("Authentication only setup is not yet implemented.");
            } else if init.metadata {
                // Skip the authentication setup
                // init::handle_meta_only();
                println!("Metadata only setup is not yet implemented.");
            } else {
                init::handle_init();
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
