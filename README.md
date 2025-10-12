# 🧰 repogen

**repogen** is a Rust-based command-line tool that automates the creation of new GitHub repositories — both remotely and locally.  
Instead of manually creating a repository on GitHub and then linking it with `git init`, `repogen` handles everything in one go.

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

## 💻 Example Usage

### 1️⃣ Init

```bash
repogen init
```

Prompts for a GitHub token and saves it securely in your local config.

---

### 2️⃣ Create a New Repository

```bash
repogen new my-cool-project --private --desc "Testing my Rust CLI"
```

**repogen** will:

- Create a new private repository on GitHub
- Clone it into `./my-cool-project`
- Optionally initialize a README and push the first commit

Output:

```
📦 Creating new GitHub repo 'my-cool-project' ...
📦 Cloning git@github.com:username/my-cool-project.git ...
✅ Repo 'my-cool-project' ready at https://github.com/username/my-cool-project
```

---

## 🧭 Command Overview

| Command              | Description                              |
| -------------------- | ---------------------------------------- |
| `repogen init`       | Authenticate and store GitHub token      |
| `repogen new <name>` | Create and clone a new GitHub repo       |
| `repogen config`     | View or edit stored configuration        |
| `repogen whoami`     | Display the connected GitHub user        |
| `repogen link`       | Link an existing folder to a GitHub repo |

---

## ⚙️ Configuration

**repogen** stores its config and authentication token in:

```
~/.config/repogen/
```

- `token` — your GitHub access token
- `config.toml` — optional user settings (future features)

For security, tokens are stored in plain text initially but can be encrypted via the system keyring in later versions.

---

## 🔐 Authentication Options

| Method                          | Description                                                               |
| ------------------------------- | ------------------------------------------------------------------------- |
| **Personal Access Token (PAT)** | Easiest method — user generates it on GitHub and provides it once         |
| **OAuth Device Flow**           | Future option — user logs in via browser, token is returned automatically |

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

| Tool                   | Purpose                              |
| ---------------------- | ------------------------------------ |
| **Rust**               | Core language — safe, fast, portable |
| **clap**               | Command-line argument parsing        |
| **reqwest**            | HTTP client for GitHub API           |
| **serde / serde_json** | JSON parsing                         |
| **dirs / toml**        | Config management                    |
| **anyhow**             | Error handling                       |
| **git2 (future)**      | Native Git operations                |

---

## 🔮 Future Roadmap

- [ ] OAuth device flow init
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

```

---
```
