# repogen - Detailed Usage Guide

> Complete reference for all repogen commands and workflows

---

## Table of Contents

- [Init Command](#init-command)
- [New Command](#new-command)
- [Config Command](#config-command)
- [Configuration File](#configuration-file)
- [Common Workflows](#common-workflows)

---

## Init Command

The `repogen init` command sets up your repogen configuration. It supports flexible workflows:

### Full Initialization (Recommended for First Time)

```bash
repogen init
```

**What it does:**

1. Collects user profile (GitHub username, name, email)
2. Sets repository preferences (privacy, license, gitignore, editor)
3. Configures GitHub authentication (PAT or OAuth)

**Interactive prompts:**

- GitHub username (required)
- Your full name (optional, for commits)
- Your email (optional, for commits)
- Make repositories private by default?
- Default license for new repositories
- Default .gitignore template
- Preferred editor

### Authentication Only

```bash
repogen init --auth
```

**Use this when you:**

- Only want to update your GitHub token
- Already have profile/preferences configured
- Need to quickly authenticate on a new machine
- Token has expired or needs rotation

**Workflow:**

1. Checks for existing token (offers to keep it)
2. Prompts for authentication method:
   - GitHub Personal Access Token (PAT)
   - OAuth Login (Browser) - _coming soon_
3. Validates and saves the token
4. Suggests running `--meta` if profile is incomplete

**Example:**

```bash
$ repogen init --auth
🔐 repogen - Authentication Setup
Configuring GitHub authentication only.

✔ How would you like to authenticate with GitHub? · GitHub Personal Access Token (PAT)

📝 Using Personal Access Token authentication
✔ Enter your GitHub Personal Access Token · ********
✅ Token received and validated!

✅ GitHub authentication configured successfully!
💡 Your token has been saved to ~/.config/repogen/config.toml
🚀 Run `repogen init --meta` to complete your profile setup.
```

### Profile & Preferences Only

```bash
repogen init --meta
```

**Use this when you:**

- Want to update your profile information
- Need to change default repository settings
- Already have authentication configured
- Want to modify defaults without touching your token

**Workflow:**

1. Shows existing configuration notice
2. Collects user profile information
3. Sets repository default preferences
4. Saves profile and preferences
5. Suggests running `--auth` if token is missing

**Example:**

```bash
$ repogen init --meta
👤 repogen - Profile & Preferences Setup
Configuring your profile and repository preferences.

👤 Step 1: User Profile
✔ GitHub username · yourusername
✔ Your full name (for commits) · Your Name
✔ Your email (optional, for commits) · your.email@example.com

⚙️ Step 2: Default Preferences
✔ Make repositories private by default? · no
✔ Default license for new repositories · MIT
✔ Default .gitignore template · Python
✔ Preferred editor (for opening repos) · VS Code

✅ Profile and preferences configured successfully!
💡 Your settings have been saved to ~/.config/repogen/config.toml
🎉 repogen is fully configured and ready to use!
```

### Help

View all init options:

```bash
repogen init --help
```

---

## New Command

Create a new GitHub repository and clone it locally.

### Basic Usage

```bash
repogen new <repository-name>
```

### Options

| Flag                   | Short | Description                 |
| ---------------------- | ----- | --------------------------- |
| `--desc <description>` | `-d`  | Repository description      |
| `--private`            | `-p`  | Make the repository private |

### Examples

**Create a public repository:**

```bash
repogen new my-project
```

**Create a private repository with description:**

```bash
repogen new my-app --private --desc "My awesome application"
```

**Using short flags:**

```bash
repogen new my-api -p -d "REST API server"
```

### What Happens

1. Creates a new repository on GitHub
2. Applies your default settings:
   - Privacy (from config or `--private` flag)
   - License (from your config)
   - .gitignore template (from your config)
3. Clones the repository to your current directory
4. Optionally opens in your preferred editor

**Expected Output:**

```
📦 Creating new GitHub repo 'my-project' ...
📦 Cloning git@github.com:username/my-project.git ...
✅ Repo 'my-project' ready at https://github.com/username/my-project
```

---

## Config Command

View and manage your repogen configuration.

### View Configuration

```bash
repogen config --view
```

Displays your current configuration settings.

### Edit Configuration

```bash
repogen config --edit
```

Opens an interactive editor to modify your settings.

### Clear Configuration

```bash
repogen config --clear
```

Resets configuration to default values.

### Help

```bash
repogen config --help
```

---

## Configuration File

repogen stores all configuration in: `~/.config/repogen/config.toml`

### Example Configuration

```toml
github_token = "ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
github_username = "yourusername"
user_name = "Your Name"
user_email = "your.email@example.com"
default_private = false
default_license = "MIT"
default_gitignore = "Python"
preferred_editor = "VS Code"
```

### Configuration Fields

| Field               | Type              | Description                                            |
| ------------------- | ----------------- | ------------------------------------------------------ |
| `github_token`      | String (optional) | Your GitHub Personal Access Token                      |
| `github_username`   | String (optional) | Your GitHub username                                   |
| `user_name`         | String (optional) | Your full name for git commits                         |
| `user_email`        | String (optional) | Your email for git commits                             |
| `default_private`   | Boolean           | Create private repos by default                        |
| `default_license`   | String (optional) | Default license (MIT, Apache-2.0, etc.)                |
| `default_gitignore` | String (optional) | Default .gitignore template (Node, Python, Rust, etc.) |
| `preferred_editor`  | String (optional) | Editor to open repos (VS Code, Vim, etc.)              |

### Manual Editing

You can manually edit the config file:

```bash
# macOS/Linux
nano ~/.config/repogen/config.toml

# Or with your preferred editor
code ~/.config/repogen/config.toml
```

---

## Common Workflows

### First-Time Setup

```bash
# Complete setup in one command
repogen init

# Create your first repository
repogen new my-first-project
```

### Separate Setup (Advanced Users)

```bash
# Step 1: Configure profile and preferences
repogen init --meta

# Step 2: Add authentication
repogen init --auth

# Step 3: Create a repository
repogen new my-project
```

### Update Token

```bash
# When your token expires or needs rotation
repogen init --auth
```

### Change Defaults

```bash
# Update default license, gitignore, etc.
repogen init --meta
```

### Quick Repository Creation

```bash
# Public repo with description
repogen new awesome-project --desc "My awesome project"

# Private repo
repogen new secret-project --private

# Private with description
repogen new startup-idea -p -d "The next big thing"
```

### Team Setup

```bash
# Each team member runs
repogen init

# Then anyone can create team repos
repogen new team-project --private
```

---

## Tips & Best Practices

### Security

- **Never commit** your `config.toml` file to version control
- **Rotate tokens regularly** using `repogen init --auth`
- **Use fine-grained tokens** with minimal required permissions

### Productivity

- **Set sensible defaults** to avoid repeating flags
- **Use `--meta`** to quickly update preferences without re-entering token
- **Create aliases** for common commands:
  ```bash
  alias rgnew="repogen new --private"
  ```

### Troubleshooting

**Config not found:**

```bash
# Reinitialize
repogen init
```

**Authentication failed:**

```bash
# Update token
repogen init --auth
```

**View current settings:**

```bash
repogen config --view
```

---

## Getting Help

- Run any command with `--help` flag
- Check the [README](../README.md) for overview
- Report issues on [GitHub](https://github.com/mgale694/repogen/issues)

---

**Happy coding with repogen! 🚀**
