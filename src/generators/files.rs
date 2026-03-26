use anyhow::Result;
use std::fs;
use std::path::Path;

// Embedded templates
const MAIN_DART: &str = include_str!("../embedded/cwa/main.dart.tmpl");
const FEATURE_SCREEN: &str = include_str!("../embedded/cwa/features/screen.dart.tmpl");
const FEATURE_SERVICE: &str = include_str!("../embedded/cwa/features/service.dart.tmpl");
const FEATURE_MODEL: &str = include_str!("../embedded/cwa/features/model.dart.tmpl");

fn to_pascal_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn to_camel_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}

pub fn generate(
    project_dir: &str,
    project_name: &str,
    structure: &std::collections::HashMap<String, Vec<String>>,
) -> Result<usize> {
    let base_path = Path::new(project_dir);
    let mut files_created = 0;

    for (dir, files) in structure {
        let dir_path = base_path.join(dir);

        for file in files {
            let file_path = dir_path.join(file);

            let content = if file == "main.dart" {
                MAIN_DART.replace("{{project_name}}", project_name)
            } else if file.ends_with("_screen.dart") {
                let feature = file.replace("_screen.dart", "");
                FEATURE_SCREEN
                    .replace("{{feature_name_pascal}}", &to_pascal_case(&feature))
                    .replace("{{feature_name_snake}}", &feature)
                    .replace("{{feature_name_camel}}", &to_camel_case(&feature))
            } else if file.ends_with("_service.dart") {
                let feature = file.replace("_service.dart", "");
                FEATURE_SERVICE
                    .replace("{{feature_name_pascal}}", &to_pascal_case(&feature))
                    .replace("{{feature_name_snake}}", &feature)
                    .replace("{{feature_name_camel}}", &to_camel_case(&feature))
            } else if file.ends_with("_model.dart") {
                let feature = file.replace("_model.dart", "");
                FEATURE_MODEL
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
