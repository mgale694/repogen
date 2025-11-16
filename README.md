# 🧰 repogen

**repogen** is a Rust-based command-line tool that automates the creation of new GitHub repositories — both remotely and locally.  
OAuth provides the **best user experience** with fully interactive setup:

1. Run `repogen init` or `repogen init --auth`
2. Select **"OAuth Login (Browser)"** (default option)
3. Follow the interactive guide to create a GitHub OAuth App (~2 minutes, one-time)
4. Enter your OAuth App's Client ID when prompted
5. Authenticate via browser
6. Done! Future authentications just need browser approvalof manually creating a repository on GitHub and then linking it with `git init`, `repogen` handles everything in one go.

---

## 🚀 Overview

Unlike `git init`, which initializes a _local_ repository only, **repogen** is _cloud-first_.  
It connects to your GitHub account, creates a new repository using the GitHub API, and immediately clones it to your local machine.

Think of it as:

> “`git init` — but it already exists on GitHub.”

---

## ✨ Key Features

- 🔐 Secure GitHub authentication (token or OAuth)
- ⚡ Create remote repositories directly from the terminal
- 💻 Automatically clone the repo to your local machine
- 🧾 Optional README, license, and boilerplate generation
- 🧭 Simple, interactive workflow
- 🦀 Fast and portable (written in Rust)

---

## 🧠 How It Works

1. **Authenticate with GitHub**  
   The user logs in using a GitHub Personal Access Token (PAT) or OAuth device flow.  
   The token is securely stored locally in a configuration directory.

