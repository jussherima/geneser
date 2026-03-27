use crate::models::template_definition::TemplateDefinition;
use crate::registry::templates_dir;
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Component, PathBuf};
use std::time::Duration;

/// A locally installed community template.
#[derive(Debug, Clone)]
pub struct LocalTemplate {
    pub name: String,
    pub path: PathBuf,
    pub definition: TemplateDefinition,
}

/// Scan ~/.config/geneser/templates/ and load all valid template.json files.
pub fn load_local_templates() -> Vec<LocalTemplate> {
    let dir = templates_dir();
    let mut templates = Vec::new();

    let entries = match fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => return templates,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let json_path = path.join("template.json");
        if let Ok(data) = fs::read_to_string(&json_path) {
            if let Ok(def) = serde_json::from_str::<TemplateDefinition>(&data) {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                templates.push(LocalTemplate {
                    name,
                    path,
                    definition: def,
                });
            }
        }
    }

    templates
}

fn validate_github_component(s: &str, label: &str) -> Result<()> {
    if s.is_empty() || s.len() > 100 {
        bail!("{} invalide (longueur 1-100 caractères attendue)", label);
    }
    if !s.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.')) {
        bail!("{} contient des caractères non autorisés : '{}'", label, s);
    }
    Ok(())
}

/// Install a template from a GitHub tar.gz archive.
/// `github_ref` is like "user/repo".
pub fn install_from_github(github_ref: &str) -> Result<LocalTemplate> {
    let parts: Vec<&str> = github_ref.splitn(2, '/').collect();
    if parts.len() != 2 {
        bail!("Format invalide. Attendu: user/repo");
    }
    let (user, repo) = (parts[0], parts[1]);
    validate_github_component(user, "Utilisateur GitHub")?;
    validate_github_component(repo, "Nom du dépôt GitHub")?;

    let url = format!(
        "https://github.com/{}/{}/archive/refs/heads/main.tar.gz",
        user, repo
    );

    println!("  Telechargement de {} ...", url);
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .context("Impossible de créer le client HTTP")?;

    let response = client
        .get(&url)
        .send()
        .with_context(|| format!("Impossible de telecharger {}", url))?;

    if !response.status().is_success() {
        bail!("Depot introuvable ou inaccessible : {} (HTTP {})", url, response.status());
    }

    let bytes = response.bytes()?;

    let dest_dir = templates_dir().join(repo);
    fs::create_dir_all(&dest_dir)?;

    // Extract tar.gz
    let gz = flate2::read::GzDecoder::new(std::io::Cursor::new(bytes));
    let mut archive = tar::Archive::new(gz);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?.to_path_buf();

        // Strip the first component (e.g., "repo-main/")
        let stripped: PathBuf = entry_path.components().skip(1).collect();
        if stripped.as_os_str().is_empty() {
            continue;
        }

        // Reject paths that escape the destination directory
        let has_traversal = stripped.components().any(|c| {
            matches!(c, Component::ParentDir | Component::RootDir | Component::Prefix(_))
        });
        if has_traversal {
            bail!("Le template contient un chemin interdit : '{}'", stripped.display());
        }

        let output_path = dest_dir.join(&stripped);
        if entry.header().entry_type().is_dir() {
            fs::create_dir_all(&output_path)?;
        } else {
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            entry.unpack(&output_path)?;
        }
    }

    // Validate template.json
    let json_path = dest_dir.join("template.json");
    let data = fs::read_to_string(&json_path)
        .with_context(|| "template.json introuvable dans le depot")?;
    let def: TemplateDefinition = serde_json::from_str(&data)
        .with_context(|| "template.json invalide — verifiez la syntaxe JSON")?;

    // La documentation est obligatoire pour les templates communautaires
    match &def.docs_path {
        None => {
            fs::remove_dir_all(&dest_dir)?;
            anyhow::bail!(
                "Ce template ne definit pas `docs_path` dans template.json.\n\
                 Ajoutez le chemin vers votre fichier de documentation :\n\
                 \x20 \"docs_path\": \"docs/ARCHITECTURE.md\"\n\
                 Les templates communautaires doivent documenter leur architecture."
            );
        }
        Some(path) => {
            let doc_file = dest_dir.join(path);
            if !doc_file.exists() {
                fs::remove_dir_all(&dest_dir)?;
                anyhow::bail!(
                    "Le fichier de documentation '{}' est introuvable dans le repo.\n\
                     Verifiez que `docs_path` pointe vers un fichier existant.",
                    path
                );
            }
            let content = fs::read_to_string(&doc_file)?;
            if content.trim().is_empty() {
                fs::remove_dir_all(&dest_dir)?;
                anyhow::bail!(
                    "Le fichier de documentation '{}' est vide.\n\
                     Documentez votre architecture avant de publier ce template.",
                    path
                );
            }
        }
    }

    println!("  Template '{}' installe dans {:?}", repo, dest_dir);
    Ok(LocalTemplate {
        name: repo.to_string(),
        path: dest_dir,
        definition: def,
    })
}

/// Remove a locally installed template by name.
pub fn remove_template(name: &str) -> Result<()> {
    let path = templates_dir().join(name);
    if !path.exists() {
        anyhow::bail!("Template '{}' introuvable", name);
    }
    fs::remove_dir_all(&path)?;
    println!("  Template '{}' supprime.", name);
    Ok(())
}

/// Update all locally installed templates.
pub fn update_all_templates() -> Result<()> {
    let dir = templates_dir();
    let entries = match fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => {
            println!("  Aucun template installe.");
            return Ok(());
        }
    };

    let repos: Vec<String> = entries
        .flatten()
        .filter(|e| e.path().is_dir())
        .filter_map(|e| {
            // Check if it has a template.json with a repo field
            let json = e.path().join("template.json");
            let data = fs::read_to_string(json).ok()?;
            let def: serde_json::Value = serde_json::from_str(&data).ok()?;
            def.get("repo")?.as_str().map(|s| s.to_string())
        })
        .collect();

    if repos.is_empty() {
        println!("  Aucun template a mettre a jour.");
        return Ok(());
    }

    for repo in repos {
        println!("  Mise a jour de {} ...", repo);
        // Re-install overwrites
        let github_ref = repo.trim_start_matches("github:");
        if let Err(e) = install_from_github(github_ref) {
            eprintln!("  Erreur lors de la mise a jour de {}: {}", repo, e);
        }
    }

    Ok(())
}
