use crate::utils::config::Config;
use console::style;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};

/// Handles configuration management for repogen
pub struct ConfigHandler {
    config: Config,
    theme: ColorfulTheme,
}

impl ConfigHandler {
    /// Create a new ConfigHandler instance
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

    /// Entry point for viewing configuration
    pub fn handle_view() {
        let handler = Self::new();
        handler.display_config();
    }

    /// Entry point for editing configuration
    pub fn handle_edit() {
        let mut handler = Self::new();
        handler.edit_config();
    }

    /// Entry point for clearing configuration
    pub fn handle_clear() {
        Self::clear_config();
    }

    /// Display current configuration in a nice format
    fn display_config(&self) {
        println!("\n{}", style("📋 repogen Configuration").cyan().bold());
        println!("{}", style("━".repeat(50)).dim());

        // User Profile Section
        println!("\n{}", style("👤 User Profile").green().bold());
        self.print_field("GitHub Username", &self.config.github_username);
        self.print_field("Full Name", &self.config.user_name);
        self.print_field("Email", &self.config.user_email);

        // Authentication Section
        println!("\n{}", style("🔐 Authentication").green().bold());
        if let Some(ref token) = self.config.github_token {
            let masked = format!("{}***", &token[..std::cmp::min(8, token.len())]);
            println!("  GitHub Token: {}", style(masked).yellow());
        } else {
            println!("  GitHub Token: {}", style("Not configured").red());
        }

        // Repository Defaults Section
        println!("\n{}", style("⚙️  Repository Defaults").green().bold());
        println!(
            "  Private by default: {}",
            self.format_bool(self.config.default_private)
        );
        self.print_field("Default License", &self.config.default_license);
        self.print_field("Default .gitignore", &self.config.default_gitignore);
        self.print_field("Preferred Editor", &self.config.preferred_editor);

        // Config File Location
        println!("\n{}", style("📁 Configuration File").green().bold());
        if let Ok(config_path) = Config::config_file() {
            println!("  Location: {}", style(config_path.display()).cyan());
        }

        println!("\n{}", style("━".repeat(50)).dim());
        println!(
            "\n💡 Run {} to modify configuration",
            style("repogen config --edit").cyan()
        );
        println!(
            "💡 Run {} to reset configuration\n",
            style("repogen config --clear").cyan()
        );
    }

    /// Helper to print a configuration field
    fn print_field(&self, label: &str, value: &Option<String>) {
        if let Some(val) = value {
            println!("  {}: {}", label, style(val).yellow());
        } else {
            println!("  {}: {}", label, style("Not set").dim());
        }
    }

    /// Helper to format boolean values
    fn format_bool(&self, value: bool) -> String {
        if value {
            style("Yes").green().to_string()
        } else {
            style("No").yellow().to_string()
        }
    }

    /// Edit configuration interactively
    fn edit_config(&mut self) {
        println!("\n{}", style("✏️  Edit Configuration").cyan().bold());
        println!("Select what you'd like to edit:\n");

        let options = vec![
            "User Profile (username, name, email)",
            "Repository Defaults (privacy, license, gitignore, editor)",
            "GitHub Authentication (token)",
            "Edit All",
            "Cancel",
        ];

        let selection = Select::with_theme(&self.theme)
            .with_prompt("What would you like to edit?")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => self.edit_user_profile(),
            1 => self.edit_repository_defaults(),
            2 => self.edit_authentication(),
            3 => self.edit_all(),
            4 => {
                println!("❌ Edit cancelled.");
                return;
            }
            _ => unreachable!(),
        }

        // Save configuration
        if let Err(e) = self.config.save() {
            eprintln!("❌ Failed to save config: {}", e);
            return;
        }

