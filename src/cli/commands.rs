//! Command implementations

use crate::Result;
use crate::core::manifest::Manifest;
use crate::core::dependency::UpdateType;
use crate::analyzer::checker::DependencyChecker;
use crate::cli::output;
use colored::Colorize;

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
    println!("  {} Patch updates available: {}", "🟢".green(), patch_updates.len());
    println!("  {} Minor updates available: {}", "🟡".yellow(), minor_updates.len());
    println!("  {} Major updates available: {}", "🔴".red(), major_updates.len());
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
            println!("  • {} {}", dep.name, dep.current_version.to_string().green());
        }
        println!();
    }

    if patch_updates.is_empty() && minor_updates.is_empty() && major_updates.is_empty() {
        output::print_success("All dependencies are up to date! 🎉");
    } else {
        println!("{}", "Run `cargo sane update` to update dependencies interactively.".dimmed());
    }
    
    Ok(())
}

pub fn update_command(
    manifest_path: Option<String>,
    dry_run: bool,
    all: bool,
) -> Result<()> {
    let _ = (manifest_path, dry_run, all);
    output::print_warning("Update command not yet implemented");
    Ok(())
}

pub fn fix_command(manifest_path: Option<String>, auto: bool) -> Result<()> {
    let _ = (manifest_path, auto);
    output::print_warning("Fix command not yet implemented");
    Ok(())
}

pub fn clean_command(manifest_path: Option<String>, dry_run: bool) -> Result<()> {
    let _ = (manifest_path, dry_run);
    output::print_warning("Clean command not yet implemented");
    Ok(())
}

pub fn health_command(manifest_path: Option<String>, json: bool) -> Result<()> {
    let _ = (manifest_path, json);
    output::print_warning("Health command not yet implemented");
    Ok(())
}