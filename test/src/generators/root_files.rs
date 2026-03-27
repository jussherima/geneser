use crate::generators::files::render_template;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Generates root-level config files (e.g. .fvmrc, lefthook.yaml, etc.)
pub fn generate(
    project_dir: &str,
    root_files: &[(String, String)],
    vars: &HashMap<String, String>,
    flags: &HashMap<String, bool>,
) -> Result<usize> {
    let base_path = Path::new(project_dir);
    let mut count = 0;

    for (filename, template) in root_files {
        let content = render_template(template, vars, flags);
        let file_path = base_path.join(filename);
        fs::write(&file_path, content)?;
        count += 1;
    }

    Ok(count)
}
