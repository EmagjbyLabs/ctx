use clap::Parser;

/// Copy the current local repository into prompt-ready context.
#[derive(Debug, Parser)]
#[command(name = "ctx")]
#[command(author, version, about)]
pub struct Cli {
    /// Include only files matching these glob-style patterns.
    #[arg(long, value_name = "PATTERN")]
    pub include: Vec<String>,

    /// Exclude files matching these glob-style patterns.
    #[arg(long, value_name = "PATTERN")]
    pub exclude: Vec<String>,

    /// Print the generated digest to stdout instead of copying it to the clipboard.
    #[arg(long)]
    pub stdout: bool,
}
