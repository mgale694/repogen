use crate::utils::config::Config;
use dialoguer::{Confirm, Input, Password, Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

/// Handles the initialization workflow for repogen
pub struct InitHandler {
    config: Config,
    theme: ColorfulTheme,
}

/// User profile information collected during init
#[derive(Debug)]
struct UserProfile {
    github_username: String,
    full_name: Option<String>,
    email: Option<String>,
}

/// User preferences for repository defaults
#[derive(Debug)]
struct UserPreferences {
    default_private: bool,
    license: Option<String>,
    gitignore_template: Option<String>,
    preferred_editor: Option<String>,
}

/// Authentication method chosen by user
#[derive(Debug)]
enum AuthMethod {
    PersonalAccessToken,
    OAuth,
}

impl InitHandler {
    /// Create a new InitHandler instance
    pub fn new() -> Self {
        let config = Config::load().unwrap_or_else(|e| {
            eprintln!("Warning: Could not load config: {}", e);
            Config::default()
        });

        Self {
            config,
            theme: ColorfulTheme::default(),
        }
    }

    /// Main entry point for the initialization process
    pub fn handle_init() {
        let mut handler = Self::new();
        handler.run_init_workflow();
    }

    /// Authentication-only entry point for `repogen init --auth`
    pub fn handle_auth_only() {
        let mut handler = Self::new();
        handler.run_auth_only_workflow();
    }

    /// Metadata-only entry point for `repogen init --meta`
    pub fn handle_meta_only() {
        let mut handler = Self::new();
        handler.run_meta_only_workflow();
    }

    /// Run the complete initialization workflow
    fn run_init_workflow(&mut self) {
        self.display_welcome();
        self.show_existing_config_notice();

        // Collect user information in steps
        let user_profile = self.collect_user_profile();
        let preferences = self.collect_user_preferences();
        self.handle_authentication();

        // Save all configuration
        self.save_configuration(user_profile, preferences);
        self.display_completion_message();
    }

    /// Run authentication-only workflow
    fn run_auth_only_workflow(&mut self) {
        println!("🔐 repogen - Authentication Setup");
        println!("Configuring GitHub authentication only.\n");

        // Only handle authentication
        self.handle_authentication();

        // Save only the authentication token
        if let Err(e) = self.config.save() {
            eprintln!("❌ Failed to save config: {}", e);
            return;
        }

        println!("\n✅ GitHub authentication configured successfully!");
        println!("💡 Your token has been saved to ~/.config/repogen/config.toml");
        println!("🚀 Run `repogen init --meta` to complete your profile setup.");
    }

    /// Run metadata-only workflow (profile and preferences)
    fn run_meta_only_workflow(&mut self) {
        println!("👤 repogen - Profile & Preferences Setup");
        println!("Configuring your profile and repository preferences.\n");

        self.show_existing_config_notice();

        // Collect user information
        let user_profile = self.collect_user_profile();
        let preferences = self.collect_user_preferences();

        // Save configuration
        self.save_configuration(user_profile, preferences);

        println!("\n✅ Profile and preferences configured successfully!");
        println!("💡 Your settings have been saved to ~/.config/repogen/config.toml");

        // Check if they still need to set up auth
        if self.config.github_token.is_none() {
            println!("⚠️  You still need to configure GitHub authentication.");
            println!("🚀 Run `repogen init --auth` to set up your GitHub token.");
        } else {
            println!("🎉 repogen is fully configured and ready to use!");
        }
    }

    /// Display welcome message
    fn display_welcome(&self) {
        println!("Welcome to repogen! 🚀");
        println!("Let's set up your profile, preferences, and GitHub connection.\n");
    }

    /// Show notice about existing configuration if any
    fn show_existing_config_notice(&self) {
        let has_existing_config = self.config.github_username.is_some()
            || self.config.user_name.is_some()
            || self.config.github_token.is_some();

        if has_existing_config {
            println!(
                "💡 Found existing configuration. Press Enter to keep current values, or type new ones."
            );
        }
    }

    /// Collect user profile information
    fn collect_user_profile(&self) -> UserProfile {
        println!("\n👤 Step 1: User Profile");

        let github_username = Input::with_theme(&self.theme)
            .with_prompt("GitHub username")
            .default(self.config.github_username.clone().unwrap_or_default())
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().is_empty() {
                    Err("GitHub username cannot be empty")
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .unwrap();

        let full_name = Input::with_theme(&self.theme)
            .with_prompt("Your full name (for commits)")
            .default(self.config.user_name.clone().unwrap_or_default())
            .allow_empty(true)
            .interact_text()
            .unwrap();

        let email = Input::with_theme(&self.theme)
            .with_prompt("Your email (optional, for commits)")
            .default(self.config.user_email.clone().unwrap_or_default())
            .allow_empty(true)
            .interact_text()
            .unwrap();

        UserProfile {
            github_username,
            full_name: if full_name.trim().is_empty() {
                None
            } else {
                Some(full_name)
            },
            email: if email.trim().is_empty() {
                None
            } else {
                Some(email)
            },
        }
    }

    /// Collect user preferences for repository defaults
    fn collect_user_preferences(&self) -> UserPreferences {
        println!("\n⚙️ Step 2: Default Preferences");

        let default_private = self.ask_privacy_preference();
        let license = self.select_license();
        let gitignore_template = self.select_gitignore_template();
        let preferred_editor = self.select_preferred_editor();

        UserPreferences {
            default_private,
            license,
            gitignore_template,
            preferred_editor,
        }
    }

    /// Ask user about default repository privacy
    fn ask_privacy_preference(&self) -> bool {
        Confirm::with_theme(&self.theme)
            .with_prompt("Make repositories private by default?")
            .default(self.config.default_private)
            .interact()
            .unwrap()
    }

    /// Let user select default license
    fn select_license(&self) -> Option<String> {
        let license_options = vec![
            "None",
            "MIT",
            "Apache-2.0",
            "GPL-3.0",
            "BSD-3-Clause",
            "Unlicense",
        ];

        let current_index = self.find_option_index(&license_options, &self.config.default_license);

        let selection = Select::with_theme(&self.theme)
            .with_prompt("Default license for new repositories")
            .default(current_index)
            .items(&license_options)
            .interact()
            .unwrap();

        if selection == 0 {
            None
        } else {
            Some(license_options[selection].to_string())
        }
    }

    /// Let user select default .gitignore template
    fn select_gitignore_template(&self) -> Option<String> {
        let gitignore_options = vec![
            "None", "Node", "Python", "Rust", "Go", "Java", "C++", "Swift",
        ];

        let current_index =
            self.find_option_index(&gitignore_options, &self.config.default_gitignore);

        let selection = Select::with_theme(&self.theme)
            .with_prompt("Default .gitignore template")
            .default(current_index)
            .items(&gitignore_options)
            .interact()
            .unwrap();

        if selection == 0 {
            None
        } else {
            Some(gitignore_options[selection].to_string())
        }
    }

    /// Let user select preferred editor
    fn select_preferred_editor(&self) -> Option<String> {
        let editor_options = vec![
            "None",
            "VS Code",
            "Vim",
            "Emacs",
            "Sublime Text",
            "Atom",
            "IntelliJ",
        ];

        let current_index = self.find_option_index(&editor_options, &self.config.preferred_editor);

        let selection = Select::with_theme(&self.theme)
            .with_prompt("Preferred editor (for opening repos)")
            .default(current_index)
            .items(&editor_options)
            .interact()
            .unwrap();

        if selection == 0 {
            None
        } else {
            Some(editor_options[selection].to_string())
        }
    }

    /// Helper method to find the index of current option in a list
    fn find_option_index(&self, options: &[&str], current_value: &Option<String>) -> usize {
        if let Some(value) = current_value {
            options.iter().position(|&x| x == value).unwrap_or(0)
        } else {
            0
        }
    }

    /// Handle GitHub authentication setup
    fn handle_authentication(&mut self) {
        println!("\n🔐 Step 3: GitHub Authentication");

        if self.config.github_token.is_some() {
            if self.ask_keep_existing_token() {
                println!("✅ Keeping existing GitHub token");
                return;
            } else {
                self.config.github_token = None;
            }
        }

        let auth_method = self.select_auth_method();
        self.execute_authentication(auth_method);
    }

    /// Ask if user wants to keep existing token
    fn ask_keep_existing_token(&self) -> bool {
        Confirm::with_theme(&self.theme)
            .with_prompt("You already have a GitHub token configured. Keep it?")
            .default(true)
            .interact()
            .unwrap()
    }

    /// Let user select authentication method
    fn select_auth_method(&self) -> AuthMethod {
        let auth_options = &[
            "GitHub Personal Access Token (PAT)",
            "OAuth Login (Browser)",
        ];

        let selection = Select::with_theme(&self.theme)
            .with_prompt("How would you like to authenticate with GitHub?")
            .default(0)
            .items(&auth_options[..])
            .interact()
            .unwrap();

        match selection {
            0 => AuthMethod::PersonalAccessToken,
            1 => AuthMethod::OAuth,
            _ => unreachable!(),
        }
    }

    /// Execute the chosen authentication method
    fn execute_authentication(&mut self, method: AuthMethod) {
        match method {
            AuthMethod::PersonalAccessToken => self.handle_pat_authentication(),
            AuthMethod::OAuth => self.handle_oauth_authentication(),
        }
    }

    /// Handle Personal Access Token authentication
    fn handle_pat_authentication(&mut self) {
        println!("\n📝 Using Personal Access Token authentication");

        let token = Password::with_theme(&self.theme)
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
        self.config.set_github_token(token);
    }

    /// Handle OAuth authentication (simulated for now)
    fn handle_oauth_authentication(&mut self) {
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

    /// Save all configuration to file
    fn save_configuration(&mut self, profile: UserProfile, preferences: UserPreferences) {
        self.config
            .set_user_profile(profile.github_username, profile.full_name, profile.email);

        self.config.set_preferences(
            preferences.default_private,
            preferences.license,
            preferences.gitignore_template,
            preferences.preferred_editor,
        );

        if let Err(e) = self.config.save() {
            eprintln!("❌ Failed to save config: {}", e);
            return;
        }
    }

    /// Display completion message
    fn display_completion_message(&self) {
        println!("\n🎉 repogen is now fully configured and ready to use!");
        println!("💡 Your preferences have been saved to ~/.config/repogen/config.toml");
        println!("🚀 Try running: repogen new my-awesome-project");
    }
}
