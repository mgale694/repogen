use anyhow::{Context, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub github_token: Option<String>,
    pub github_username: Option<String>,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub default_private: bool,
    pub default_license: Option<String>,
    pub default_gitignore: Option<String>,
    pub preferred_editor: Option<String>,
    pub oauth_client_id: Option<String>,
    #[serde(default)]
    pub auto_clone: bool,
    pub clone_directory: Option<String>,
}

impl Config {
    /// Get the config directory path
    pub fn config_dir() -> Result<PathBuf> {
        let home = home_dir().context("Could not find home directory")?;
        Ok(home.join(".config").join("repogen"))
    }

    /// Get the config file path
    pub fn config_file() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    /// Load config from file, or create default if it doesn't exist
    pub fn load() -> Result<Self> {
        let config_file = Self::config_file()?;

        if !config_file.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_file).context("Failed to read config file")?;

        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        Ok(config)
    }

    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir()?;
        let config_file = Self::config_file()?;

        // Create config directory if it doesn't exist
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;

        let toml_string = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&config_file, toml_string).context("Failed to write config file")?;

        println!("📁 Config saved to: {}", config_file.display());
        Ok(())
    }

    /// Set GitHub token
    pub fn set_github_token(&mut self, token: String) {
        self.github_token = Some(token);
    }

    /// Set user profile information
    pub fn set_user_profile(
        &mut self,
        username: String,
        name: Option<String>,
        email: Option<String>,
    ) {
        self.github_username = Some(username);
        self.user_name = name;
        self.user_email = email;
    }

    /// Set user preferences
    pub fn set_preferences(
        &mut self,
        default_private: bool,
        license: Option<String>,
        gitignore: Option<String>,
        editor: Option<String>,
    ) {
        self.default_private = default_private;
        self.default_license = license;
        self.default_gitignore = gitignore;
        self.preferred_editor = editor;
    }

    /// Set OAuth client ID
    pub fn set_oauth_client_id(&mut self, client_id: String) {
        self.oauth_client_id = Some(client_id);
    }

    /// Set clone settings
    pub fn set_clone_settings(&mut self, auto_clone: bool, clone_directory: Option<String>) {
        self.auto_clone = auto_clone;
        self.clone_directory = clone_directory;
    }
}
