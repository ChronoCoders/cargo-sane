# 🧠 cargo-sane

**Stop losing your mind over Rust dependency conflicts.**

`cargo-sane` is a command-line tool that helps you manage Rust dependencies intelligently. It checks for updates, categorizes them by severity, and (soon) updates them interactively.

## ✨ Features

- 🔍 **Smart Dependency Analysis** - Scans your `Cargo.toml` and checks crates.io for updates
- 📊 **Categorized Updates** - Separates updates into patch, minor, and major versions
- 🎨 **Beautiful Output** - Color-coded, easy-to-read terminal interface
- ⚡ **Fast** - Concurrent API calls with progress indicators
- 🔒 **Safe** - Shows what will change before you commit

## 📦 Installation
```bash
cargo install cargo-sane
```

Or build from source:
```bash
git clone https://github.com/yourusername/cargo-sane
cd cargo-sane
cargo install --path .
```

## 🚀 Usage

### Check for Updates
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

### Verbose Mode
```bash
cargo sane check --verbose
```

Shows additional details about each update type.

### Specify Cargo.toml Path
```bash
cargo sane check --manifest-path /path/to/Cargo.toml
```

## 📋 Commands

- `cargo sane check` - Analyze dependencies and show available updates
- `cargo sane update` - *(Coming soon)* Update dependencies interactively
- `cargo sane fix` - *(Coming soon)* Fix dependency conflicts
- `cargo sane clean` - *(Coming soon)* Remove unused dependencies
- `cargo sane health` - *(Coming soon)* Check for security vulnerabilities

## 🎯 Roadmap

- [x] Smart dependency checking
- [x] Update categorization (patch/minor/major)
- [x] Beautiful terminal output
- [ ] Interactive dependency updates
- [ ] Conflict resolution
- [ ] Security vulnerability scanning (RustSec integration)
- [ ] Unused dependency detection
- [ ] CI/CD integration (GitHub Actions)
- [ ] Configuration file support

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

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
- [colored](https://github.com/mackwic/colored) - Terminal colors
- [indicatif](https://github.com/console-rs/indicatif) - Progress bars

---

Made with ❤️ by Rust developers, for Rust developers.
