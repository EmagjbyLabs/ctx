use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};

#[derive(Debug, Clone)]
pub struct Repo {
    root: PathBuf,
}

impl Repo {
    pub fn discover() -> Result<Self> {
        let current_dir = env::current_dir().context("failed to read current working directory")?;
        let root = find_git_root(&current_dir)
            .with_context(|| format!("not inside a Git repository: {}", current_dir.display()))?;

        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        self.root.as_path()
    }

    pub fn name(&self) -> &str {
        self.root
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
    }

    #[cfg(test)]
    pub fn from_root_for_test(root: PathBuf) -> Self {
        Self { root }
    }
}

fn find_git_root(start: &Path) -> Result<PathBuf> {
    let mut current = start;

    loop {
        if current.join(".git").exists() {
            return Ok(current.to_path_buf());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => bail!("no .git directory found"),
        }
    }
}
