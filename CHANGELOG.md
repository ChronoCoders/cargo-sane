# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-11-17

### Added
- **Security vulnerability scanning** (`cargo sane health`)
  - Check dependencies against known CVEs and security advisories
  - Color-coded severity levels (Critical/High/Medium/Low)
  - JSON output support for CI/CD integration (`--json` flag)
  - Shows patched versions and advisory URLs
  - RustSec-style advisory database

- **Unused dependency detection** (`cargo sane clean`)
  - Analyzes source code to find actually-used dependencies
  - Identifies and removes unused crates from Cargo.toml
  - Dry-run mode to preview changes (`--dry-run` flag)
  - Handles use statements, extern crate, and macro invocations
  - Preserves formatting and creates automatic backups

- **Dependency conflict resolution** (`cargo sane fix`)
  - Analyzes dependency tree for version conflicts using `cargo tree`
  - Identifies duplicate package versions
  - Suggests optimal version resolutions
  - Auto-fix mode via `cargo update` (`--auto` flag)
  - Interactive conflict resolution workflow

- **Configuration file system**
  - Load settings from `.cargo-sane.toml` (project) or `~/.config/cargo-sane/config.toml` (global)
  - Configurable options: `auto_update_patch`, `auto_update_minor`, `ignore_crates`
  - Additional settings: `verbose`, `create_backups`, `check_security`
  - Partial config loading with smart defaults
  - Sample configuration generation

- **Comprehensive test suite**
  - 31 tests covering all commands and functionality
  - Integration tests for CLI behavior
  - Unit tests for core functionality
  - Test fixtures and utilities

- **Dependency removal support**
  - New `remove_dependency()` method in DependencyUpdater
  - Handles both simple and detailed dependency formats
  - Preserves file formatting during removal

### Changed
- All 5 commands now fully implemented (check, update, fix, clean, health)
- Improved code organization with complete module implementations
- Enhanced error handling across all new features
- Better progress indicators for long-running operations

### Fixed
- Empty module files now contain full implementations
- Configuration system now properly loads and saves

## [0.2.0] - 2025-01-26

### Added
- Interactive dependency update command (`cargo sane update`)
- Automatic backup system (creates `Cargo.toml.backup`)
- Multi-select TUI for choosing which dependencies to update
- `--all` flag to update all dependencies without prompting
- `--dry-run` flag to preview changes
- Support for all Cargo.toml formats (simple, detailed, optional dependencies)
- Format and comment preservation during updates

### Changed
- Improved error messages
- Better progress indicators

## [0.1.0] - 2025-01-25

### Added
- Initial release
- `cargo sane check` command to analyze dependencies
- Update categorization (patch/minor/major)
- Colored terminal output
- Progress bars for API calls
- Support for crates.io dependency checking
- `--verbose` flag for detailed output