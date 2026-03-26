mod cli;
mod commands;
mod embedded;
mod generators;
mod models;
mod templates;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};
use console::style;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Create { name }) => commands::create::run(name),
        Some(Commands::List) => commands::list::run(),
        None => commands::create::run(None),
    };

    if let Err(e) = result {
        eprintln!("\n  {} {}", style("✗").red().bold(), style(e).red());
        std::process::exit(1);
    }
}