        println!("\n✅ Configuration updated successfully!");
        println!(
            "💡 Run {} to view your updated config",
            style("repogen config --view").cyan()
        );
    }

    /// Edit user profile information
    fn edit_user_profile(&mut self) {
        println!("\n{}", style("👤 Edit User Profile").green().bold());

        let github_username = Input::with_theme(&self.theme)
            .with_prompt("GitHub username")
            .default(self.config.github_username.clone().unwrap_or_default())
            .allow_empty(false)
            .interact_text()
            .unwrap();

        let user_name = Input::with_theme(&self.theme)
            .with_prompt("Full name (for commits)")
            .default(self.config.user_name.clone().unwrap_or_default())
            .allow_empty(true)
            .interact_text()
            .unwrap();

        let user_email = Input::with_theme(&self.theme)
            .with_prompt("Email (for commits)")
            .default(self.config.user_email.clone().unwrap_or_default())
            .allow_empty(true)
            .interact_text()
            .unwrap();

        self.config.set_user_profile(
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
    }

    /// Edit repository default preferences
    fn edit_repository_defaults(&mut self) {
        println!("\n{}", style("⚙️  Edit Repository Defaults").green().bold());

        let default_private = Confirm::with_theme(&self.theme)
            .with_prompt("Make repositories private by default?")
            .default(self.config.default_private)
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
        let current_license_index =
            self.find_option_index(&license_options, &self.config.default_license);
        let license_selection = Select::with_theme(&self.theme)
            .with_prompt("Default license")
            .default(current_license_index)
            .items(&license_options)
            .interact()
            .unwrap();
        let selected_license = if license_selection == 0 {
            None
        } else {
            Some(license_options[license_selection].to_string())
        };

        let gitignore_options = vec![
            "None", "Node", "Python", "Rust", "Go", "Java", "C++", "Swift",
        ];
        let current_gitignore_index =
            self.find_option_index(&gitignore_options, &self.config.default_gitignore);
        let gitignore_selection = Select::with_theme(&self.theme)
            .with_prompt("Default .gitignore template")
            .default(current_gitignore_index)
            .items(&gitignore_options)
            .interact()
            .unwrap();
        let selected_gitignore = if gitignore_selection == 0 {
            None
        } else {
            Some(gitignore_options[gitignore_selection].to_string())
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
        let current_editor_index =
            self.find_option_index(&editor_options, &self.config.preferred_editor);
        let editor_selection = Select::with_theme(&self.theme)
            .with_prompt("Preferred editor")
            .default(current_editor_index)
            .items(&editor_options)
            .interact()
            .unwrap();
        let selected_editor = if editor_selection == 0 {
            None
        } else {
            Some(editor_options[editor_selection].to_string())
        };

        self.config.set_preferences(
            default_private,
            selected_license,
            selected_gitignore,
            selected_editor,
        );
    }

    /// Edit GitHub authentication
    fn edit_authentication(&mut self) {
        println!(
            "\n{}",
            style("🔐 Edit GitHub Authentication").green().bold()
        );
        println!(
            "⚠️  For security reasons, we recommend using 'repogen init --auth' to update your token."
        );
        println!("This ensures proper validation and secure handling.\n");

        let confirm = Confirm::with_theme(&self.theme)
            .with_prompt("Do you want to continue editing authentication here?")
            .default(false)
            .interact()
            .unwrap();

        if confirm {
            println!("💡 Please run: {}", style("repogen init --auth").cyan());
        } else {
            println!("❌ Authentication edit cancelled.");
        }
    }

    /// Edit all configuration sections
    fn edit_all(&mut self) {
        println!("\n{}", style("✏️  Edit All Configuration").cyan().bold());

        self.edit_user_profile();
        self.edit_repository_defaults();

        println!(
            "\n💡 To update authentication, run: {}",
            style("repogen init --auth").cyan()
        );
    }

    /// Helper to find option index
    fn find_option_index(&self, options: &[&str], current_value: &Option<String>) -> usize {
        if let Some(value) = current_value {
            options.iter().position(|&x| x == value).unwrap_or(0)
        } else {
            0
        }
    }

    /// Clear configuration to defaults
    fn clear_config() {
        println!("\n{}", style("🗑️  Clear Configuration").red().bold());
        println!("This will reset all configuration to default values.");
        println!(
            "{}",
            style("⚠️  This action cannot be undone!").yellow().bold()
        );

        let theme = ColorfulTheme::default();
        let confirm = Confirm::with_theme(&theme)
            .with_prompt("Are you sure you want to clear all configuration?")
            .default(false)
            .interact()
            .unwrap();

        if !confirm {
            println!("❌ Clear cancelled.");
            return;
        }

        // Double confirmation for safety
        let double_confirm = Confirm::with_theme(&theme)
            .with_prompt("Really clear? This will delete your GitHub token and all settings!")
            .default(false)
            .interact()
            .unwrap();

        if !double_confirm {
            println!("❌ Clear cancelled.");
            return;
        }

        // Get config file path and delete it
        match Config::config_file() {
            Ok(config_path) => {
                if config_path.exists() {
                    match std::fs::remove_file(&config_path) {
                        Ok(_) => {
                            println!("✅ Configuration cleared successfully!");
                            println!("💡 Run {} to set up again", style("repogen init").cyan());
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to delete config file: {}", e);
                        }
                    }
                } else {
                    println!("ℹ️  No configuration file found. Already clear!");
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to locate config file: {}", e);
            }
        }
    }
}

/// Public function to handle config view
pub fn handle_config_view() {
    ConfigHandler::handle_view();
}

/// Public function to handle config edit
pub fn handle_config_edit() {
    ConfigHandler::handle_edit();
}

/// Public function to handle config clear
pub fn handle_config_clear() {
    ConfigHandler::handle_clear();
}
