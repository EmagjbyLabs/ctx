use std::path::Path;

use anyhow::{Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};

#[derive(Debug, Clone)]
pub struct FileFilters {
    includes: Option<GlobSet>,
    excludes: Option<GlobSet>,
}

impl FileFilters {
    pub fn from_patterns(includes: &[String], excludes: &[String]) -> Result<Self> {
        Ok(Self {
            includes: build_optional_globset(includes, "include")?,
            excludes: build_optional_globset(excludes, "exclude")?,
        })
    }

    pub fn allows(&self, relative_path: &Path) -> bool {
        self.includes
            .as_ref()
            .is_none_or(|includes| includes.is_match(relative_path))
            && self
                .excludes
                .as_ref()
                .is_none_or(|excludes| !excludes.is_match(relative_path))
    }
}

fn build_optional_globset(patterns: &[String], kind: &str) -> Result<Option<GlobSet>> {
    if patterns.is_empty() {
        return Ok(None);
    }

    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        let glob =
            Glob::new(pattern).with_context(|| format!("invalid {kind} pattern: {pattern}"))?;

        builder.add(glob);
    }

    let set = builder
        .build()
        .with_context(|| format!("failed to build {kind} pattern set"))?;

    Ok(Some(set))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_everything_without_patterns() {
        let filters = FileFilters::from_patterns(&[], &[]).expect("filters should build");

        assert!(filters.allows(Path::new("src/main.rs")));
        assert!(filters.allows(Path::new("Cargo.toml")));
    }

    #[test]
    fn include_patterns_allow_only_matches() {
        let filters =
            FileFilters::from_patterns(&["src/**/*.rs".to_string(), "Cargo.toml".to_string()], &[])
                .expect("filters should build");

        assert!(filters.allows(Path::new("src/main.rs")));
        assert!(filters.allows(Path::new("src/command/run.rs")));
        assert!(filters.allows(Path::new("Cargo.toml")));
        assert!(!filters.allows(Path::new("README.md")));
    }

    #[test]
    fn exclude_patterns_remove_matches() {
        let filters =
            FileFilters::from_patterns(&[], &["target/**".to_string(), "*.lock".to_string()])
                .expect("filters should build");

        assert!(filters.allows(Path::new("src/main.rs")));
        assert!(!filters.allows(Path::new("target/debug/ctx")));
        assert!(!filters.allows(Path::new("Cargo.lock")));
    }

    #[test]
    fn include_and_exclude_can_be_combined() {
        let filters = FileFilters::from_patterns(
            &["src/**/*.rs".to_string()],
            &["src/generated/**".to_string()],
        )
        .expect("filters should build");

        assert!(filters.allows(Path::new("src/main.rs")));
        assert!(!filters.allows(Path::new("src/generated/bindings.rs")));
        assert!(!filters.allows(Path::new("README.md")));
    }

    #[test]
    fn invalid_patterns_return_errors() {
        let result = FileFilters::from_patterns(&["[".to_string()], &[]);

        assert!(result.is_err());
    }
}
