use crate::generators::{files, flutter, pubspec, root_files, structure};
use crate::models::options::{ExtraPackage, RoutingSolution, StateManagement, VersionStrategy};
use crate::models::template_config::TemplateConfig;
use crate::models::template_definition::TemplateDefinition;
use crate::registry::loader::load_local_templates;
use crate::templates::code_with_andrea::CodeWithAndreaTemplate;
use crate::templates::custom::CustomTemplate;
use crate::ui::prompts;
use anyhow::{Context, Result};
use console::style;
use std::collections::HashMap;

/// Embedded official templates: (menu label, JSON source, uses_embedded_dart_files, docs)
/// `uses_embedded_dart_files` = true uniquement pour CWA Medium dont les .dart sont embarqués.
pub const OFFICIAL_TEMPLATES: &[(&str, &str, bool, &str)] = &[
    (
        "CodeWithAndrea (Feature-first)",
        include_str!("../config/templates/cwa.json"),
        false,
        include_str!("../config/templates/docs/cwa_feature_first.md"),
    ),
    (
        "CodeWithAndrea (Medium)",
        include_str!("../config/templates/code_with_andrea.json"),
        true,
        include_str!("../config/templates/docs/cwa_medium.md"),
    ),
    (
        "Feature-First MVVM + GetX",
        include_str!("../config/templates/getx_mvvm.json"),
        false,
        include_str!("../config/templates/docs/getx_mvvm.md"),
    ),
    (
        "Clean Architecture + BLoC",
        include_str!("../config/templates/clean_bloc.json"),
        false,
        include_str!("../config/templates/docs/clean_bloc.md"),
    ),
    (
        "Riverpod + Freezed (Minimal)",
        include_str!("../config/templates/riverpod_minimal.json"),
        false,
        include_str!("../config/templates/docs/riverpod_minimal.md"),
    ),
];

const IDX_CUSTOM: usize = OFFICIAL_TEMPLATES.len();

fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() || name.len() > 128 {
        anyhow::bail!("Le nom du projet doit contenir entre 1 et 128 caractères.");
    }
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        anyhow::bail!(
            "Le nom du projet ne peut contenir que des lettres, chiffres et underscores (a-z, 0-9, _)."
        );
    }
    if name.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        anyhow::bail!("Le nom du projet ne peut pas commencer par un chiffre.");
    }
    Ok(())
}

pub fn run(name: Option<String>) -> Result<()> {
    println!("\n  {}", style("Geneser CLI").bold().cyan());
    println!("  {}\n", style("Flutter Architecture Generator").dim());

    let project_name = match name {
        Some(n) => n,
        None => prompts::ask_input("Nom du projet", Some("my_app"))?,
    };
    validate_project_name(&project_name)?;

    // Build template menu
    let local_templates = load_local_templates();
    let mut menu: Vec<String> = OFFICIAL_TEMPLATES
        .iter()
        .map(|(label, _, _, _)| label.to_string())
        .collect();
    menu.push("Custom (Choisir packages)".to_string());
    for lt in &local_templates {
        menu.push(format!("{} [communautaire]", lt.definition.name));
    }

    let menu_refs: Vec<&str> = menu.iter().map(|s| s.as_str()).collect();
    let template_idx = prompts::ask_select("Choisissez un template:", &menu_refs, 0)?;

    let config = if template_idx < OFFICIAL_TEMPLATES.len() {
        let (_, json_src, _, _) = OFFICIAL_TEMPLATES[template_idx];
        let def: TemplateDefinition = serde_json::from_str(json_src)
            .with_context(|| "JSON template invalide — ceci est un bug dans geneser")?;
        run_json_template(&project_name, &def)?
    } else if template_idx == IDX_CUSTOM {
        run_custom_template(&project_name)?
    } else {
        let lt = &local_templates[template_idx - IDX_CUSTOM - 1];
        run_json_template(&project_name, &lt.definition)?
    };

    let uses_dart_templates = template_idx < OFFICIAL_TEMPLATES.len()
        && OFFICIAL_TEMPLATES[template_idx].2;  // index 2 = uses_embedded_dart_files

    let version_options = [VersionStrategy::Stable, VersionStrategy::Latest];
    let version_idx = prompts::ask_select("Versions des packages:", &version_options, 0)?;
    let version_strategy = version_options[version_idx];

    let starter_code =
        prompts::ask_confirm("Generer le boilerplate de démarrage (main.dart, screens, repos) ?", true)?;

    println!("\n{}", style("Recapitulatif :").bold());
    println!(" - Projet   : {}", style(&project_name).green());
    println!(" - Template : {}", style(&config.name).green());
    println!(
        " - Features : {}",
        style(config.features.join(", ")).green()
    );
    println!(" - Versions : {}", style(&version_strategy).green());
    println!(
        " - Starter  : {}",
        style(if starter_code { "Boilerplate complet" } else { "Stubs vides" }).green()
    );

    let confirm = prompts::ask_confirm("Generer ce projet maintenant ?", true)?;
    if !confirm {
        println!("{}", style("Operation annulee.").red());
        return Ok(());
    }
    println!();

    // 1. flutter create
    flutter::create_project(&project_name)?;
    println!("  {} Projet de base cree.", style("✓").green());

    // 2. Directories
    let dirs_created = structure::generate(&project_name, &config.structure)?;
    println!("  {} {} dossiers crees.", style("✓").green(), dirs_created);

    // 3. Files
    let files_created = if uses_dart_templates {
        let tmpl_map = CodeWithAndreaTemplate::templates(&project_name);
        files::generate_code_with_andrea(
            &project_name,
            &project_name,
            &config.structure,
            &config.flags,
            &tmpl_map,
        )?
    } else {
        files::generate(
            &project_name,
            &project_name,
            &config.structure,
            &config.state_management,
            starter_code,
        )?
    };
    println!("  {} {} fichiers generes.", style("✓").green(), files_created);

    // 4. Root config files (CWA Medium uniquement)
    if uses_dart_templates && !config.root_files.is_empty() {
        let mut vars = HashMap::new();
        vars.insert("project_name".to_string(), project_name.clone());
        let root_count =
            root_files::generate(&project_name, &config.root_files, &vars, &config.flags)?;
        println!(
            "  {} {} fichiers racine generes.",
            style("✓").green(),
            root_count
        );
    }

    // 5. Pubspec
    pubspec::add_dependencies(&project_name, &config.packages, &version_strategy)?;
    if !config.dev_packages.is_empty() {
        pubspec::add_dev_dependencies(&project_name, &config.dev_packages, &version_strategy)?;
    }
    flutter::pub_get(&project_name)?;
    println!(
        "  {} {} packages ajoutes.",
        style("✓").green(),
        config.packages.len() + config.dev_packages.len()
    );

    println!(
        "\n  {} {}",
        style("Succes !").green().bold(),
        style(format!("Projet {} pret.", project_name)).bold()
    );
    println!("  cd {}", project_name);
    Ok(())
}

