# Release Notes: cargo-sane v0.3.0

**Release Date:** November 17, 2025

🎉 **Major Feature Release** - All planned commands are now fully implemented!

---

## 🚀 Highlights

This release marks a significant milestone for cargo-sane, bringing the project from 40% to **100% feature complete**. All five planned commands are now fully functional with comprehensive test coverage.

### New Commands

#### 🛡️ Security Vulnerability Scanning (`cargo sane health`)
Protect your project from known security vulnerabilities with built-in advisory scanning.

```bash
cargo sane health
```

**Features:**
- Checks dependencies against RustSec-style advisory database
- Color-coded severity levels (🔴 Critical, 🟠 High, 🟡 Medium, 🟢 Low)
- Shows affected versions and patched versions
- Links to advisory details
- JSON output for CI/CD integration (`--json` flag)

**Example:**
```
🏥 Health Report:
  Total dependencies: 12
  ⚠️ Vulnerable: 2

🚨 Vulnerabilities Found:
  🟠 hyper 0.14.5 (HIGH)
  ID: RUSTSEC-2021-0078
  Fix: Update to 0.14.10
```

---

#### 🧹 Unused Dependency Detection (`cargo sane clean`)
Keep your Cargo.toml lean by removing dependencies you're not actually using.

```bash
cargo sane clean --dry-run
```

**Features:**
- Scans all source files (`src/`, `tests/`, `examples/`)
- Detects `use` statements, `extern crate`, and macro invocations
- Handles crate name normalization (hyphens ↔ underscores)
- Dry-run mode for safe preview
- Automatic backup before removal
- Interactive confirmation dialog

---

#### 🔧 Dependency Conflict Resolution (`cargo sane fix`)
Identify and resolve version conflicts in your dependency tree.

```bash
cargo sane fix
```

**Features:**
- Analyzes dependency tree using `cargo tree --duplicates`
- Identifies packages with multiple versions
- Suggests optimal version to use
- Auto-fix mode via `cargo update` (`--auto` flag)
- Interactive workflow with confirmation
- Explains conflict causes and solutions

---

## ⚙️ Configuration System

Customize cargo-sane behavior with configuration files:

**Locations (in priority order):**
1. `.cargo-sane.toml` (project directory)
2. `~/.config/cargo-sane/config.toml` (global)
3. `~/.cargo-sane.toml` (global alternative)

**Available Settings:**
```toml
auto_update_patch = false    # Auto-approve patch updates
auto_update_minor = false    # Auto-approve minor updates
ignore_crates = []           # Crates to skip during checks
verbose = false              # Show detailed output
create_backups = true        # Backup before modifications
check_security = true        # Enable security checks
```

---

## 📊 Statistics

| Metric | v0.2.0 | v0.3.0 | Change |
|--------|--------|--------|--------|
| Commands Implemented | 2/5 | 5/5 | +60% |
| Lines of Code | 959 | 2,521+ | +163% |
| Test Count | 2 | 31 | +1,450% |
| Test Coverage | ~5% | ~75% | +1,400% |

---

## 📦 What's New

### Added
- **Health command** - Security vulnerability scanning with advisory database
- **Clean command** - Unused dependency detection and removal
- **Fix command** - Dependency conflict analysis and resolution
- **Configuration system** - Persistent settings with TOML files
- **31 comprehensive tests** - Unit and integration tests for all features
- **Dependency removal** - New `remove_dependency()` API

### Changed
- All placeholder modules now have full implementations
- Improved error handling with detailed context
- Enhanced progress indicators for all operations
- Better code organization and modularity

### Fixed
- Empty module stubs replaced with complete implementations
- Configuration system now properly loads partial configs

---

## ⬆️ Upgrading

```bash
cargo install cargo-sane --force
```

Or if you prefer building from source:

```bash
git clone https://github.com/ChronoCoders/cargo-sane
cd cargo-sane
cargo install --path .
```

---

## 🔜 What's Next

Future releases will focus on:
- [ ] CI/CD integration (GitHub Actions templates)
- [ ] Workspace support for monorepos
- [ ] Dependency license checking
- [ ] Automated PR creation for updates
- [ ] Real-time RustSec database updates

---

## 🙏 Acknowledgments

Thank you to all contributors and users who provided feedback. This release represents a major step forward in making Rust dependency management simpler and safer.

---

## 📝 Full Changelog

See [CHANGELOG.md](CHANGELOG.md) for the complete list of changes.

---

**Stay sane with your dependencies! 🧠**
