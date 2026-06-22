use anyhow::Result;

use crate::{
    cli::Cli, clipboard::copy_to_clipboard, filter::FileFilters, render::render_digest, repo::Repo,
    walk::collect_candidate_files,
};

pub fn run(cli: Cli) -> Result<()> {
    let repo = Repo::discover()?;
    let filters = FileFilters::from_patterns(&cli.include, &cli.exclude)?;
    let files = collect_candidate_files(repo.root(), &filters)?;
    let digest = render_digest(&repo, &files)?;

    if cli.stdout {
        print!("{digest}");
        eprintln!("ctx rendered {} files, {} bytes", files.len(), digest.len());
    } else {
        copy_to_clipboard(&digest)?;
        eprintln!(
            "ctx copied {} files, {} bytes to clipboard",
            files.len(),
            digest.len()
        );
    }

    Ok(())
}