/// Flux interactif entièrement piloté par un `TemplateDefinition` JSON.
/// Utilisé pour tous les templates officiels et communautaires.
fn run_json_template(project_name: &str, def: &TemplateDefinition) -> Result<TemplateConfig> {
    let mut vars: HashMap<String, String> = HashMap::new();
    let mut feature_list: Vec<String> = Vec::new();

    for prompt in &def.prompts {
        match prompt.prompt_type.as_str() {
            "select" => {
                let opts: Vec<&str> = prompt.options.iter().map(|s| s.as_str()).collect();
                let idx = prompts::ask_select(&prompt.label, &opts, 0)?;
                vars.insert(prompt.id.clone(), prompt.options[idx].clone());
            }
            "multiselect" => {
                let opts: Vec<&str> = prompt.options.iter().map(|s| s.as_str()).collect();
                let defaults = vec![false; prompt.options.len()];
                let selected = prompts::ask_multiselect(&prompt.label, &opts, &defaults)?;
                let values: Vec<String> =
                    selected.iter().map(|&i| prompt.options[i].clone()).collect();
                if prompt.id == "features" {
                    feature_list = values.clone();
                }
                vars.insert(prompt.id.clone(), values.join(","));
            }
            _ => {}
        }
    }

    // CWA Medium : "home" toujours présent
    if def.name.contains("Medium") && !feature_list.contains(&"home".to_string()) {
        feature_list.insert(0, "home".to_string());
    }

    let flags = def.evaluate_flags(&vars);
    let packages = def.resolve_packages(&flags);
    let dev_packages = def.resolve_dev_packages(&flags);
    let mut structure = def.resolve_structure(&flags, project_name);

    // Expansion du feature_template JSON pour chaque feature sélectionnée
    def.resolve_features(&feature_list, &mut structure);

    Ok(TemplateConfig::new(
        &def.name,
        &def.description,
        feature_list,
        &def.state_management,
        &def.routing,
        packages,
        structure,
    )
    .with_flags(flags)
    .with_dev_packages(dev_packages))
}

/// Template custom : choix libre du state management, routing et packages.
fn run_custom_template(project_name: &str) -> Result<TemplateConfig> {
    let state_options = [
        StateManagement::Riverpod,
        StateManagement::Bloc,
        StateManagement::GetX,
        StateManagement::Provider,
    ];
    let state_idx = prompts::ask_select("State Management:", &state_options, 0)?;

    let routing_options = [
        RoutingSolution::GoRouter,
        RoutingSolution::AutoRoute,
        RoutingSolution::GetXRouting,
        RoutingSolution::Navigator2,
    ];
    let routing_idx = prompts::ask_select("Routing:", &routing_options, 0)?;

    let extra_options = [
        ExtraPackage::Drift,
        ExtraPackage::Dio,
        ExtraPackage::Freezed,
        ExtraPackage::FlutterGen,
        ExtraPackage::Hive,
        ExtraPackage::SharedPreferences,
        ExtraPackage::Firebase,
        ExtraPackage::Equatable,
        ExtraPackage::Dartz,
        ExtraPackage::Intl,
    ];
    let defaults = vec![false; extra_options.len()];
    let selected_extra =
        prompts::ask_multiselect("Packages additionnels:", &extra_options, &defaults)?;
    let extras: Vec<ExtraPackage> = selected_extra.iter().map(|&i| extra_options[i]).collect();

    let basic_features = vec!["authentication", "home"];
    let feature_defaults = vec![true, true];
    let selected_feat =
        prompts::ask_multiselect("Features de base:", &basic_features, &feature_defaults)?;
    let features: Vec<String> = selected_feat
        .iter()
        .map(|&i| basic_features[i].to_string())
        .collect();

    Ok(CustomTemplate::build_config(
        project_name,
        features,
        state_options[state_idx],
        routing_options[routing_idx],
        extras,
    ))
}
