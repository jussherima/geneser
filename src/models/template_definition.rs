use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateDefinition {
    pub name: String,
    pub description: String,
    pub state_management: String,
    pub routing: String,
    pub prompts: Vec<PromptDef>,
    pub packages: PackagesDef,
    pub dev_packages: PackagesDef,
    pub structure: StructureDef,
    pub root_files: Vec<String>,
    /// flag_name -> expression like "firebase != none"
    pub flags: HashMap<String, String>,
    /// Base path for features, e.g. "lib/src/features" or "lib/features"
    #[serde(default = "default_features_path")]
    pub features_path: String,
    /// Pattern of dirs/files to create for each selected feature.
    /// Use `{feature}` as placeholder (replaced by the actual feature name).
    #[serde(default)]
    pub feature_template: HashMap<String, Vec<String>>,
    /// Chemin vers le fichier de documentation de l'architecture dans le repo.
    /// Obligatoire pour les templates communautaires. Ex: "docs/ARCHITECTURE.md"
    pub docs_path: Option<String>,
}

fn default_features_path() -> String {
    "lib/src/features".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct PromptDef {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub prompt_type: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PackagesDef {
    pub base: Vec<String>,
    #[serde(default)]
    pub conditional: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StructureDef {
    /// dir -> files
    pub base: HashMap<String, Vec<String>>,
    /// flag_name -> (dir -> extra files)
    #[serde(default)]
    pub conditional: HashMap<String, HashMap<String, Vec<String>>>,
}

impl TemplateDefinition {
    /// Evaluate flag expressions against a variable map.
    pub fn evaluate_flags(&self, vars: &HashMap<String, String>) -> HashMap<String, bool> {
        self.flags
            .iter()
            .map(|(flag, expr)| (flag.clone(), evaluate_expr(expr, vars)))
            .collect()
    }

    /// Select packages (base + conditional based on flags).
    pub fn resolve_packages(&self, flags: &HashMap<String, bool>) -> Vec<String> {
        let mut pkgs = self.packages.base.clone();
        for (flag, packages) in &self.packages.conditional {
            if *flags.get(flag).unwrap_or(&false) {
                pkgs.extend(packages.clone());
            }
        }
        pkgs
    }

    /// Select dev packages (base + conditional based on flags).
    pub fn resolve_dev_packages(&self, flags: &HashMap<String, bool>) -> Vec<String> {
        let mut pkgs = self.dev_packages.base.clone();
        for (flag, packages) in &self.dev_packages.conditional {
            if *flags.get(flag).unwrap_or(&false) {
                pkgs.extend(packages.clone());
            }
        }
        pkgs
    }

    /// Build structure map from base + conditional entries.
    /// Replaces `{project_name}` placeholder in file names.
    pub fn resolve_structure(
        &self,
        flags: &HashMap<String, bool>,
        project_name: &str,
    ) -> HashMap<String, Vec<String>> {
        let mut structure: HashMap<String, Vec<String>> = self
            .structure
            .base
            .iter()
            .map(|(dir, files)| {
                (
                    dir.clone(),
                    files
                        .iter()
                        .map(|f| f.replace("{project_name}", project_name))
                        .collect(),
                )
            })
            .collect();

        for (flag, extra_dirs) in &self.structure.conditional {
            if *flags.get(flag).unwrap_or(&false) {
                for (dir, extra_files) in extra_dirs {
                    let entry = structure.entry(dir.clone()).or_default();
                    for f in extra_files {
                        let f = f.replace("{project_name}", project_name);
                        if !entry.contains(&f) {
                            entry.push(f);
                        }
                    }
                }
            }
        }

        structure
    }

    /// Expand `feature_template` for each selected feature and merge into `structure`.
    ///
    /// In `feature_template`, keys and values can contain `{feature}` (replaced by
    /// the feature name). Each key is prefixed with `{features_path}/` unless it
    /// already starts with `lib/`.
    pub fn resolve_features(
        &self,
        features: &[String],
        structure: &mut HashMap<String, Vec<String>>,
    ) {
        if self.feature_template.is_empty() {
            return;
        }
        for feature in features {
            for (template_key, template_files) in &self.feature_template {
                let key_expanded = template_key.replace("{feature}", feature);
                // Prefix with features_path if the key doesn't start with {feature}
                // (meaning it's already an absolute lib/ path)
                let dir = if key_expanded.starts_with("lib/") {
                    key_expanded
                } else {
                    format!("{}/{}", self.features_path, key_expanded)
                };
                let files: Vec<String> = template_files
                    .iter()
                    .map(|f| f.replace("{feature}", feature))
                    .collect();
                structure.entry(dir).or_insert(files);
            }
        }
    }
}

/// Evaluates "var == value" or "var != value".
fn evaluate_expr(expr: &str, vars: &HashMap<String, String>) -> bool {
    if let Some((var, val)) = expr.split_once(" == ") {
        vars.get(var.trim()).map(|v| v.as_str()) == Some(val.trim())
    } else if let Some((var, val)) = expr.split_once(" != ") {
        vars.get(var.trim()).map(|v| v.as_str()) != Some(val.trim())
    } else {
        false
    }
}
