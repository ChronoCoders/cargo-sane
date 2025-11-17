//! Command implementations

use crate::analyzer::checker::DependencyChecker;
use crate::analyzer::conflicts::ConflictDetector;
use crate::analyzer::health::HealthChecker;
use crate::cli::output;
use crate::core::dependency::{Dependency, UpdateType};
use crate::core::manifest::Manifest;
use crate::updater::DependencyUpdater;
use crate::utils::cargo::DependencyUsageAnalyzer;
use crate::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};

pub fn check_command(manifest_path: Option<String>, verbose: bool) -> Result<()> {
    output::print_header("🧠 cargo-sane check");
    println!();

    // Load Cargo.toml
    let manifest = Manifest::find(manifest_path)?;

    if let Some(name) = manifest.package_name() {
        output::print_info(&format!("Package: {}", name));
    }
    output::print_info(&format!("Manifest: {}", manifest.path.display()));
    println!();

    // Check dependencies
    let checker = DependencyChecker::new()?;
    let dependencies = checker.check_dependencies(&manifest)?;

    if dependencies.is_empty() {
        output::print_warning("No dependencies found in Cargo.toml");
        return Ok(());
    }

    // Categorize dependencies
    let mut up_to_date = Vec::new();
    let mut patch_updates = Vec::new();
    let mut minor_updates = Vec::new();
    let mut major_updates = Vec::new();

    for dep in &dependencies {
        match dep.update_type() {
            UpdateType::UpToDate => up_to_date.push(dep),
            UpdateType::Patch => patch_updates.push(dep),
            UpdateType::Minor => minor_updates.push(dep),
            UpdateType::Major => major_updates.push(dep),
        }
    }

    // Print summary
    println!("📊 Update Summary:");
    println!("  {} Up to date: {}", "✅".green(), up_to_date.len());
    println!(
        "  {} Patch updates available: {}",
        "🟢".green(),
        patch_updates.len()
    );
    println!(
        "  {} Minor updates available: {}",
        "🟡".yellow(),
        minor_updates.len()
    );
    println!(
        "  {} Major updates available: {}",
        "🔴".red(),
        major_updates.len()
    );
    println!();

    // Show patch updates
    if !patch_updates.is_empty() {
        println!("{}", "🟢 Patch updates:".green().bold());
        for dep in &patch_updates {
            if let Some(latest) = &dep.latest_version {
                println!(
                    "  • {} {} → {}",
                    dep.name.bold(),
                    dep.current_version.to_string().dimmed(),
                    latest.to_string().green()
                );
                if verbose {
                    println!("    (patch update - likely safe)");
                }
            }
        }
        println!();
    }

    // Show minor updates
    if !minor_updates.is_empty() {
        println!("{}", "🟡 Minor updates:".yellow().bold());
        for dep in &minor_updates {
            if let Some(latest) = &dep.latest_version {
                println!(
                    "  • {} {} → {}",
                    dep.name.bold(),
                    dep.current_version.to_string().dimmed(),
                    latest.to_string().yellow()
                );
                if verbose {
                    println!("    (minor update - should be backwards compatible)");
                }
            }
        }
        println!();
    }

    // Show major updates
    if !major_updates.is_empty() {
        println!("{}", "🔴 Major updates:".red().bold());
        for dep in &major_updates {
            if let Some(latest) = &dep.latest_version {
                println!(
                    "  • {} {} → {}",
                    dep.name.bold(),
                    dep.current_version.to_string().dimmed(),
                    latest.to_string().red()
                );
                if verbose {
                    println!("    (major update - may contain breaking changes)");
                }
            }
        }
        println!();
    }

    // Show up to date if verbose
    if verbose && !up_to_date.is_empty() {
        println!("{}", "✅ Up to date:".green().bold());
        for dep in up_to_date {
            println!(
                "  • {} {}",
                dep.name,
                dep.current_version.to_string().green()
            );
        }
        println!();
    }

    if patch_updates.is_empty() && minor_updates.is_empty() && major_updates.is_empty() {
        output::print_success("All dependencies are up to date! 🎉");
    } else {
        println!(
            "{}",
            "Run `cargo sane update` to update dependencies interactively.".dimmed()
        );
    }

    Ok(())
}

