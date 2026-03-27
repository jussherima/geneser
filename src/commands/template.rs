use crate::commands::create::OFFICIAL_TEMPLATES;
use crate::registry::loader;
use anyhow::{bail, Result};
use console::style;
use std::fs;

pub fn run(action: TemplateAction) -> Result<()> {
    match action {
        TemplateAction::Add { source } => add(&source),
        TemplateAction::List => list(),
        TemplateAction::Remove { name } => remove(&name),
        TemplateAction::Update => update(),
        TemplateAction::Docs { name } => docs(&name),
    }
}

pub enum TemplateAction {
    Add { source: String },
    List,
    Remove { name: String },
    Update,
    Docs { name: String },
}

fn add(source: &str) -> Result<()> {
    let github_ref = source.strip_prefix("github:").unwrap_or(source);
    let lt = loader::install_from_github(github_ref)?;

    println!("  {} Template '{}' installe.", style("✓").green(), lt.definition.name);

    // Rappeler que la doc est disponible
    if lt.definition.docs_path.is_some() {
        println!();
        println!(
            "  {} Lisez la documentation de l'architecture :",
            style("i").cyan()
        );
        println!(
            "    {}",
            style(format!("geneser template docs {}", lt.name)).cyan()
        );
    }

    Ok(())
}

fn list() -> Result<()> {
    println!("\n  {}", style("Templates disponibles").bold().cyan());

    println!("\n  {}", style("Officiels (embarques) :").dim());
    for (label, _, _, _) in OFFICIAL_TEMPLATES {
        println!(
            "    • {}  {}",
            style(label).green(),
            style("[doc disponible]").dim()
        );
    }
    println!("    • {}", style("Custom").green());

    let locals = loader::load_local_templates();
    if locals.is_empty() {
        println!("\n  {} Aucun template communautaire installe.", style("i").dim());
        println!(
            "  Installez-en un avec : {}",
            style("geneser template add github:user/repo").cyan()
        );
    } else {
        println!("\n  {}", style("Communautaires (locaux) :").dim());
        for t in &locals {
            let has_docs = if t.definition.docs_path.is_some() {
                style(" [doc]").cyan().to_string()
            } else {
                String::new()
            };
            println!(
                "    • {}{}  —  {}",
                style(&t.definition.name).green(),
                has_docs,
                t.definition.description
            );
        }
        println!();
        println!("  {} Pour lire la doc d'un template :", style("i").dim());
        println!("    {}", style("geneser template docs <nom>").cyan());
    }

    Ok(())
}

fn remove(name: &str) -> Result<()> {
    loader::remove_template(name)?;
    println!("  {} Template supprime.", style("✓").green());
    Ok(())
}

fn update() -> Result<()> {
    loader::update_all_templates()?;
    println!("  {} Templates mis a jour.", style("✓").green());
    Ok(())
}

fn docs(name: &str) -> Result<()> {
    // Chercher d'abord dans les templates officiels
    let official = OFFICIAL_TEMPLATES.iter().find(|(label, _, _, _)| {
        label.eq_ignore_ascii_case(name)
    });

    if let Some((label, _, _, doc_content)) = official {
        println!(
            "\n  {} {}",
            style("●").cyan().bold(),
            style(label).bold().cyan()
        );
        println!("{}", style("─".repeat(60)).dim());
        println!("{}", doc_content);
        println!("{}", style("─".repeat(60)).dim());
        return Ok(());
    }

    // Sinon chercher dans les templates communautaires
    let locals = loader::load_local_templates();
    let lt = locals
        .iter()
        .find(|t| {
            t.name.eq_ignore_ascii_case(name)
                || t.definition.name.eq_ignore_ascii_case(name)
        })
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Template '{}' introuvable. Listez les templates avec : geneser template list",
                name
            )
        })?;

    let docs_path = lt.definition.docs_path.as_deref().ok_or_else(|| {
        anyhow::anyhow!(
            "Le template '{}' ne definit pas de documentation (docs_path absent).",
            lt.definition.name
        )
    })?;

    let doc_file = lt.path.join(docs_path);
    if !doc_file.exists() {
        bail!(
            "Fichier de documentation introuvable : {:?}\n\
             Le template est peut-etre corrompu. Reinstallez-le avec :\n\
             geneser template add github:...",
            doc_file
        );
    }

    let content = fs::read_to_string(&doc_file)?;

    println!(
        "\n  {} {}",
        style("●").cyan().bold(),
        style(&lt.definition.name).bold().cyan()
    );
    println!("  {}\n", style(&lt.definition.description).dim());
    println!("{}", style("─".repeat(60)).dim());
    println!("{}", content);
    println!("{}", style("─".repeat(60)).dim());

    Ok(())
}
