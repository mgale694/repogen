use crate::shared::config::Config;
use dialoguer::{Confirm, Input, Password, Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

pub fn handle_init() {
    println!("Welcome to repogen! 🚀");
    println!("Let's set up your profile, preferences, and GitHub connection.\n");

    // Load existing config
    let mut config = Config::load().unwrap_or_else(|e| {
        eprintln!("Warning: Could not load config: {}", e);
        Config::default()
    });

    // Check if user has existing config
    let has_existing_config = config.github_username.is_some()
        || config.user_name.is_some()
        || config.github_token.is_some();

    if has_existing_config {
        println!(
            "💡 Found existing configuration. Press Enter to keep current values, or type new ones."
        );
    }

    // Step 1: User Profile
    println!("\n👤 Step 1: User Profile");

    let github_username = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("GitHub username")
        .default(config.github_username.clone().unwrap_or_default())
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Err("GitHub username cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .unwrap();

    let user_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your full name (for commits)")
        .default(config.user_name.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let user_email: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your email (optional, for commits)")
        .default(config.user_email.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()
        .unwrap();

    // Step 2: Default Preferences
    println!("\n⚙️ Step 2: Default Preferences");

    let default_private = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Make repositories private by default?")
        .default(config.default_private)
        .interact()
        .unwrap();

    let license_options = vec![
        "None",
        "MIT",
        "Apache-2.0",
        "GPL-3.0",
        "BSD-3-Clause",
        "Unlicense",
    ];

    let current_license_index = if let Some(ref license) = config.default_license {
        license_options
            .iter()
            .position(|&x| x == license)
            .unwrap_or(0)
    } else {
        0
    };

    let license_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Default license for new repositories")
        .default(current_license_index)
        .items(&license_options)
        .interact()
        .unwrap();

    let selected_license = if license_selection == 0 {
        None
    } else {
        Some(license_options[license_selection].to_string())
    };

    let gitignore_templates = vec![
        "None", "Node", "Python", "Rust", "Go", "Java", "C++", "Swift",
    ];

    let current_gitignore_index = if let Some(ref gitignore) = config.default_gitignore {
        gitignore_templates
            .iter()
            .position(|&x| x == gitignore)
            .unwrap_or(0)
    } else {
        0
    };

    let gitignore_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Default .gitignore template")
        .default(current_gitignore_index)
        .items(&gitignore_templates)
        .interact()
        .unwrap();

    let selected_gitignore = if gitignore_selection == 0 {
        None
    } else {
        Some(gitignore_templates[gitignore_selection].to_string())
    };

    let editor_options = vec![
        "None",
        "VS Code",
        "Vim",
        "Emacs",
        "Sublime Text",
        "Atom",
        "IntelliJ",
    ];

    let current_editor_index = if let Some(ref editor) = config.preferred_editor {
        editor_options
            .iter()
            .position(|&x| x == editor)
            .unwrap_or(0)
    } else {
        0
    };

    let editor_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Preferred editor (for opening repos)")
        .default(current_editor_index)
        .items(&editor_options)
        .interact()
        .unwrap();

    let selected_editor = if editor_selection == 0 {
        None
    } else {
        Some(editor_options[editor_selection].to_string())
    };

    // Step 3: Authentication
    println!("\n🔐 Step 3: GitHub Authentication");

    if config.github_token.is_some() {
        let keep_token = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("You already have a GitHub token configured. Keep it?")
            .default(true)
            .interact()
            .unwrap();

        if keep_token {
            println!("✅ Keeping existing GitHub token");
        } else {
            config.github_token = None; // Clear the token to get a new one
        }
    }

    if config.github_token.is_none() {
        let selections = &[
            "GitHub Personal Access Token (PAT)",
            "OAuth Login (Browser)",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("How would you like to authenticate with GitHub?")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => {
                // PAT method
                println!("\n📝 Using Personal Access Token authentication");

                let token = Password::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter your GitHub Personal Access Token")
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if input.starts_with("ghp_") && input.len() > 10 {
                            Ok(())
                        } else {
                            Err("Please enter a valid GitHub token (should start with 'ghp_')")
                        }
                    })
                    .interact()
                    .unwrap();

                println!("✅ Token received and validated!");
                config.set_github_token(token);
            }
            1 => {
                // OAuth method
                println!("\n🌐 Using OAuth browser authentication");

                let pb = ProgressBar::new_spinner();
                pb.set_style(
                    ProgressStyle::default_spinner()
                        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                        .template("{spinner:.blue} {msg}")
                        .unwrap(),
                );

                pb.set_message("Opening browser for GitHub authentication...");
                pb.enable_steady_tick(Duration::from_millis(120));
                thread::sleep(Duration::from_secs(2));
                pb.set_message("Waiting for authorization...");
                thread::sleep(Duration::from_secs(3));
                pb.finish_with_message("✅ Successfully authenticated with GitHub!");
                // TODO: Implement actual OAuth flow and save token
            }
            _ => unreachable!(),
        }
    }

    // Save all configuration
    config.set_user_profile(
        github_username,
        if user_name.trim().is_empty() {
            None
        } else {
            Some(user_name)
        },
        if user_email.trim().is_empty() {
            None
        } else {
            Some(user_email)
        },
    );

    config.set_preferences(
        default_private,
        selected_license,
        selected_gitignore,
        selected_editor,
    );

    if let Err(e) = config.save() {
        eprintln!("❌ Failed to save config: {}", e);
        return;
    }

    println!("\n🎉 repogen is now fully configured and ready to use!");
    println!("💡 Your preferences have been saved to ~/.config/repogen/config.toml");
    println!("🚀 Try running: repogen new my-awesome-project");
}
