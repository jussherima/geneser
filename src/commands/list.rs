use anyhow::Result;
use console::style;

pub fn run() -> Result<()> {
    println!("\n  {}", style("Templates officiels :").bold().cyan());

    let templates = [
        (
            "CodeWithAndrea (Feature-first)",
            "riverpod / go_router",
            "Feature-first avec Riverpod, GoRouter et codegen Freezed.",
        ),
        (
            "CodeWithAndrea (Medium)",
            "riverpod / go_router",
            "Production-grade avec Firebase et Sentry optionnels, root config embarquee.",
        ),
        (
            "Feature-First MVVM + GetX",
            "getx / getx",
            "MVVM avec GetX (state + routing), stockage et HTTP configurables.",
        ),
        (
            "Clean Architecture + BLoC",
            "bloc / go_router",
            "Clean Archi (data/domain/presentation) avec BLoC, get_it/injectable, Drift/Hive.",
        ),
        (
            "Riverpod + Freezed (Minimal)",
            "riverpod / go_router",
            "Structure minimaliste Feature-First avec Riverpod, Freezed et go_router.",
        ),
        (
            "Custom",
            "au choix",
            "Selection manuelle du State Management, Routing et packages additionnels.",
        ),
    ];

    for (name, stack, desc) in &templates {
        println!();
        println!("  {} {}", style("•").cyan(), style(name).green().bold());
        println!("    Stack  : {}", style(stack).dim());
        println!("    {}",  desc);
    }

    println!("\n  {} Pour les templates communautaires :", style("i").dim());
    println!("    geneser template list");

    println!();
    Ok(())
}
