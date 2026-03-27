use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Embedded CWA templates
const CWA_MAIN_DART: &str = include_str!("../embedded/cwa/main.dart.tmpl");
const CWA_FEATURE_SCREEN: &str = include_str!("../embedded/cwa/features/screen.dart.tmpl");
const CWA_FEATURE_SERVICE: &str = include_str!("../embedded/cwa/features/service.dart.tmpl");
const CWA_FEATURE_MODEL: &str = include_str!("../embedded/cwa/features/model.dart.tmpl");

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut c = part.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    let mut c = pascal.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}

/// Render a template string with variable substitution and conditional blocks.
///
/// Supports:
/// - `{{var_name}}` → replaced by vars\[var_name\]
/// - `{{#if flag_name}}...{{/if flag_name}}` → block kept if flags\[flag_name\] == true, removed otherwise
/// - `{{#unless flag_name}}...{{/unless flag_name}}` → inverse of #if
pub fn render_template(
    template: &str,
    vars: &HashMap<String, String>,
    flags: &HashMap<String, bool>,
) -> String {
    let mut result = template.to_string();

    // Process {{#if flag}}...{{/if flag}} blocks
    loop {
        let Some(start) = result.find("{{#if ") else {
            break;
        };
        let Some(tag_end) = result[start..].find("}}") else {
            break;
        };
        let flag_name = result[start + 6..start + tag_end].trim().to_string();
        let end_tag = format!("{{{{/if {}}}}}", flag_name);
        let Some(end_pos) = result.find(&end_tag) else {
            break;
        };

        let block_content = &result[start + tag_end + 2..end_pos];
        let flag_value = flags.get(&flag_name).copied().unwrap_or(false);

        let replacement = if flag_value {
            block_content.to_string()
        } else {
            String::new()
        };

        result = format!(
            "{}{}{}",
            &result[..start],
            replacement,
            &result[end_pos + end_tag.len()..]
        );
    }

    // Process {{#unless flag}}...{{/unless flag}} blocks
    loop {
        let Some(start) = result.find("{{#unless ") else {
            break;
        };
        let Some(tag_end) = result[start..].find("}}") else {
            break;
        };
        let flag_name = result[start + 10..start + tag_end].trim().to_string();
        let end_tag = format!("{{{{/unless {}}}}}", flag_name);
        let Some(end_pos) = result.find(&end_tag) else {
            break;
        };

        let block_content = &result[start + tag_end + 2..end_pos];
        let flag_value = flags.get(&flag_name).copied().unwrap_or(false);

        let replacement = if !flag_value {
            block_content.to_string()
        } else {
            String::new()
        };

        result = format!(
            "{}{}{}",
            &result[..start],
            replacement,
            &result[end_pos + end_tag.len()..]
        );
    }

    // Variable substitution
    for (key, value) in vars {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }

    // Clean up empty lines left by conditionals
    let lines: Vec<&str> = result.lines().collect();
    let mut cleaned = Vec::new();
    let mut prev_empty = false;
    for line in lines {
        let is_empty = line.trim().is_empty();
        if is_empty && prev_empty {
            continue;
        }
        cleaned.push(line);
        prev_empty = is_empty;
    }
    cleaned.join("\n")
}

pub fn generate(
    project_dir: &str,
    project_name: &str,
    structure: &HashMap<String, Vec<String>>,
) -> Result<usize> {
    let base_path = Path::new(project_dir);
    let mut files_created = 0;

    for (dir, files) in structure {
        let dir_path = base_path.join(dir);

        for file in files {
            let file_path = dir_path.join(file);

            let content = if file == "main.dart" {
                CWA_MAIN_DART.replace("{{project_name}}", project_name)
            } else if file.ends_with("_screen.dart") {
                let feature = file.replace("_screen.dart", "");
                CWA_FEATURE_SCREEN
                    .replace("{{feature_name_pascal}}", &to_pascal_case(&feature))
                    .replace("{{feature_name_snake}}", &feature)
                    .replace("{{feature_name_camel}}", &to_camel_case(&feature))
            } else if file.ends_with("_service.dart") {
                let feature = file.replace("_service.dart", "");
                CWA_FEATURE_SERVICE
                    .replace("{{feature_name_pascal}}", &to_pascal_case(&feature))
                    .replace("{{feature_name_snake}}", &feature)
                    .replace("{{feature_name_camel}}", &to_camel_case(&feature))
            } else if file.ends_with("_model.dart") {
                let feature = file.replace("_model.dart", "");
                CWA_FEATURE_MODEL
                    .replace("{{feature_name_pascal}}", &to_pascal_case(&feature))
                    .replace("{{feature_name_snake}}", &feature)
                    .replace("{{feature_name_camel}}", &to_camel_case(&feature))
            } else {
                format!("// TODO: Implement {}\n", file)
            };

            fs::write(&file_path, content)?;
            files_created += 1;
        }
    }

    Ok(files_created)
}

/// Generate files for CodeWithAndrea template using the conditional template engine.
pub fn generate_code_with_andrea(
    project_dir: &str,
    project_name: &str,
    structure: &HashMap<String, Vec<String>>,
    flags: &HashMap<String, bool>,
    templates: &HashMap<String, &str>,
) -> Result<usize> {
    let base_path = Path::new(project_dir);
    let mut files_created = 0;

    let project_name_pascal = to_pascal_case(project_name);

    let mut vars = HashMap::new();
    vars.insert("project_name".to_string(), project_name.to_string());
    vars.insert("project_name_pascal".to_string(), project_name_pascal.clone());

    for (dir, files) in structure {
        let dir_path = base_path.join(dir);

        for file in files {
            let file_path = dir_path.join(file);

            // Look up template by file name
            let content = if let Some(tmpl) = templates.get(file.as_str()) {
                render_template(tmpl, &vars, flags)
            } else if file.ends_with("_screen.dart") {
                let feature = file.replace("_screen.dart", "");
                let feature_pascal = to_pascal_case(&feature);
                let mut fvars = vars.clone();
                fvars.insert("feature_name_pascal".to_string(), feature_pascal);
                fvars.insert("feature_name_snake".to_string(), feature.clone());
                if let Some(tmpl) = templates.get("feature_screen.dart") {
                    render_template(tmpl, &fvars, flags)
                } else {
                    format!("// TODO: Implement {}\n", file)
                }
            } else {
                format!("// TODO: Implement {}\n", file)
            };

            fs::write(&file_path, content)?;
            files_created += 1;
        }
    }

    Ok(files_created)
}
