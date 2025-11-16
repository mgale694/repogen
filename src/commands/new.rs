use crate::cli;
use crate::utils::config::Config;
use anyhow::{Context, Result, anyhow};
use console::Style;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

/// Handles the repository creation workflow
pub struct NewHandler {
    config: Config,
    args: cli::New,
}

/// Request body for GitHub repository creation
#[derive(Debug, Serialize)]
struct CreateRepoRequest {
    name: String,
    description: Option<String>,
    private: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitignore_template: Option<String>,
    auto_init: bool,
}

/// Response from GitHub repository creation
#[derive(Debug, Deserialize)]
struct CreateRepoResponse {
    html_url: String,
    clone_url: String,
    ssh_url: String,
    name: String,
    full_name: String,
    private: bool,
}

impl NewHandler {
    /// Create a new NewHandler instance
    pub fn new(args: cli::New) -> Result<Self> {
        let config = Config::load().context("Failed to load configuration")?;

        Ok(Self { config, args })
    }

    /// Create the repository on GitHub
    pub fn create_repository(&mut self) -> Result<()> {
        // Display banner
        self.display_banner();

        // Validate we have a token
        let token = self.config.github_token.as_ref().ok_or_else(|| {
            anyhow!("No GitHub token found. Run `repogen init --auth` to authenticate.")
        })?;

        // Determine settings (CLI flags override config defaults)
        let is_private = self.determine_privacy();
        let license = self.determine_license();
        let gitignore = self.determine_gitignore();

        // Display configuration
        self.display_config(&is_private, &license, &gitignore);

        // Create request body
        let request = CreateRepoRequest {
            name: self.args.name.clone(),
            description: self.args.description.clone(),
            private: is_private,
            license_template: license.clone(),
            gitignore_template: gitignore.clone(),
            auto_init: self.args.readme,
        };

        // Make API call
        println!("\n📦 Creating repository on GitHub...");
        let response = self.call_github_api(token, &request)?;

        // Display success
        self.display_success(&response);

        // Auto-clone if enabled
        if self.config.auto_clone {
            self.clone_repository(&response)?;
        }

        Ok(())
    }

    /// Display the banner
    fn display_banner(&self) {
        let cyan = Style::new().cyan().bold();
        println!("\n{}", cyan.apply_to("📦 repogen - Create New Repository"));
        println!("{}", "━".repeat(50));
    }

    /// Determine if repository should be private
    fn determine_privacy(&self) -> bool {
        // --public flag takes precedence (explicit public)
        if self.args.public == Some(true) {
            return false;
        }

        // --private flag takes precedence (explicit private)
        if self.args.private == Some(true) {
            return true;
        }

        // Fall back to config default
        self.config.default_private
    }

    /// Determine license to use
    fn determine_license(&self) -> Option<String> {
        // CLI flag overrides config
        if let Some(license) = &self.args.license {
            if license.to_lowercase() == "none" {
                return None;
            }
            return Some(license.clone());
        }

        // Use config default
        self.config.default_license.clone()
    }

    /// Determine gitignore template to use
    fn determine_gitignore(&self) -> Option<String> {
        // CLI flag overrides config
        if let Some(gitignore) = &self.args.gitignore {
            if gitignore.to_lowercase() == "none" {
                return None;
            }
            return Some(gitignore.clone());
        }

        // Use config default
        self.config.default_gitignore.clone()
    }

    /// Display the configuration being used
    fn display_config(
        &self,
        is_private: &bool,
        license: &Option<String>,
        gitignore: &Option<String>,
    ) {
        println!("\n📋 Repository Configuration:");
        println!("   Name: {}", self.args.name);

        if let Some(desc) = &self.args.description {
            println!("   Description: {}", desc);
        }

        println!(
            "   Visibility: {}",
            if *is_private {
                "Private 🔒"
            } else {
                "Public 🌍"
            }
        );

        if let Some(lic) = license {
            println!("   License: {}", lic);
        }

        if let Some(gi) = gitignore {
            println!("   .gitignore: {}", gi);
        }

        println!(
            "   Initialize with README: {}",
            if self.args.readme { "Yes" } else { "No" }
        );
    }

    /// Call GitHub API to create repository
    fn call_github_api(
        &self,
        token: &str,
        request: &CreateRepoRequest,
    ) -> Result<CreateRepoResponse> {
        let client = Client::new();

        let response = client
            .post("https://api.github.com/user/repos")
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "repogen-cli")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&request)
            .send()
            .context("Failed to send request to GitHub API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(anyhow!("GitHub API error ({}): {}", status, error_text));
        }

        let repo: CreateRepoResponse = response
            .json()
            .context("Failed to parse GitHub API response")?;

        Ok(repo)
    }

    /// Display success message
    fn display_success(&self, response: &CreateRepoResponse) {
        let green = Style::new().green().bold();
        let cyan = Style::new().cyan();

        println!(
            "\n{}",
            green.apply_to("✅ Repository created successfully!")
        );
        println!("\n{}", cyan.apply_to("📍 Repository Details:"));
        println!("   Name: {}", response.full_name);
        println!("   URL: {}", response.html_url);
        println!(
            "   Visibility: {}",
            if response.private {
                "Private 🔒"
            } else {
                "Public 🌍"
            }
        );

        println!("\n{}", cyan.apply_to("🔗 Clone URLs:"));
        println!("   HTTPS: {}", response.clone_url);
        println!("   SSH:   {}", response.ssh_url);

        println!("\n{}", cyan.apply_to("💡 Next Steps:"));
        println!("   git clone {}", response.clone_url);
        println!("   cd {}", response.name);

        if let Some(editor) = &self.config.preferred_editor {
            match editor.as_str() {
                "VS Code" => println!("   code ."),
                "Vim" => println!("   vim ."),
                "Emacs" => println!("   emacs ."),
                "Sublime Text" => println!("   subl ."),
                _ => {}
            }
        }
    }

    /// Clone the repository to the configured directory
    fn clone_repository(&self, response: &CreateRepoResponse) -> Result<()> {
        use std::process::Command;
        use std::env;
        use std::path::PathBuf;

        let cyan = Style::new().cyan().bold();
        let green = Style::new().green().bold();

        println!("\n{}", cyan.apply_to("📥 Cloning repository..."));

        // Determine target directory
        let target_dir = if let Some(ref dir) = self.config.clone_directory {
            PathBuf::from(dir)
        } else {
            env::current_dir().context("Failed to get current directory")?
        };

        // Ensure target directory exists
        if !target_dir.exists() {
            std::fs::create_dir_all(&target_dir)
                .context(format!("Failed to create directory: {:?}", target_dir))?;
        }

        // Run git clone
        let output = Command::new("git")
            .arg("clone")
            .arg(&response.clone_url)
            .current_dir(&target_dir)
            .output()
            .context("Failed to execute git clone. Is git installed?")?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Git clone failed: {}", error_msg));
        }

        // Determine final repository path
        let repo_path = target_dir.join(&response.name);
        let repo_path_str = repo_path.display().to_string();

        println!(
            "{} Repository cloned to: {}",
            green.apply_to("✅"),
            cyan.apply_to(&repo_path_str)
        );

        println!("\n{}", cyan.apply_to("💡 Navigate to your repository:"));
        println!("   cd {}", repo_path_str);

        Ok(())
    }
}
