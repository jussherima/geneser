use crate::generators::{files, flutter, pubspec, root_files, structure};
use crate::models::code_with_andrea_options::{FirebaseLevel, CodeWithAndreaOptions, ObservabilityLevel};
use crate::models::options::{ExtraPackage, RoutingSolution, StateManagement};
use crate::templates::custom::CustomTemplate;
use crate::templates::cwa::CwaTemplate;
use crate::templates::code_with_andrea::CodeWithAndreaTemplate;
use crate::ui::prompts;
use anyhow::Result;
use console::style;
use std::collections::HashMap;

pub fn run(name: Option<String>) -> Result<()> {
    println!("\n  {}", style("Geneser CLI").bold().cyan());
    println!("  {}\n", style("Flutter Architecture Generator").dim());

    let project_name = match name {
        Some(n) => n,
        None => prompts::ask_input("Nom du projet", Some("my_app"))?,
    };

    let templates = [
        "CodeWithAndrea (Feature-first)",
        "Custom (Choisir packages)",
        "CodeWithAndrea (Medium)",
    ];
    let template_idx = prompts::ask_select("Choisissez un template:", &templates, 0)?;

    let is_code_with_andrea = template_idx == 2;

    let config = match template_idx {
        0 => {
            // CWA Workflow
            let features = vec![
                "authentication",
                "home",
                "products",
                "cart",
                "orders",
                "reviews",
            ];
            let defaults = vec![true, true, false, false, false, false];
            let selected_indices = prompts::ask_multiselect(
                "Quelles features voulez-vous generer ?",
                &features,
                &defaults,
            )?;
            let selected_features: Vec<String> = selected_indices
                .iter()
                .map(|&i| features[i as usize].to_string())
                .collect();
            CwaTemplate::build_config(&project_name, selected_features)
        }
        1 => {
            // Custom Workflow
            let state_options = [
                StateManagement::Riverpod,
                StateManagement::Bloc,
                StateManagement::GetX,
                StateManagement::Provider,
            ];
            let state_idx = prompts::ask_select("State Management:", &state_options, 0)?;
            let state_management = state_options[state_idx];

            let routing_options = [
                RoutingSolution::GoRouter,
                RoutingSolution::AutoRoute,
                RoutingSolution::GetXRouting,
                RoutingSolution::Navigator2,
            ];
            let routing_idx = prompts::ask_select("Routing:", &routing_options, 0)?;
            let routing = routing_options[routing_idx];

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
            let selected_extra_indices =
                prompts::ask_multiselect("Packages additionnels:", &extra_options, &defaults)?;
            let extras: Vec<ExtraPackage> = selected_extra_indices
                .iter()
                .map(|&i| extra_options[i])
                .collect();

            let basic_features = vec!["authentication", "home"];
            let feature_defaults = vec![true, true];
            let selected_feature_indices =
                prompts::ask_multiselect("Features de base:", &basic_features, &feature_defaults)?;
            let selected_features: Vec<String> = selected_feature_indices
                .iter()
                .map(|&i| basic_features[i as usize].to_string())
                .collect();

            CustomTemplate::build_config(
                &project_name,
                selected_features,
                state_management,
                routing,
                extras,
            )
        }
        2 => {
            // CodeWithAndrea Workflow
            let firebase_options = [
                FirebaseLevel::None,
                FirebaseLevel::AuthFirestore,
                FirebaseLevel::Full,
            ];
            let firebase_idx =
                prompts::ask_select("Firebase:", &firebase_options, 0)?;

            let obs_options = [
                ObservabilityLevel::None,
                ObservabilityLevel::Sentry,
                ObservabilityLevel::SentryAnalytics,
            ];
            let obs_idx =
                prompts::ask_select("Observabilite:", &obs_options, 0)?;

            let extra_features = vec!["profile", "settings", "notifications"];
            let feature_defaults = vec![false, false, false];
            let selected_feature_indices = prompts::ask_multiselect(
                "Features additionnelles (home toujours inclus):",
                &extra_features,
                &feature_defaults,
            )?;
            let selected_features: Vec<String> = selected_feature_indices
                .iter()
                .map(|&i| extra_features[i as usize].to_string())
                .collect();

            let options = CodeWithAndreaOptions {
                firebase: firebase_options[firebase_idx],
                observability: obs_options[obs_idx],
                features: selected_features,
            };

            CodeWithAndreaTemplate::build_config(&project_name, &options)
        }
        _ => unreachable!(),
    };

    println!("\n{}", style("Recapitulatif :").bold());
    println!(" - Projet : {}", style(&project_name).green());
    println!(" - Template : {}", style(&config.name).green());
    println!(
        " - Features : {}",
        style(config.features.join(", ")).green()
    );

    let confirm = prompts::ask_confirm("Generer ce projet maintenant ?", true)?;

    if !confirm {
        println!("{}", style("Operation annulee.").red());
        return Ok(());
    }

    println!();

    // Step 1: flutter create
    flutter::create_project(&project_name)?;
    println!("  {} Projet de base cree.", style("✓").green());

    // Step 2: Generate structure
    let dirs_created = structure::generate(&project_name, &config.structure)?;
    println!("  {} {} dossiers crees.", style("✓").green(), dirs_created);

    // Step 3: Generate files
    let files_created = if is_code_with_andrea {
        let tmpl_map = CodeWithAndreaTemplate::templates(&project_name);
        files::generate_code_with_andrea(
            &project_name,
            &project_name,
            &config.structure,
            &config.flags,
            &tmpl_map,
        )?
    } else {
        files::generate(&project_name, &project_name, &config.structure)?
    };
    println!(
        "  {} {} fichiers generes.",
        style("✓").green(),
        files_created
    );

    // Step 4: Root files (code_with_andrea only)
    if is_code_with_andrea && !config.root_files.is_empty() {
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

    // Step 5: Update pubspec & get
    pubspec::add_dependencies(&project_name, &config.packages)?;
    if is_code_with_andrea && !config.dev_packages.is_empty() {
        pubspec::add_dev_dependencies(&project_name, &config.dev_packages)?;
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
