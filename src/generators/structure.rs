use anyhow::{bail, Result};
use std::fs;
use std::path::{Component, Path};

/// Returns true if the path contains no `..` or absolute components.
fn is_safe_relative_path(path: &str) -> bool {
    Path::new(path).components().all(|c| {
        matches!(c, Component::Normal(_) | Component::CurDir)
    })
}

pub fn generate(
    project_dir: &str,
    structure: &std::collections::HashMap<String, Vec<String>>,
) -> Result<usize> {
    let base_path = Path::new(project_dir);
    let mut dirs_created = 0;

    for (dir, _) in structure {
        if !is_safe_relative_path(dir) {
            bail!("Chemin de dossier invalide dans le template : '{}'", dir);
        }
        let full_path = base_path.join(dir);
        if !full_path.exists() {
            fs::create_dir_all(&full_path)?;
            dirs_created += 1;
        }
    }

    Ok(dirs_created)
}
