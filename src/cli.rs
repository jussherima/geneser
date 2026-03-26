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
}
