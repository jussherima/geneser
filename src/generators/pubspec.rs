use crate::models::options::VersionStrategy;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Packages that come from the Flutter SDK and need `sdk: flutter` syntax.
const SDK_PACKAGES: &[&str] = &["flutter_localizations", "flutter_test", "flutter_driver"];

fn is_sdk_package(pkg: &str) -> bool {
    SDK_PACKAGES.contains(&pkg)
}

pub fn add_dependencies(
    project_dir: &str,
    packages: &[String],
    strategy: &VersionStrategy,
) -> Result<()> {
    if packages.is_empty() {
        return Ok(());
    }

    let pubspec_path = Path::new(project_dir).join("pubspec.yaml");
    let mut pubspec = fs::read_to_string(&pubspec_path).context("Could not read pubspec.yaml")?;

    let deps_marker = "\ndependencies:\n";
    if let Some(pos) = pubspec.find(deps_marker) {
        let mut new_deps = String::new();
        for pkg in packages {
            // Skip if already present
            if pubspec.contains(&format!("\n  {}:", pkg)) {
                continue;
            }
            if is_sdk_package(pkg) {
                new_deps.push_str(&format!("  {}:\n    sdk: flutter\n", pkg));
            } else {
                let version = strategy.version_for(pkg);
                new_deps.push_str(&format!("  {}: {}\n", pkg, version));
            }
        }
        if !new_deps.is_empty() {
            pubspec.insert_str(pos + deps_marker.len(), &new_deps);
            fs::write(&pubspec_path, pubspec)?;
        }
    } else {
        anyhow::bail!("Bloc 'dependencies' introuvable dans pubspec.yaml");
    }

    Ok(())
}

pub fn add_dev_dependencies(
    project_dir: &str,
    packages: &[String],
    strategy: &VersionStrategy,
) -> Result<()> {
    if packages.is_empty() {
        return Ok(());
    }

    let pubspec_path = Path::new(project_dir).join("pubspec.yaml");
    let mut pubspec = fs::read_to_string(&pubspec_path).context("Could not read pubspec.yaml")?;

    let dev_deps_marker = "\ndev_dependencies:\n";
    if let Some(pos) = pubspec.find(dev_deps_marker) {
        let mut new_deps = String::new();
        for pkg in packages {
            if pubspec.contains(&format!("\n  {}:", pkg)) {
                continue;
            }
            if is_sdk_package(pkg) {
                new_deps.push_str(&format!("  {}:\n    sdk: flutter\n", pkg));
            } else {
                let version = strategy.version_for(pkg);
                new_deps.push_str(&format!("  {}: {}\n", pkg, version));
            }
        }
        if !new_deps.is_empty() {
            pubspec.insert_str(pos + dev_deps_marker.len(), &new_deps);
        }
    } else {
        // Add dev_dependencies block at the end
        pubspec.push_str("\ndev_dependencies:\n");
        for pkg in packages {
            if pubspec.contains(&format!("\n  {}:", pkg)) {
                continue;
            }
            if is_sdk_package(pkg) {
                pubspec.push_str(&format!("  {}:\n    sdk: flutter\n", pkg));
            } else {
                let version = strategy.version_for(pkg);
                pubspec.push_str(&format!("  {}: {}\n", pkg, version));
            }
        }
    }

    fs::write(&pubspec_path, pubspec)?;
    Ok(())
}
