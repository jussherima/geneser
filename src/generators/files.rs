use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Embedded CWA templates
const CWA_MAIN_DART: &str = include_str!("../embedded/cwa/main.dart.tmpl");
const CWA_FEATURE_SCREEN: &str = include_str!("../embedded/cwa/features/screen.dart.tmpl");
const CWA_FEATURE_SERVICE: &str = include_str!("../embedded/cwa/features/service.dart.tmpl");
const CWA_FEATURE_MODEL: &str = include_str!("../embedded/cwa/features/model.dart.tmpl");

// Boilerplate templates (starter code)
const BP_MAIN_BLOC: &str = include_str!("../embedded/boilerplate/main_bloc.dart.tmpl");
const BP_MAIN_GETX: &str = include_str!("../embedded/boilerplate/main_getx.dart.tmpl");
const BP_MAIN_PROVIDER: &str = include_str!("../embedded/boilerplate/main_provider.dart.tmpl");
const BP_SCREEN_BLOC: &str = include_str!("../embedded/boilerplate/screen_bloc.dart.tmpl");
const BP_SCREEN_GETX: &str = include_str!("../embedded/boilerplate/screen_getx.dart.tmpl");
const BP_SCREEN_PROVIDER: &str = include_str!("../embedded/boilerplate/screen_provider.dart.tmpl");
const BP_REPOSITORY: &str = include_str!("../embedded/boilerplate/repository.dart.tmpl");
const BP_GETX_CONTROLLER: &str = include_str!("../embedded/boilerplate/getx_controller.dart.tmpl");
const BP_CUBIT: &str = include_str!("../embedded/boilerplate/cubit.dart.tmpl");

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

/// Extracts (feature_name, template_key) from a file name.
/// E.g. "profile_screen.dart" -> ("profile", "feature_screen.dart")
///      "profile_repository.dart" -> ("profile", "feature_repository.dart")
///      "profile.dart" (in domain dir) -> ("profile", "feature_model.dart")
fn extract_feature_template(file: &str) -> Option<(String, String)> {
    let suffixes = [
        "_screen.dart",
        "_controller.dart",
        "_repository.dart",
        "_service.dart",
        "_model.dart",
    ];
    for suffix in &suffixes {
        if file.ends_with(suffix) {
            let feature = file.replace(suffix, "");
            let template_key = format!("feature{}", suffix);
            return Some((feature, template_key));
        }
    }
    None
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
    state_management: &str,
    starter_code: bool,
) -> Result<usize> {
    let base_path = Path::new(project_dir);
    let mut files_created = 0;

    for (dir, files) in structure {
        let dir_path = base_path.join(dir);

        for file in files {
            let file_path = dir_path.join(file);
            let content = generate_file_content(
                file,
                project_name,
                state_management,
                starter_code,
            );
            fs::write(&file_path, content)?;
            files_created += 1;
        }
    }

    Ok(files_created)
}

fn generate_file_content(
    file: &str,
    project_name: &str,
    state_management: &str,
    starter_code: bool,
) -> String {
    // Helper: apply feature variable substitutions
    let apply_feature_vars = |tmpl: &str, feature: &str| -> String {
        tmpl.replace("{{feature_name_pascal}}", &to_pascal_case(feature))
            .replace("{{feature_name_snake}}", feature)
            .replace("{{feature_name_camel}}", &to_camel_case(feature))
    };

    if file == "main.dart" {
        let tmpl = if starter_code {
            match state_management {
                "bloc" => BP_MAIN_BLOC,
                "getx" => BP_MAIN_GETX,
                "provider" => BP_MAIN_PROVIDER,
                _ => CWA_MAIN_DART, // riverpod + fallback
            }
        } else {
            CWA_MAIN_DART
        };
        return tmpl.replace("{{project_name}}", project_name);
    }

    if file.ends_with("_screen.dart") {
        let feature = file.replace("_screen.dart", "");
        let tmpl = if starter_code {
            match state_management {
                "bloc" => BP_SCREEN_BLOC,
                "getx" => BP_SCREEN_GETX,
                "provider" => BP_SCREEN_PROVIDER,
                _ => CWA_FEATURE_SCREEN,
            }
        } else {
            CWA_FEATURE_SCREEN
        };
        return apply_feature_vars(tmpl, &feature);
    }

    if file.ends_with("_service.dart") {
        let feature = file.replace("_service.dart", "");
        return apply_feature_vars(CWA_FEATURE_SERVICE, &feature);
    }

    if file.ends_with("_model.dart") {
        let feature = file.replace("_model.dart", "");
        return apply_feature_vars(CWA_FEATURE_MODEL, &feature);
    }

    if starter_code {
        if file.ends_with("_repository.dart") {
            let feature = file.replace("_repository.dart", "");
            return apply_feature_vars(BP_REPOSITORY, &feature);
        }
        if file.ends_with("_controller.dart") {
            let feature = file.replace("_controller.dart", "");
            return apply_feature_vars(BP_GETX_CONTROLLER, &feature);
        }
        if file.ends_with("_cubit.dart") {
            let feature = file.replace("_cubit.dart", "");
            return apply_feature_vars(BP_CUBIT, &feature);
        }
    }

    format!("// TODO: Implement {}\n", file)
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
            } else if let Some((feature, template_key)) = extract_feature_template(file) {
                let feature_pascal = to_pascal_case(&feature);
                let feature_camel = to_camel_case(&feature);
                let mut fvars = vars.clone();
                fvars.insert("feature_name_pascal".to_string(), feature_pascal);
                fvars.insert("feature_name_snake".to_string(), feature);
                fvars.insert("feature_name_camel".to_string(), feature_camel);
                if let Some(tmpl) = templates.get(&template_key) {
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
