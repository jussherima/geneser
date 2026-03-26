use anyhow::Result;
use console::style;

pub fn run() -> Result<()> {
    println!("\n  {}", style("Templates disponibles :").bold().cyan());

    println!("  - {}", style("CodeWithAndrea").green().bold());
    println!("    Architecture Feature-first avec Riverpod et GoRouter.");

    println!("  - {}", style("Custom").green().bold());
    println!("    Selection manuelle du State Management et Routing.");

    println!("  - {}", style("Fybego").green().bold());
    println!("    Production-grade avec Riverpod, GoRouter, Firebase et Sentry optionnels.");

    println!();
    Ok(())
}