pub fn update_command(manifest_path: Option<String>, dry_run: bool, all: bool) -> Result<()> {
    output::print_header("🧠 cargo-sane update");
    println!();

    // Load Cargo.toml
    let manifest = Manifest::find(manifest_path)?;

    if let Some(name) = manifest.package_name() {
        output::print_info(&format!("Package: {}", name));
    }
    output::print_info(&format!("Manifest: {}", manifest.path.display()));
    println!();

    // Check dependencies
    let checker = DependencyChecker::new()?;
    let dependencies = checker.check_dependencies(&manifest)?;

    // Filter only dependencies with updates
    let updatable: Vec<&Dependency> = dependencies.iter().filter(|d| d.has_update()).collect();

    if updatable.is_empty() {
        output::print_success("All dependencies are up to date! 🎉");
        return Ok(());
    }

    println!(
        "Found {} dependencies with updates available.\n",
        updatable.len()
    );

    // Select which dependencies to update
    let to_update = if all {
        updatable
    } else {
        select_dependencies_to_update(&updatable)?
    };

    if to_update.is_empty() {
        output::print_info("No dependencies selected for update.");
        return Ok(());
    }

    // Show what will be updated
    println!("\n{}", "📝 Updates to apply:".bold());
    for dep in &to_update {
        if let Some(latest) = &dep.latest_version {
            let update_type = match dep.update_type() {
                UpdateType::Patch => "🟢 PATCH",
                UpdateType::Minor => "🟡 MINOR",
                UpdateType::Major => "🔴 MAJOR",
                UpdateType::UpToDate => "✅ UP-TO-DATE",
            };
            println!(
                "  {} {} {} → {}",
                update_type,
                dep.name.bold(),
                dep.current_version.to_string().dimmed(),
                latest.to_string().cyan()
            );
        }
    }
    println!();

    // Confirm unless --all flag is used
    if !all && !dry_run {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Apply these updates?")
            .default(true)
            .interact()?;

        if !confirm {
            output::print_info("Update cancelled.");
            return Ok(());
        }
    }

    if dry_run {
        output::print_info("Dry-run mode: No changes will be made.");
        return Ok(());
    }

    // Create updater
    let mut updater = DependencyUpdater::new(manifest)?;

    // Apply updates
    println!("\n{}", "🔄 Applying updates...".bold());
    for dep in to_update {
        if let Some(latest) = &dep.latest_version {
            match updater.update_dependency(dep, &latest.to_string()) {
                Ok(_) => {
                    println!(
                        "  ✓ Updated {} to {}",
                        dep.name.green(),
                        latest.to_string().cyan()
                    );
                }
                Err(e) => {
                    eprintln!("  ✗ Failed to update {}: {}", dep.name.red(), e);
                }
            }
        }
    }

    // Save changes
    updater.save()?;
    println!();
    output::print_success("Cargo.toml updated successfully!");
    output::print_info("Backup saved as Cargo.toml.backup");
    println!();
    println!(
        "{}",
        "Don't forget to run `cargo check` to verify everything still compiles!".dimmed()
    );

    Ok(())
}

/// Interactive selection of dependencies to update
fn select_dependencies_to_update<'a>(deps: &[&'a Dependency]) -> Result<Vec<&'a Dependency>> {
    let items: Vec<String> = deps
        .iter()
        .map(|d| {
            let update_type = match d.update_type() {
                UpdateType::Patch => "🟢",
                UpdateType::Minor => "🟡",
                UpdateType::Major => "🔴",
                UpdateType::UpToDate => "✅",
            };
            format!(
                "{} {} {} → {}",
                update_type,
                d.name,
                d.current_version,
                d.latest_version.as_ref().unwrap()
            )
        })
        .collect();

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select dependencies to update (Space to select, Enter to confirm)")
        .items(&items)
        .interact()?;

    let selected: Vec<&Dependency> = selections.iter().map(|&i| deps[i]).collect();
    Ok(selected)
}

