mod cli;
mod commands;
mod config;
mod embedded;
mod generators;
mod models;
mod registry;
mod templates;
mod ui;

use clap::Parser;
use cli::{Cli, Commands, TemplateCommands};
use commands::template::TemplateAction;
use console::style;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Create { name }) => commands::create::run(name),
        Some(Commands::List) => commands::list::run(),
        Some(Commands::Template { action }) => {
            let action = match action {
                TemplateCommands::Add { source } => TemplateAction::Add { source },
                TemplateCommands::List => TemplateAction::List,
                TemplateCommands::Remove { name } => TemplateAction::Remove { name },
                TemplateCommands::Update => TemplateAction::Update,
                TemplateCommands::Docs { name } => TemplateAction::Docs { name },
            };
            commands::template::run(action)
        }
        None => commands::create::run(None),
    };

    if let Err(e) = result {
        eprintln!("\n  {} {}", style("✗").red().bold(), style(e).red());
        std::process::exit(1);
    }
}
