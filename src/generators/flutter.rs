use anyhow::{Context, Result};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum FlutterRunner {
    Fvm,
    Flutter,
    None,
}

/// Detect whether `fvm` or `flutter` is available in PATH.
pub fn detect_flutter_runner() -> FlutterRunner {
    if Command::new("fvm")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return FlutterRunner::Fvm;
    }
    if Command::new("flutter")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return FlutterRunner::Flutter;
    }
    FlutterRunner::None
}

/// Parse `fvm releases` to list stable Flutter versions.
pub fn fvm_list_versions() -> Result<Vec<String>> {
    let output = Command::new("fvm")
        .arg("releases")
        .output()
        .context("Failed to run fvm releases")?;

    if !output.status.success() {
        anyhow::bail!(
            "fvm releases failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut versions: Vec<String> = Vec::new();
    let mut in_stable = false;

    for line in stdout.lines() {
        let trimmed = line.trim();
        // FVM outputs sections like "Channels:", "Releases:", or version lines
        // We look for lines that contain version-like patterns (e.g. 3.27.4)
        if trimmed.contains("stable") && !trimmed.starts_with(|c: char| c.is_ascii_digit()) {
            in_stable = true;
            continue;
        }
        if in_stable {
            // Stop at next section header or empty block
            if trimmed.is_empty()
                || (trimmed.contains("beta") && !trimmed.starts_with(|c: char| c.is_ascii_digit()))
                || (trimmed.contains("dev") && !trimmed.starts_with(|c: char| c.is_ascii_digit()))
            {
                break;
            }
        }
        // Parse version from each line — extract first token that looks like a version
        if let Some(ver) = extract_version(trimmed) {
            if in_stable || versions.is_empty() {
                versions.push(ver);
            }
        }
    }

    // If section parsing failed, fallback: grab all version-like strings
    if versions.is_empty() {
        for line in stdout.lines() {
            if let Some(ver) = extract_version(line.trim()) {
                versions.push(ver);
            }
        }
    }

    // Most recent first
    versions.reverse();
    // Limit to reasonable number
    versions.truncate(20);
    Ok(versions)
}

fn extract_version(s: &str) -> Option<String> {
    for token in s.split_whitespace() {
        let clean = token.trim_matches(|c: char| !c.is_ascii_digit() && c != '.');
        let parts: Vec<&str> = clean.split('.').collect();
        if parts.len() >= 2 && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit())) {
            return Some(clean.to_string());
        }
    }
    None
}

/// List locally installed FVM Flutter versions (`fvm list`).
pub fn fvm_list_local() -> Result<Vec<String>> {
    let output = Command::new("fvm")
        .arg("list")
        .output()
        .context("Failed to run fvm list")?;

    if !output.status.success() {
        anyhow::bail!(
            "fvm list failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut versions: Vec<String> = Vec::new();
    for line in stdout.lines() {
        if let Some(ver) = extract_version(line.trim()) {
            versions.push(ver);
        }
    }
    versions.reverse();
    Ok(versions)
}

/// Install a Flutter version via FVM.
pub fn fvm_install(version: &str) -> Result<()> {
    let pb = make_spinner();
    pb.set_message(format!("Installing Flutter {} via FVM...", version));
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let output = Command::new("fvm")
        .arg("install")
        .arg(version)
        .output()
        .context("Failed to run fvm install")?;

    pb.finish_and_clear();

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // "already installed" is not an error
        if !stderr.to_lowercase().contains("already installed") {
            anyhow::bail!("fvm install {} failed: {}", version, stderr);
        }
    }
    Ok(())
}

/// Pin a Flutter version in the project via `fvm use`.
pub fn fvm_use(version: &str, project_dir: &str) -> Result<()> {
    let output = Command::new("fvm")
        .current_dir(project_dir)
        .arg("use")
        .arg(version)
        .output()
        .context("Failed to run fvm use")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("fvm use {} failed: {}", version, stderr);
    }
    println!(
        "  {} Flutter {} pinned via FVM.",
        style("✓").green(),
        version
    );
    Ok(())
}

fn make_spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb
}

pub fn create_project(name: &str, runner: &FlutterRunner) -> Result<()> {
    let pb = make_spinner();
    pb.set_message(format!("Running flutter create {}...", name));
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let status = match runner {
        FlutterRunner::Fvm => Command::new("fvm")
            .arg("flutter")
            .arg("create")
            .arg(name)
            .arg("--empty")
            .output()
            .context("Failed to execute fvm flutter create")?,
        _ => Command::new("flutter")
            .arg("create")
            .arg(name)
            .arg("--empty")
            .output()
            .context("Failed to execute flutter create. Is Flutter in your PATH?")?,
    };

    pb.finish_and_clear();

    if !status.status.success() {
        let stderr = String::from_utf8_lossy(&status.stderr);
        anyhow::bail!("flutter create failed: {}", stderr);
    }

    Ok(())
}

pub fn pub_get(project_dir: &str, runner: &FlutterRunner) -> Result<()> {
    let pb = make_spinner();
    pb.set_message("Running flutter pub get...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let status = match runner {
        FlutterRunner::Fvm => Command::new("fvm")
            .current_dir(project_dir)
            .arg("flutter")
            .arg("pub")
            .arg("get")
            .output()
            .context("Failed to execute fvm flutter pub get")?,
        _ => Command::new("flutter")
            .current_dir(project_dir)
            .arg("pub")
            .arg("get")
            .output()
            .context("Failed to execute flutter pub get")?,
    };

    pb.finish_and_clear();

    if !status.status.success() {
        let stderr = String::from_utf8_lossy(&status.stderr);
        anyhow::bail!("flutter pub get failed: {}", stderr);
    }

    Ok(())
}