pub fn fix_command(manifest_path: Option<String>, auto: bool) -> Result<()> {
    output::print_header("🧠 cargo-sane fix");
    println!();

    // Load Cargo.toml
    let manifest = Manifest::find(manifest_path)?;

    if let Some(name) = manifest.package_name() {
        output::print_info(&format!("Package: {}", name));
    }
    output::print_info(&format!("Manifest: {}", manifest.path.display()));
    println!();

    output::print_info("Analyzing dependency tree for conflicts...");
    println!();

    // Detect conflicts
    let detector = ConflictDetector::new();
    let report = detector.detect_conflicts(&manifest)?;

    if !report.has_conflicts {
        output::print_success("No version conflicts detected! 🎉");
        println!();
        println!(
            "{}",
            format!("Total packages in dependency tree: {}", report.total_packages).dimmed()
        );
        return Ok(());
    }

    // Show conflicts
    println!("🔍 Conflict Analysis:");
    println!(
        "  Total packages: {}",
        report.total_packages.to_string().bold()
    );
    println!(
        "  {} Conflicts found: {}",
        "⚠️".yellow(),
        report.conflicts.len().to_string().red().bold()
    );
    println!();

    println!("{}", "📋 Version Conflicts:".yellow().bold());
    for conflict in &report.conflicts {
        println!();
        println!("  {} {}", "📦".cyan(), conflict.package_name.bold());
        println!("    Versions in use:");
        for version in &conflict.versions {
            println!("      • {}", version.yellow());
        }
        if let Some(suggested) = &conflict.suggested_version {
            println!("    Suggested: {}", suggested.green().bold());
        }
    }
    println!();

    // Provide fix suggestions
    println!("{}", "🔧 Recommended Actions:".bold());
    println!();

    let mut has_fixable = false;
    for conflict in &report.conflicts {
        if let Some(suggested) = &conflict.suggested_version {
            has_fixable = true;
            println!(
                "  {} Update to {} {}",
                "•".green(),
                conflict.package_name.bold(),
                suggested.green()
            );
        }
    }

    if !has_fixable {
        println!("  No automatic fixes available.");
        println!();
        output::print_warning("These conflicts are typically caused by transitive dependencies.");
        println!("  Consider:");
        println!("    • Updating your direct dependencies");
        println!("    • Using cargo update to update the lock file");
        println!("    • Checking if newer versions of your dependencies resolve these conflicts");
        return Ok(());
    }

    println!();
    output::print_info("Note: Version conflicts in the dependency tree are often unavoidable.");
    println!("  They occur when different packages depend on different versions of the same crate.");
    println!("  Cargo handles this by compiling multiple versions, which increases binary size.");
    println!();

    if auto {
        output::print_info("Auto-fix mode: Attempting to update dependencies...");
        println!();

        // Try to run cargo update for conflicting packages
        for conflict in &report.conflicts {
            if conflict.suggested_version.is_some() {
                println!("  Updating {}...", conflict.package_name);
                let result = std::process::Command::new("cargo")
                    .arg("update")
                    .arg("-p")
                    .arg(&conflict.package_name)
                    .current_dir(manifest.path.parent().unwrap())
                    .output();

                match result {
                    Ok(output) if output.status.success() => {
                        println!("    ✓ Updated {}", conflict.package_name.green());
                    }
                    _ => {
                        println!("    ✗ Failed to update {}", conflict.package_name.red());
                    }
                }
            }
        }

        println!();
        output::print_success("Fix attempt complete!");
        println!("{}", "Run `cargo sane fix` again to check if conflicts remain.".dimmed());
    } else {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to attempt automatic fixes via `cargo update`?")
            .default(false)
            .interact()?;

        if confirm {
            println!();
            output::print_info("Running cargo update for conflicting packages...");

            for conflict in &report.conflicts {
                if conflict.suggested_version.is_some() {
                    println!("  Updating {}...", conflict.package_name);
                    let result = std::process::Command::new("cargo")
                        .arg("update")
                        .arg("-p")
                        .arg(&conflict.package_name)
                        .current_dir(manifest.path.parent().unwrap())
                        .output();

                    match result {
                        Ok(output) if output.status.success() => {
                            println!("    ✓ Updated {}", conflict.package_name.green());
                        }
                        _ => {
                            println!("    ✗ Failed to update {}", conflict.package_name.red());
                        }
                    }
                }
            }

            println!();
            output::print_success("Fix attempt complete!");
            println!(
                "{}",
                "Run `cargo sane fix` again to check if conflicts remain.".dimmed()
            );
        } else {
            output::print_info("No changes made.");
            println!();
            println!("You can manually fix conflicts by:");
            println!("  1. Updating your dependencies in Cargo.toml");
            println!("  2. Running `cargo update` to refresh the lock file");
            println!("  3. Using `cargo update -p <package>` for specific packages");
        }
    }

    Ok(())
}

