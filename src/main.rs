use clap::Parser;
use std::io::{self, Write};

mod cli;
use cli::Cli;

fn main() {
    let args = Cli::parse();

    match args.command {
        cli::Commands::Init(_) => {
            // TODO: Move this to a separate function/module in init.rs
            println!("Welcome to repogen! 🚀");
            println!("Let's set up your GitHub connection.\n");

            print!(
                "Enter your GitHub Personal Access Token (or press Enter to open browser for login): "
            );
            io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let token = input.trim();
                    if token.is_empty() {
                        println!("Browser login not implemented yet. Please provide a token.");
                    } else {
                        println!(
                            "Token received: {}***",
                            &token[..std::cmp::min(8, token.len())]
                        );
                        // TODO: Save token securely and validate it
                    }
                }
                Err(error) => {
                    eprintln!("Error reading input: {}", error);
                }
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
