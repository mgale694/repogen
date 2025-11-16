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
   - **OAuth Login (Browser)** - Recommended
   - **GitHub Personal Access Token (PAT)** - Quick alternative
3. Validates and saves the token
4. Suggests running `--meta` if profile is incomplete

#### OAuth Authentication (Recommended)

OAuth provides browser-based authentication with one-time setup:

**First Time Setup:**

```bash
$ repogen init --auth

✔ How would you like to authenticate with GitHub? › OAuth Login (Browser) - Recommended

🌐 OAuth Browser Authentication

� OAuth Setup Required
This is a one-time setup that takes about 2 minutes.

✔ Would you like to set up OAuth now? › yes

📝 OAuth App Setup Guide
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Step 1: Create a GitHub OAuth App
   → Open: https://github.com/settings/developers
✅ Browser opened automatically

Step 2: Click 'New OAuth App'

Step 3: Fill in the application details:
   • Application name: repogen
   • Homepage URL: https://github.com/yourusername/repogen
   • Authorization callback URL: http://127.0.0.1
   • Description: CLI tool for GitHub repository creation

Step 4: After creating the app:
   • Check the box: ☑️  Enable Device Flow
   • Copy the Client ID (starts with 'Iv1.')

✔ GitHub OAuth App Client ID › Iv1.abc123def456

✅ Client ID saved to config!
🎉 OAuth setup complete! Now let's authenticate...

📝 Requesting device code from GitHub...

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃  Please visit: https://github.com/login/device  ┃
┃  And enter code: WDJB-MJHT           ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

✅ Authorization successful!
✅ GitHub authentication configured successfully!
```

**Returning Users:** After setup, OAuth is seamless - just approve in browser!

💡 See [OAUTH_SETUP.md](OAUTH_SETUP.md) for detailed information.

#### Personal Access Token (Quick Alternative)

**Example:**

```bash
$ repogen init --auth

✔ How would you like to authenticate with GitHub? › GitHub Personal Access Token (PAT)

📝 Using Personal Access Token authentication
💡 Create a token at: https://github.com/settings/tokens/new
   Required scopes: repo, user

✔ Enter your GitHub Personal Access Token · ********
🔍 Validating token with GitHub... ✅ Success!
👤 Authenticated as: yourusername

✅ GitHub authentication configured successfully!
💡 Your token has been saved to ~/.config/repogen/config.toml
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

View and manage your repogen configuration. The config command provides three powerful ways to manage your settings.

### View Configuration

Display your current configuration in a beautifully formatted way.

```bash
# View with explicit flag
repogen config --view

# Or use default behavior (no flag = view)
repogen config
```

**What you'll see:**

```
📋 repogen Configuration
══════════════════════════════════════════════════

👤 User Profile
  GitHub Username: yourusername
  Full Name: Your Name
  Email: your.email@example.com

🔐 Authentication
  GitHub Token: ghp_1234***

⚙️  Repository Defaults
  Private by default: No
  Default License: MIT
  Default .gitignore: Python
  Preferred Editor: VS Code

📁 Configuration File
  Location: /Users/you/.config/repogen/config.toml

══════════════════════════════════════════════════

💡 Run repogen config --edit to modify configuration
💡 Run repogen config --clear to reset configuration
```

**Features:**

- ✅ Color-coded sections for easy reading
- ✅ Masked token display (shows only first 8 characters)
- ✅ Clear indication of unset values
- ✅ Shows config file location
- ✅ Helpful tips for next actions

### Edit Configuration

Interactively modify your configuration settings without re-entering everything.

```bash
repogen config --edit
```

**Interactive Menu:**

```
✏️  Edit Configuration
Select what you'd like to edit:

> User Profile (username, name, email)
  Repository Defaults (privacy, license, gitignore, editor)
  GitHub Authentication (token)
  Edit All
  Cancel
```

**Edit Options:**

1. **User Profile** - Update username, name, email
   - Current values shown as defaults
   - Just press Enter to keep existing values
2. **Repository Defaults** - Update default settings

   - Privacy (public/private repos by default)
   - License (MIT, Apache-2.0, GPL-3.0, BSD-3-Clause, Unlicense, or None)
   - .gitignore template (Node, Python, Rust, Go, Java, C++, Swift, or None)
   - Editor (VS Code, Vim, Emacs, Sublime Text, Atom, IntelliJ, or None)

3. **GitHub Authentication** - Secure token update

   - Recommends using `repogen init --auth` for security
   - Ensures proper validation and secure handling

4. **Edit All** - Update profile and repository defaults in one go

5. **Cancel** - Exit without making changes

**Example Session:**

```bash
$ repogen config --edit
✔ What would you like to edit? · Repository Defaults

⚙️  Edit Repository Defaults
✔ Make repositories private by default? · yes
✔ Default license · MIT
✔ Default .gitignore template · Python
✔ Preferred editor · VS Code

✅ Configuration updated successfully!
💡 Run repogen config --view to view your updated config
```

**Benefits:**

- ✅ Selective editing - only update what you need
- ✅ Current values shown as defaults - no retyping
- ✅ Secure token handling - redirects to proper auth flow
- ✅ Auto-saves after editing
- ✅ Helpful suggestions for next steps

### Clear Configuration

Reset your configuration to defaults. This completely removes your config file.

```bash
repogen config --clear
```

**Safety Features:**

```
🗑️  Clear Configuration
This will reset all configuration to default values.
⚠️  This action cannot be undone!

? Are you sure you want to clear all configuration? (y/N)

? Really clear? This will delete your GitHub token and all settings! (y/N)
```

**Features:**

- ✅ Double confirmation required
- ✅ Clear warning messages
- ✅ Deletes config file completely
- ✅ Helpful message after clearing
- ✅ Safe cancellation at any point

**After clearing:**

```
✅ Configuration cleared successfully!
💡 Run repogen init to set up again
```

### Help

View all config command options:

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
oauth_client_id = "Iv1.abc123def456"
```

### Configuration Fields

| Field               | Type              | Description                                            |
| ------------------- | ----------------- | ------------------------------------------------------ |
| `github_token`      | String (optional) | Your GitHub Personal Access Token or OAuth token       |
| `github_username`   | String (optional) | Your GitHub username                                   |
| `user_name`         | String (optional) | Your full name for git commits                         |
| `user_email`        | String (optional) | Your email for git commits                             |
| `default_private`   | Boolean           | Create private repos by default                        |
| `default_license`   | String (optional) | Default license (MIT, Apache-2.0, etc.)                |
| `default_gitignore` | String (optional) | Default .gitignore template (Node, Python, Rust, etc.) |
| `preferred_editor`  | String (optional) | Editor to open repos (VS Code, Vim, etc.)              |
| `oauth_client_id`   | String (optional) | GitHub OAuth App Client ID for OAuth authentication    |

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
