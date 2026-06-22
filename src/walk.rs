use std::{
    ffi::OsStr,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use ignore::WalkBuilder;

const MAX_FILE_SIZE_BYTES: u64 = 512 * 1024;

const DEFAULT_EXCLUDED_DIRS: &[&str] = &[
    ".git",
    "target",
    "node_modules",
    "dist",
    "build",
    ".svelte-kit",
];

const DEFAULT_EXCLUDED_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "webp", "ico", "pdf", "zip", "tar", "gz", "xz", "7z", "rar",
    "mp4", "mov", "mp3", "wav", "ogg", "ttf", "otf", "woff", "woff2", "class", "jar", "wasm", "so",
    "dll", "dylib", "exe",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateFile {
    pub absolute_path: PathBuf,
    pub relative_path: PathBuf,
    pub size_bytes: u64,
}

pub fn collect_candidate_files(root: &Path) -> Result<Vec<CandidateFile>> {
    let mut files = Vec::new();

    let walker = WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .filter_entry(|entry| !is_default_excluded_dir(entry.path()))
        .build();

    for result in walker {
        let entry = result.context("failed to walk repository entry")?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let metadata = entry
            .metadata()
            .with_context(|| format!("failed to read metadata for {}", path.display()))?;

        if metadata.len() > MAX_FILE_SIZE_BYTES {
            continue;
        }

        if is_default_excluded_extension(path) {
            continue;
        }

        if looks_binary(path)? {
            continue;
        }

        let relative_path = path
            .strip_prefix(root)
            .with_context(|| {
                format!(
                    "failed to strip repository root {} from {}",
                    root.display(),
                    path.display()
                )
            })?
            .to_path_buf();

        files.push(CandidateFile {
            absolute_path: path.to_path_buf(),
            relative_path,
            size_bytes: metadata.len(),
        });
    }

    files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    Ok(files)
}

fn is_default_excluded_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(OsStr::to_str)
        .is_some_and(|name| DEFAULT_EXCLUDED_DIRS.contains(&name))
}

fn is_default_excluded_extension(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .map(str::to_ascii_lowercase)
        .is_some_and(|extension| DEFAULT_EXCLUDED_EXTENSIONS.contains(&extension.as_str()))
}

fn looks_binary(path: &Path) -> Result<bool> {
    let mut file = File::open(path)
        .with_context(|| format!("failed to open file for binary check: {}", path.display()))?;

    let mut buffer = [0_u8; 1024];
    let read = file
        .read(&mut buffer)
        .with_context(|| format!("failed to read file sample: {}", path.display()))?;

    Ok(buffer[..read].contains(&0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excludes_default_dirs_by_name() {
        assert!(is_default_excluded_dir(Path::new("target")));
        assert!(is_default_excluded_dir(Path::new("node_modules")));
        assert!(is_default_excluded_dir(Path::new(".git")));
        assert!(!is_default_excluded_dir(Path::new("src")));
    }

    #[test]
    fn excludes_default_binary_extensions() {
        assert!(is_default_excluded_extension(Path::new("logo.png")));
        assert!(is_default_excluded_extension(Path::new("archive.zip")));
        assert!(is_default_excluded_extension(Path::new("font.woff2")));
        assert!(!is_default_excluded_extension(Path::new("main.rs")));
    }
}
