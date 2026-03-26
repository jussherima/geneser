use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn add_dependencies(project_dir: &str, packages: &[String]) -> Result<()> {
    if packages.is_empty() {
        return Ok(());
    }

    let pubspec_path = Path::new(project_dir).join("pubspec.yaml");
    let mut pubspec = fs::read_to_string(&pubspec_path).context("Could not read pubspec.yaml")?;

    let deps_marker = "dependencies:\n";
    if let Some(pos) = pubspec.find(deps_marker) {
        let mut new_deps = String::new();
        for pkg in packages {
            new_deps.push_str(&format!("  {}: any\n", pkg));
        }
        pubspec.insert_str(pos + deps_marker.len(), &new_deps);
        fs::write(&pubspec_path, pubspec)?;
    } else {
        anyhow::bail!("Could not find dependencies block in pubspec.yaml");
    }

    Ok(())
}

pub fn add_dev_dependencies(project_dir: &str, packages: &[String]) -> Result<()> {
    if packages.is_empty() {
        return Ok(());
    }

    let pubspec_path = Path::new(project_dir).join("pubspec.yaml");
    let mut pubspec = fs::read_to_string(&pubspec_path).context("Could not read pubspec.yaml")?;

    let dev_deps_marker = "dev_dependencies:\n";
    if let Some(pos) = pubspec.find(dev_deps_marker) {
        let mut new_deps = String::new();
        for pkg in packages {
            new_deps.push_str(&format!("  {}: any\n", pkg));
        }
        pubspec.insert_str(pos + dev_deps_marker.len(), &new_deps);
    } else {
        // Add dev_dependencies block at the end
        pubspec.push_str("\ndev_dependencies:\n");
        for pkg in packages {
            pubspec.push_str(&format!("  {}: any\n", pkg));
        }
    }

    fs::write(&pubspec_path, pubspec)?;
    Ok(())
}
