use crate::generators::{files, flutter, pubspec, structure};
use crate::models::options::{ExtraPackage, RoutingSolution, StateManagement};
use crate::templates::custom::CustomTemplate;
use crate::templates::cwa::CwaTemplate;
use crate::ui::prompts;
use anyhow::Result;
use console::style;

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
    ];
    let template_idx = prompts::ask_select("Choisissez un template:", &templates, 0)?;

    let config = if template_idx == 0 {
        // CWA Workflow
        let features = vec![
            "authentication",
            "home",
            "products",
            "cart",
            "orders",
            "reviews",
        ];

        // Defaults to true for basic features
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
    } else {
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
        // default all false
        let defaults = vec![false; extra_options.len()];
        let selected_extra_indices =
            prompts::ask_multiselect("Packages additionnels:", &extra_options, &defaults)?;

        let extras: Vec<ExtraPackage> = selected_extra_indices
            .iter()
            .map(|&i| extra_options[i])
            .collect();

        // Simple feature selection for custom
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
    let files_created = files::generate(&project_name, &project_name, &config.structure)?;
    println!(
        "  {} {} fichiers generes.",
        style("✓").green(),
        files_created
    );

    // Step 4: Update pubspec & get
    pubspec::add_dependencies(&project_name, &config.packages)?;
    flutter::pub_get(&project_name)?;
    println!(
        "  {} {} packages ajoutes.",
        style("✓").green(),
        config.packages.len()
    );

    println!(
        "\n  {} {}",
        style("Succes !").green().bold(),
        style(format!("Projet {} pret.", project_name)).bold()
    );
    println!("  cd {}", project_name);

    Ok(())
}
