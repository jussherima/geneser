use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;

pub fn create_project(name: &str) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")?,
    );
    pb.set_message(format!("Running flutter create {}...", name));
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let status = Command::new("flutter")
        .arg("create")
        .arg(name)
        .arg("--empty")
        .output()
        .context("Failed to execute flutter create. Is Flutter in your PATH?")?;

    pb.finish_and_clear();

    if !status.status.success() {
        let stderr = String::from_utf8_lossy(&status.stderr);
        anyhow::bail!("flutter create failed: {}", stderr);
    }

    Ok(())
}

pub fn pub_get(project_dir: &str) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")?,
    );
    pb.set_message("Running flutter pub get...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let status = Command::new("flutter")
        .current_dir(project_dir)
        .arg("pub")
        .arg("get")
        .output()
        .context("Failed to execute flutter pub get")?;

    pb.finish_and_clear();

    if !status.status.success() {
        let stderr = String::from_utf8_lossy(&status.stderr);
        anyhow::bail!("flutter pub get failed: {}", stderr);
    }

    Ok(())
}
