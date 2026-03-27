use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "geneser",
    version,
    about = "Geneser CLI - Generate Flutter project architectures from templates",
    long_about = "Geneser is a CLI tool that generates Flutter project architectures\nfrom famous templates like CodeWithAndrea, with interactive prompts\nto customize your project setup."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Flutter project with a template architecture
    Create {
        /// Project name (optional, will be prompted if not provided)
        #[arg(short, long)]
        name: Option<String>,
    },
    /// List all available templates
    List,
    /// Manage community templates
    Template {
        #[command(subcommand)]
        action: TemplateCommands,
    },
}

#[derive(Subcommand)]
pub enum TemplateCommands {
    /// Install a community template (e.g. github:user/repo)
    Add {
        /// Source in the form github:user/repo
        source: String,
    },
    /// List installed community templates
    List,
    /// Remove an installed community template
    Remove {
        /// Template name (folder name in ~/.config/geneser/templates/)
        name: String,
    },
    /// Update all installed community templates
    Update,
    /// Show the architecture documentation of a community template
    Docs {
        /// Template name (as shown in `geneser template list`)
        name: String,
    },
}
