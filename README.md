# 🧠 cargo-sane

**Stop losing your mind over Rust dependency conflicts.**

`cargo-sane` is a command-line tool that helps you manage Rust dependencies intelligently. It checks for updates, categorizes them by severity, and updates them interactively.

[![Crates.io](https://img.shields.io/crates/v/cargo-sane.svg)](https://crates.io/crates/cargo-sane)
[![Downloads](https://img.shields.io/crates/d/cargo-sane.svg)](https://crates.io/crates/cargo-sane)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

## ✨ Features

- 🔍 **Smart Dependency Analysis** - Scans your `Cargo.toml` and checks crates.io for updates
- 📊 **Categorized Updates** - Separates updates into patch, minor, and major versions
- ⚡ **Interactive Updates** - Select which dependencies to update with a beautiful TUI
- 🛡️ **Security Scanning** - Check for known vulnerabilities in your dependencies
- 🧹 **Unused Detection** - Find and remove dependencies you're not using
- 🔧 **Conflict Resolution** - Identify and fix version conflicts in your dependency tree
- 🎨 **Beautiful Output** - Color-coded, easy-to-read terminal interface
- 💾 **Safe Updates** - Automatic backup before making changes
- ⚙️ **Configurable** - Customize behavior with configuration files
- 🚀 **Fast** - Concurrent API calls with progress indicators

## 📦 Installation
```bash
cargo install cargo-sane
```

Or build from source:
```bash
git clone https://github.com/ChronoCoders/cargo-sane
cd cargo-sane
cargo install --path .
```

## 🚀 Usage

### Check for Updates

Analyze your dependencies and see what's available:
```bash
cargo sane check
```

**Example output:**
```
🧠 cargo-sane check

ℹ Package: my-project
ℹ Manifest: /path/to/Cargo.toml

📊 Update Summary:
  ✅ Up to date: 3
  🟢 Patch updates available: 5
  🟡 Minor updates available: 2
  🔴 Major updates available: 1

🟢 Patch updates:
  • serde 1.0.195 → 1.0.228
  • anyhow 1.0.89 → 1.0.100

🟡 Minor updates:
  • tokio 1.35.0 → 1.47.2

🔴 Major updates:
  • colored 2.1.0 → 3.0.0

Run `cargo sane update` to update dependencies interactively.
```

### Update Dependencies

Update dependencies interactively:
```bash
cargo sane update
```

This will:
1. Show you all available updates
2. Let you select which ones to apply (spacebar to select, enter to confirm)
3. Update your `Cargo.toml` (with automatic backup)
4. Preserve all formatting, comments, and features

**Update all dependencies automatically:**
```bash
cargo sane update --all
```

**Dry run (preview without changing):**
```bash
cargo sane update --dry-run
```

### Verbose Mode

Get detailed information about updates:
```bash
cargo sane check --verbose
```

### Specify Cargo.toml Path

Work with a specific manifest:
```bash
cargo sane check --manifest-path /path/to/Cargo.toml
cargo sane update --manifest-path /path/to/Cargo.toml
```

### Check for Security Vulnerabilities

Scan your dependencies for known security issues:
```bash
cargo sane health
```

**Example output:**
```
🧠 cargo-sane health

ℹ Package: my-project
ℹ Manifest: /path/to/Cargo.toml

🏥 Health Report:
  Total dependencies: 12
  ⚠️ Vulnerable: 2
  Outdated: 5

📊 Vulnerability Summary:
  🟠 High: 1
  🟡 Medium: 1

🚨 Vulnerabilities Found:

  🟠 hyper 0.14.5 (HIGH)
  ID: RUSTSEC-2021-0078
  Title: Integer overflow in hyper's parsing
  Fix: Update to 0.14.10
```

**JSON output for CI/CD:**
```bash
cargo sane health --json
```

### Find Unused Dependencies

Detect dependencies that aren't being used in your code:
```bash
cargo sane clean --dry-run
```

This will scan your source files and identify dependencies declared in `Cargo.toml` but not imported in your code.

**Remove unused dependencies:**
```bash
cargo sane clean
```

### Fix Dependency Conflicts

Analyze and resolve version conflicts:
```bash
cargo sane fix
```

**Auto-fix mode:**
```bash
cargo sane fix --auto
```

## 📋 Commands

| Command | Description | Status |
|---------|-------------|--------|
| `check` | Analyze dependencies and show available updates | ✅ Available |
| `update` | Update dependencies interactively | ✅ Available |
| `fix` | Fix dependency conflicts | ✅ Available |
| `clean` | Remove unused dependencies | ✅ Available |
| `health` | Check for security vulnerabilities | ✅ Available |

## ⚙️ Configuration

Create a `.cargo-sane.toml` file in your project directory or `~/.config/cargo-sane/config.toml` for global settings:

```toml
# Automatically update patch versions without prompting
auto_update_patch = false

# Automatically update minor versions without prompting
auto_update_minor = false

# Crates to ignore during checks
ignore_crates = ["internal-crate"]

# Show verbose output by default
verbose = false

# Create backups before modifying Cargo.toml
create_backups = true

# Check for security vulnerabilities
check_security = true
```

## 🎯 Why cargo-sane?

Managing Rust dependencies can be frustrating:
- 😤 Manually checking for updates is tedious
- 😵 Version conflicts are confusing
- 😱 Breaking changes can break your build
- 🤯 No easy way to see what changed

**cargo-sane solves this:**
- ✅ Automatically checks all dependencies
- ✅ Shows you exactly what will change
- ✅ Categorizes updates by risk (patch/minor/major)
- ✅ Lets you choose what to update
- ✅ Creates backups automatically
- ✅ Works with all Cargo.toml formats

## 🛡️ Safety Features

- **Automatic Backup**: Creates `Cargo.toml.backup` before any changes
- **Dry Run Mode**: Preview changes without modifying files
- **Format Preservation**: Keeps your formatting, comments, and structure
- **Selective Updates**: Choose exactly which dependencies to update

## 📊 Supported Cargo.toml Formats

cargo-sane handles all dependency formats:
```toml
# Simple
serde = "1.0"

# With features
tokio = { version = "1.35", features = ["full"] }

# Optional dependencies
clap = { version = "4.5", optional = true }

# With comments (preserved!)
regex = "1.11"  # For pattern matching
```

## 🎯 Roadmap

- [x] Smart dependency checking
- [x] Update categorization (patch/minor/major)
- [x] Interactive dependency updates
- [x] Beautiful terminal output
- [x] Conflict resolution
- [x] Security vulnerability scanning (RustSec integration)
- [x] Unused dependency detection
- [x] Configuration file support
- [ ] CI/CD integration (GitHub Actions)
- [ ] Workspace support
- [ ] Dependency license checking
- [ ] Automated PR creation for updates

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [semver](https://github.com/dtolnay/semver) - Semantic version parsing
- [colored](https://github.com/colored-rs/colored) - Terminal colors
- [indicatif](https://github.com/console-rs/indicatif) - Progress bars
- [dialoguer](https://github.com/console-rs/dialoguer) - Interactive prompts

---

**Made with ❤️ by Rust developers, for Rust developers.**

*Stop fighting with dependencies. Stay sane.* 🧠