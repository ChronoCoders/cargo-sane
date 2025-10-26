# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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