pub fn clean_command(manifest_path: Option<String>, dry_run: bool) -> Result<()> {
    output::print_header("🧠 cargo-sane clean");
    println!();

    // Load Cargo.toml
    let manifest = Manifest::find(manifest_path)?;

    if let Some(name) = manifest.package_name() {
        output::print_info(&format!("Package: {}", name));
    }
    output::print_info(&format!("Manifest: {}", manifest.path.display()));
    println!();

    output::print_info("Scanning source files for dependency usage...");

    // Analyze dependency usage
    let analyzer = DependencyUsageAnalyzer::new(&manifest.path)?;
    let declared_deps = manifest.get_dependencies();
    let unused = analyzer.find_unused_dependencies(&declared_deps)?;

    if unused.is_empty() {
        output::print_success("All dependencies are being used! 🎉");
        return Ok(());
    }

    println!(
        "\n{} Found {} potentially unused {}:\n",
        "⚠️".yellow(),
        unused.len().to_string().bold(),
        if unused.len() == 1 {
            "dependency"
        } else {
            "dependencies"
        }
    );

    for dep in &unused {
        println!("  • {}", dep.red());
    }
    println!();

    output::print_warning("Note: This analysis may have false positives for:");
    println!("  - Procedural macros (e.g., serde with derive feature)");
    println!("  - Build dependencies");
    println!("  - Dependencies used only in doc comments");
    println!("  - Dependencies re-exported from other crates");
    println!();

    if dry_run {
        output::print_info("Dry-run mode: No changes will be made.");
        println!();
        println!("To remove these dependencies, you can:");
        for dep in &unused {
            println!("  cargo remove {}", dep);
        }
    } else {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to remove these dependencies from Cargo.toml?")
            .default(false)
            .interact()?;

        if confirm {
            let mut updater = DependencyUpdater::new(manifest)?;
            println!("\n{}", "🗑️  Removing unused dependencies...".bold());

            for dep in &unused {
                match updater.remove_dependency(dep) {
                    Ok(_) => {
                        println!("  ✓ Removed {}", dep.green());
                    }
                    Err(e) => {
                        eprintln!("  ✗ Failed to remove {}: {}", dep.red(), e);
                    }
                }
            }

            updater.save()?;
            println!();
            output::print_success("Cargo.toml updated successfully!");
            output::print_info("Backup saved as Cargo.toml.backup");
            println!();
            println!(
                "{}",
                "Don't forget to run `cargo check` to verify everything still compiles!".dimmed()
            );
        } else {
            output::print_info("No changes made.");
        }
    }

    Ok(())
}

pub fn health_command(manifest_path: Option<String>, json: bool) -> Result<()> {
    if !json {
        output::print_header("🧠 cargo-sane health");
        println!();
    }

    // Load Cargo.toml
    let manifest = Manifest::find(manifest_path)?;

    if !json {
        if let Some(name) = manifest.package_name() {
            output::print_info(&format!("Package: {}", name));
        }
        output::print_info(&format!("Manifest: {}", manifest.path.display()));
        println!();
    }

    // Check dependencies first to get version info
    let checker = DependencyChecker::new()?;
    let dependencies = checker.check_dependencies(&manifest)?;

    if dependencies.is_empty() {
        if json {
            println!("{{\"dependencies\": [], \"vulnerable_count\": 0}}");
        } else {
            output::print_warning("No dependencies found in Cargo.toml");
        }
        return Ok(());
    }

    // Run health check
    let health_checker = HealthChecker::new()?;
    let report = health_checker.check_health(&dependencies)?;

    if json {
        // Output as JSON
        let json_output = serde_json::to_string_pretty(&report)
            .unwrap_or_else(|_| "{}".to_string());
        println!("{}", json_output);
    } else {
        // Print summary
        println!("🏥 Health Report:");
        println!(
            "  Total dependencies: {}",
            report.total_dependencies.to_string().bold()
        );
        println!(
            "  {} Vulnerable: {}",
            if report.vulnerable_count > 0 {
                "⚠️".to_string()
            } else {
                "✅".to_string()
            },
            if report.vulnerable_count > 0 {
                report.vulnerable_count.to_string().red().bold().to_string()
            } else {
                report.vulnerable_count.to_string().green().to_string()
            }
        );
        println!("  Outdated: {}", report.outdated_count);
        println!();

        if report.vulnerable_count > 0 {
            println!("📊 Vulnerability Summary:");
            if report.critical_count > 0 {
                println!(
                    "  {} Critical: {}",
                    "🔴".red(),
                    report.critical_count.to_string().red().bold()
                );
            }
            if report.high_count > 0 {
                println!(
                    "  {} High: {}",
                    "🟠",
                    report.high_count.to_string().red()
                );
            }
            if report.medium_count > 0 {
                println!(
                    "  {} Medium: {}",
                    "🟡",
                    report.medium_count.to_string().yellow()
                );
            }
            if report.low_count > 0 {
                println!(
                    "  {} Low: {}",
                    "🟢",
                    report.low_count.to_string().green()
                );
            }
            println!();

            // Show vulnerable dependencies
            println!("{}", "🚨 Vulnerabilities Found:".red().bold());
            for dep in &report.dependencies {
                if dep.is_vulnerable() {
                    for advisory in &dep.advisories {
                        println!();
                        println!(
                            "  {} {} {} ({})",
                            advisory.severity.emoji(),
                            dep.name.bold(),
                            dep.version.dimmed(),
                            advisory.severity.as_str().red()
                        );
                        println!("  ID: {}", advisory.id.cyan());
                        println!("  Title: {}", advisory.title);
                        if let Some(patched) = &advisory.patched_versions {
                            println!("  Fix: Update to {}", patched.green());
                        }
                        if let Some(url) = &advisory.url {
                            println!("  More info: {}", url.dimmed());
                        }
                    }
                }
            }
            println!();
            output::print_warning("Action required: Update vulnerable dependencies!");
        } else {
            output::print_success("No known vulnerabilities found! 🎉");
        }
    }

    Ok(())
}
