use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn generate(
    project_dir: &str,
    structure: &std::collections::HashMap<String, Vec<String>>,
) -> Result<usize> {
    let base_path = Path::new(project_dir);
    let mut dirs_created = 0;

    for (dir, _) in structure {
        let full_path = base_path.join(dir);
        if !full_path.exists() {
            fs::create_dir_all(&full_path)?;
            dirs_created += 1;
        }
    }

    Ok(dirs_created)
}