2. **Create a New Repository**  
   repogen calls the [GitHub REST API](https://docs.github.com/en/rest/repos/repos#create-a-repository-for-the-authenticated-user)  
   to create a new repo for the authenticated user.

3. **Clone Locally**  
   Once created, repogen clones the repository to your current working directory.

4. **Optional Post-Setup**
   - Add a `README.md`
   - Commit and push defaults
   - Add license, `.gitignore`, or template files

---

## 💻 Quick Start

### 1️⃣ Initialize repogen

```bash
# Full setup (recommended for first time)
repogen init

# Or configure in steps
repogen init --meta  # Set up profile & preferences
repogen init --auth  # Add GitHub authentication
```

### 2️⃣ Create a New Repository

```bash
repogen new my-cool-project --private --desc "My awesome project"
```

**repogen** will:

- Create a new private repository on GitHub
- Clone it into `./my-cool-project`
- Apply your default settings (license, .gitignore, etc.)

Output:

```
📦 Creating new GitHub repo 'my-cool-project' ...
📦 Cloning git@github.com:username/my-cool-project.git ...
✅ Repo 'my-cool-project' ready at https://github.com/username/my-cool-project
```

---

## 🧭 Command Overview

| Command                 | Description                                            |
| ----------------------- | ------------------------------------------------------ |
| `repogen init`          | Full setup: profile, preferences, and authentication   |
| `repogen init --auth`   | Configure GitHub authentication only                   |
| `repogen init --meta`   | Configure profile and preferences only                 |
| `repogen new <name>`    | Create and clone a new GitHub repo                     |
| `repogen config --view` | View current configuration                             |
| `repogen config --edit` | Edit configuration interactively                       |
| `repogen whoami`        | Display the connected GitHub user (coming soon)        |
| `repogen link`          | Link an existing folder to a GitHub repo (coming soon) |

> 📚 For detailed usage of each command, see the [USAGE.md](docs/USAGE.md) documentation.

---

## 📖 Documentation

For detailed usage instructions, examples, and workflows:

- **[Usage Guide](docs/USAGE.md)** - Complete command reference and examples
- **[Docs Index](docs/)** - All documentation in one place

---

## ⚙️ Configuration

**repogen** stores its config and authentication token in:

```
~/.config/repogen/config.toml
```

Example configuration:

```toml
github_token = "ghp_..."
github_username = "yourusername"
user_name = "Your Name"
user_email = "your.email@example.com"
default_private = false
default_license = "MIT"
default_gitignore = "Python"
preferred_editor = "VS Code"
```

For security, tokens are stored in plain text initially but can be encrypted via the system keyring in later versions.

---

## 🔐 Authentication Options

| Method                          | Description                                                                 |
| ------------------------------- | --------------------------------------------------------------------------- |
| **OAuth Device Flow** 🌟        | ✅ **Recommended** — Fully interactive setup with guided OAuth app creation |
| **Personal Access Token (PAT)** | ✅ **Active** — Quick alternative, validated against API automatically      |

### OAuth Device Flow (Recommended) 🌟

OAuth provides the **best user experience** with fully interactive setup:

OAuth provides the best user experience with **fully interactive setup**:

1. Run `repogen init --auth`
2. Select "OAuth Login (Browser)"
3. Follow the interactive guide to create a GitHub OAuth App (~2 minutes, one-time)
4. Enter your OAuth App's Client ID
5. Authenticate via browser
6. Done! Future authentications use your saved Client ID

**Why OAuth?**

- ✅ Browser-based authentication (no copy-pasting tokens)
- ✅ Guided setup process (no manual code editing)
- ✅ Client ID saved to config (reusable forever)
- ✅ Best for teams and organizations
- ✅ More secure token management

### Personal Access Token (Quick Alternative)

PAT is the fastest option for personal use:

1. Run `repogen init --auth`
2. Select **"Personal Access Token"**
3. Create token at https://github.com/settings/tokens/new (`repo`, `user` scopes)
4. Paste when prompted
5. Automatically validated ✅

**When to use PAT:**

- 💨 Quickest setup (30 seconds)
- 👤 Personal projects
- 🔧 Testing or development

See [docs/OAUTH_SETUP.md](docs/OAUTH_SETUP.md) for detailed OAuth information.

---

## 🧩 Architecture Overview

| Component                             | Responsibility                               |
| ------------------------------------- | -------------------------------------------- |
| **CLI (clap)**                        | Parse commands and arguments                 |
| **GitHub Client (reqwest)**           | Handle GitHub API calls                      |
| **Config Handler (dirs, fs)**         | Manage local config and tokens               |
| **Git Integration (git CLI / git2)**  | Clone and initialize repos locally           |
| **Output Layer (console, indicatif)** | Display clean, interactive terminal feedback |

---

## 🦀 Tech Stack

| Tool              | Purpose                              |
| ----------------- | ------------------------------------ |
| **Rust**          | Core language — safe, fast, portable |
| **clap**          | Command-line argument parsing        |
| **dialoguer**     | Interactive CLI prompts and forms    |
| **indicatif**     | Progress bars and spinners           |
| **console**       | Terminal styling and colors          |
| **serde / toml**  | Configuration serialization          |
| **dirs**          | Cross-platform config directories    |
| **anyhow**        | Error handling and context           |
| **reqwest**       | HTTP client for GitHub API & OAuth   |
| **webbrowser**    | Opening browser for OAuth flow       |
| **git2 (future)** | Native Git operations                |

---

## 🔮 Future Roadmap

- [x] OAuth device flow with interactive setup
- [x] Token validation against GitHub API
- [x] Client ID configuration and storage
- [ ] Actual repository creation via GitHub API
- [ ] Secure token storage using system keychain
- [ ] Repo templates (e.g., Python, Node, Rust boilerplates)
- [ ] Organization-level repo creation (`--org my-org`)
- [ ] `.gitignore`, LICENSE, and CI setup options
- [ ] Integration with GitHub Actions (auto-setup workflows)

---

## 💡 Why Use repogen?

Developers spend time repeating the same setup tasks:

- Opening GitHub
- Clicking “New repository”
- Copying clone URLs
- Running `git init`, `git remote add`, `git push`

**repogen** makes that instant and repeatable.

From zero to a ready-to-code GitHub repo in **one command**.

---

## 🧾 License

MIT License © 2025 Matthew Gale

---

## 🧱 Contributing

Contributions are welcome!
If you’d like to help improve **repogen**, feel free to open an issue or submit a pull request.

---

## 📫 Contact

Created by **Matthew Gale**
For feedback or collaboration, reach out via GitHub or LinkedIn.
