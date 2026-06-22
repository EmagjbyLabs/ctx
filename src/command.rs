use anyhow::Result;

use crate::{cli::Cli, render::render_digest, repo::Repo, walk::collect_candidate_files};

pub fn run(cli: Cli) -> Result<()> {
    let repo = Repo::discover()?;
    let files = collect_candidate_files(repo.root())?;
    let digest = render_digest(&repo, &files)?;

    if cli.stdout {
        print!("{digest}");
    } else {
        println!("{digest}");
    }

    eprintln!("ctx rendered {} files, {} bytes", files.len(), digest.len());

    Ok(())
}
