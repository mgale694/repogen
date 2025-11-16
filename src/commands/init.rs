use crate::utils::config::Config;
use anyhow::{Context, Result, anyhow};
use dialoguer::{Confirm, Input, Password, Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde::Deserialize;
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

/// Response from GitHub device code request
#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
}

/// Response from GitHub OAuth token request
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    #[allow(dead_code)]
    token_type: String,
    #[allow(dead_code)]
    scope: String,
}

/// Error response from GitHub OAuth
#[derive(Debug, Deserialize)]
struct TokenErrorResponse {
    error: String,
    error_description: Option<String>,
}

/// GitHub API user information for validation
#[derive(Debug, Deserialize)]
struct GitHubUser {
    login: String,
    #[allow(dead_code)]
    name: Option<String>,
    #[allow(dead_code)]
    email: Option<String>,
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
            "OAuth Login (Browser) - Recommended",
            "GitHub Personal Access Token (PAT)",
        ];

        let selection = Select::with_theme(&self.theme)
            .with_prompt("How would you like to authenticate with GitHub?")
            .default(0)
            .items(&auth_options[..])
            .interact()
            .unwrap();

        match selection {
            0 => AuthMethod::OAuth,
            1 => AuthMethod::PersonalAccessToken,
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
        println!("💡 Create a token at: https://github.com/settings/tokens/new");
        println!("   Required scopes: repo, user\n");

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

        // Validate token with GitHub API
        print!("🔍 Validating token with GitHub... ");
        match Self::validate_github_token(&token) {
            Ok(user) => {
                println!("✅ Success!");
                println!("👤 Authenticated as: {}", user.login);

                // Optionally use the validated username
                if self.config.github_username.is_none() {
                    self.config.github_username = Some(user.login);
                }

                self.config.set_github_token(token);
            }
            Err(e) => {
                println!("❌ Failed!");
                eprintln!("Error: {}", e);
                eprintln!("\n⚠️  Token validation failed. Please check:");
                eprintln!("   1. Token is correct and not expired");
                eprintln!("   2. Token has required scopes (repo, user)");
                eprintln!("   3. You have internet connection");

                let retry = Confirm::with_theme(&self.theme)
                    .with_prompt("Would you like to try again?")
                    .default(true)
                    .interact()
                    .unwrap();

                if retry {
                    self.handle_pat_authentication();
                }
            }
        }
    }

    /// Handle OAuth authentication using GitHub Device Flow
    fn handle_oauth_authentication(&mut self) {
        println!("\n🌐 OAuth Browser Authentication");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        // Check if OAuth client ID is already configured
        if self.config.oauth_client_id.is_none() {
            println!("📋 OAuth Setup Required");
            println!("\nOAuth authentication requires a GitHub OAuth App.");
            println!("This is a one-time setup that takes about 2 minutes.\n");

            let setup_now = Confirm::with_theme(&self.theme)
                .with_prompt("Would you like to set up OAuth now?")
                .default(false)
                .interact()
                .unwrap();

            if !setup_now {
                println!("\n💡 No problem! You can use Personal Access Token instead.");
                let use_pat = Confirm::with_theme(&self.theme)
                    .with_prompt("Use Personal Access Token?")
                    .default(true)
                    .interact()
                    .unwrap();

                if use_pat {
                    self.handle_pat_authentication();
                }
                return;
            }

            // Guide user through OAuth app creation
            self.guide_oauth_setup();
        }

        // Run OAuth device flow with configured client ID
        match self.run_device_flow() {
            Ok(token) => {
                println!("✅ Successfully authenticated with GitHub!");
                self.config.set_github_token(token);
            }
            Err(e) => {
                eprintln!("❌ OAuth authentication failed: {}", e);
                println!("\n💡 You can try:");
                println!("   1. Run the setup again: repogen init --auth");
                println!("   2. Use a Personal Access Token instead");

                let use_pat = Confirm::with_theme(&self.theme)
                    .with_prompt("Would you like to use Personal Access Token?")
                    .default(true)
                    .interact()
                    .unwrap();

                if use_pat {
                    self.handle_pat_authentication();
                }
            }
        }
    }

    /// Guide user through OAuth app setup
    fn guide_oauth_setup(&mut self) {
        println!("\n📝 OAuth App Setup Guide");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        println!("Step 1: Create a GitHub OAuth App");
        println!("   → Open: https://github.com/settings/developers\n");

        if webbrowser::open("https://github.com/settings/developers").is_ok() {
            println!("✅ Browser opened automatically\n");
        } else {
            println!("⚠️  Please open the URL manually in your browser\n");
        }

        println!("Step 2: Click 'New OAuth App' (or 'Register a new application')\n");

        println!("Step 3: Fill in the application details:");
        println!("   • Application name: repogen");
        println!("   • Homepage URL: https://github.com/mgale694/repogen");
        println!("   • Authorization callback URL: http://127.0.0.1");
        println!("   • Description: CLI tool for GitHub repository creation\n");

        println!("Step 4: After creating the app:");
        println!("   • Check the box: ☑️  Enable Device Flow");
        println!("   • Copy the Client ID (starts with 'Iv1.' or similar)\n");

        Confirm::with_theme(&self.theme)
            .with_prompt("Press Enter when you've created the app and have the Client ID ready")
            .default(true)
            .interact()
            .unwrap();

        // Prompt for client ID
        println!("\n📋 Enter OAuth App Details\n");

        let client_id = Input::with_theme(&self.theme)
            .with_prompt("GitHub OAuth App Client ID")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().is_empty() {
                    Err("Client ID cannot be empty")
                } else if input.trim().len() < 10 {
                    Err("Client ID seems too short. Please check and try again.")
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .unwrap();

        // Save client ID to config
        self.config
            .set_oauth_client_id(client_id.trim().to_string());

        if let Err(e) = self.config.save() {
            eprintln!("⚠️  Warning: Could not save client ID to config: {}", e);
            println!("You may need to enter it again next time.");
        } else {
            println!("\n✅ Client ID saved to config!");
        }

        println!("\n🎉 OAuth setup complete! Now let's authenticate...\n");
    }

    /// Run the GitHub OAuth Device Flow
    ///
    /// Uses the OAuth client ID stored in config to authenticate via GitHub's device flow.
    fn run_device_flow(&self) -> Result<String> {
        const DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
        const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

        // Get client ID from config
        let client_id =
            self.config.oauth_client_id.as_ref().ok_or_else(|| {
                anyhow!("OAuth client ID not configured. Please run setup first.")
            })?;

        let client = Client::new();

        // Step 1: Request device and user verification codes
        println!("📝 Requesting device code from GitHub...");
        let device_response: DeviceCodeResponse = client
            .post(DEVICE_CODE_URL)
            .header("Accept", "application/json")
            .form(&[("client_id", client_id.as_str()), ("scope", "repo user")])
            .send()
            .context("Failed to request device code")?
            .json()
            .context("Failed to parse device code response")?;

        // Step 2: Show user code and open browser
        println!("\n┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        println!("┃  Please visit: {}   ┃", device_response.verification_uri);
        println!(
            "┃  And enter code: {}                       ┃",
            device_response.user_code
        );
        println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛\n");

        // Try to open browser automatically
        if webbrowser::open(&device_response.verification_uri).is_ok() {
            println!("✅ Browser opened automatically");
        } else {
            println!("⚠️  Could not open browser automatically");
        }

        // Step 3: Poll for authorization
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message("Waiting for authorization...");
        pb.enable_steady_tick(Duration::from_millis(120));

        let interval = Duration::from_secs(device_response.interval);
        let mut attempts = 0;
        let max_attempts = (device_response.expires_in / device_response.interval) as usize;

        loop {
            if attempts >= max_attempts {
                pb.finish_with_message("❌ Device code expired");
                return Err(anyhow!("Device code expired. Please try again."));
            }

            thread::sleep(interval);
            attempts += 1;

            // Poll for token
            let response = client
                .post(TOKEN_URL)
                .header("Accept", "application/json")
                .form(&[
                    ("client_id", client_id.as_str()),
                    ("device_code", &device_response.device_code),
                    ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ])
                .send()
                .context("Failed to poll for token")?;

            let text = response.text().context("Failed to read response")?;

            // Try to parse as success response first
            if let Ok(token_response) = serde_json::from_str::<TokenResponse>(&text) {
                pb.finish_with_message("✅ Authorization successful!");
                return Ok(token_response.access_token);
            }

            // Parse as error response
            if let Ok(error_response) = serde_json::from_str::<TokenErrorResponse>(&text) {
                match error_response.error.as_str() {
                    "authorization_pending" => {
                        // User hasn't authorized yet, keep polling
                        continue;
                    }
                    "slow_down" => {
                        // We're polling too fast, add 5 seconds to interval
                        pb.set_message("Slowing down polling...");
                        thread::sleep(Duration::from_secs(5));
                        continue;
                    }
                    "expired_token" => {
                        pb.finish_with_message("❌ Device code expired");
                        return Err(anyhow!("Device code expired. Please try again."));
                    }
                    "access_denied" => {
                        pb.finish_with_message("❌ Access denied");
                        return Err(anyhow!("User denied access"));
                    }
                    _ => {
                        pb.finish_with_message("❌ Authentication failed");
                        return Err(anyhow!(
                            "Authentication error: {} - {}",
                            error_response.error,
                            error_response.error_description.unwrap_or_default()
                        ));
                    }
                }
            }

            // Unexpected response
            pb.finish_with_message("❌ Unexpected response");
            return Err(anyhow!("Unexpected response from GitHub: {}", text));
        }
    }

    /// Validate a GitHub token by making an API call
    fn validate_github_token(token: &str) -> Result<GitHubUser> {
        let client = Client::new();

        let response = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "repogen-cli")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .context("Failed to validate token with GitHub API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_default();
            return Err(anyhow!(
                "GitHub API returned error {}: {}",
                status,
                error_text
            ));
        }

        let user: GitHubUser = response
            .json()
            .context("Failed to parse GitHub user response")?;

        Ok(user)
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
