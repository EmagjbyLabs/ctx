use anyhow::Result;

use crate::{cli::Cli, repo::Repo, walk::collect_candidate_files};

pub fn run(cli: Cli) -> Result<()> {
    let repo = Repo::discover()?;
    let files = collect_candidate_files(repo.root())?;

    println!("ctx initialized");
    println!("repository: {}", repo.name());
    println!("root: {}", repo.root().display());

    if !cli.include.is_empty() {
        println!("include patterns: {}", cli.include.join(", "));
    }

    if !cli.exclude.is_empty() {
        println!("exclude patterns: {}", cli.exclude.join(", "));
    }

    if cli.stdout {
        println!("stdout mode: enabled");
    }

    for file in files {
        println!("{}", file.relative_path.display());
    }

    Ok(())
}
