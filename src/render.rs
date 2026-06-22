use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::{repo::Repo, walk::CandidateFile};

const FILE_SEPARATOR: &str = "============================================================";

#[derive(Debug, Clone)]
struct TreeNode {
    children: BTreeMap<String, TreeNode>,
    is_file: bool,
}

impl TreeNode {
    fn directory() -> Self {
        Self {
            children: BTreeMap::new(),
            is_file: false,
        }
    }

    fn file() -> Self {
        Self {
            children: BTreeMap::new(),
            is_file: true,
        }
    }
}

pub fn render_digest(repo: &Repo, files: &[CandidateFile]) -> Result<String> {
    let mut output = String::new();

    render_header(&mut output, repo, files);
    render_tree_section(&mut output, repo, files);
    render_files_section(&mut output, files)?;

    Ok(output)
}

fn render_header(output: &mut String, repo: &Repo, files: &[CandidateFile]) {
    let total_bytes: u64 = files.iter().map(|file| file.size_bytes).sum();

    output.push_str("# Repository Context\n\n");
    output.push_str(&format!("Repository: {}\n", repo.name()));
    output.push_str(&format!("Root: {}\n", repo.root().display()));
    output.push_str(&format!("Files: {}\n", files.len()));
    output.push_str(&format!("Bytes: {}\n", total_bytes));
    output.push('\n');
}

fn render_tree_section(output: &mut String, repo: &Repo, files: &[CandidateFile]) {
    output.push_str("## Directory structure\n\n");
    output.push_str("```text\n");
    output.push_str(repo.name());
    output.push_str("/\n");

    let tree = build_tree(files);

    let children = tree.children.iter().collect::<Vec<_>>();

    for (index, (name, node)) in children.iter().enumerate() {
        let is_last = index + 1 == children.len();
        render_tree_node(output, name, node, "", is_last);
    }

    output.push_str("```\n\n");
}

fn build_tree(files: &[CandidateFile]) -> TreeNode {
    let mut root = TreeNode::directory();

    for file in files {
        insert_path(&mut root, &file.relative_path);
    }

    root
}

fn insert_path(root: &mut TreeNode, path: &Path) {
    let components = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    let mut current = root;

    for (index, component) in components.iter().enumerate() {
        let is_file = index + 1 == components.len();

        current = current
            .children
            .entry(component.clone())
            .or_insert_with(|| {
                if is_file {
                    TreeNode::file()
                } else {
                    TreeNode::directory()
                }
            });
    }
}

fn render_tree_node(output: &mut String, name: &str, node: &TreeNode, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };

    output.push_str(prefix);
    output.push_str(connector);
    output.push_str(name);

    if !node.is_file {
        output.push('/');
    }

    output.push('\n');

    if node.children.is_empty() {
        return;
    }

    let next_prefix = if is_last {
        format!("{prefix}    ")
    } else {
        format!("{prefix}│   ")
    };

    let children = node.children.iter().collect::<Vec<_>>();

    for (index, (child_name, child_node)) in children.iter().enumerate() {
        let child_is_last = index + 1 == children.len();
        render_tree_node(output, child_name, child_node, &next_prefix, child_is_last);
    }
}

fn render_files_section(output: &mut String, files: &[CandidateFile]) -> Result<()> {
    output.push_str("## Files\n\n");

    let mut seen_paths = BTreeSet::<PathBuf>::new();

    for file in files {
        if !seen_paths.insert(file.relative_path.clone()) {
            continue;
        }

        let contents = fs::read_to_string(&file.absolute_path).with_context(|| {
            format!(
                "failed to read file contents for {}",
                file.relative_path.display()
            )
        })?;

        output.push_str(FILE_SEPARATOR);
        output.push('\n');
        output.push_str("FILE: ");
        output.push_str(&file.relative_path.display().to_string());
        output.push('\n');
        output.push_str(FILE_SEPARATOR);
        output.push('\n');
        output.push_str(&contents);

        if !contents.ends_with('\n') {
            output.push('\n');
        }

        output.push('\n');
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::walk::CandidateFile;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn unique_temp_dir() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();

        std::env::temp_dir().join(format!("repoctx-render-test-{nonce}"))
    }

    #[test]
    fn renders_tree_with_nested_files() {
        let files = vec![
            CandidateFile {
                absolute_path: PathBuf::from("/tmp/repo/src/main.rs"),
                relative_path: PathBuf::from("src/main.rs"),
                size_bytes: 10,
            },
            CandidateFile {
                absolute_path: PathBuf::from("/tmp/repo/Cargo.toml"),
                relative_path: PathBuf::from("Cargo.toml"),
                size_bytes: 10,
            },
        ];

        let mut output = String::new();
        render_tree_section(&mut output, &fake_repo("/tmp/repo"), &files);

        assert!(output.contains("repo/"));
        assert!(output.contains("Cargo.toml"));
        assert!(output.contains("src/"));
        assert!(output.contains("main.rs"));
    }

    #[test]
    fn renders_file_blocks_with_path_headers() {
        let root = unique_temp_dir();
        fs::create_dir_all(root.join("src")).expect("test directory should be created");
        fs::write(root.join("src/main.rs"), "fn main() {}\n").expect("test file should be written");

        let files = vec![CandidateFile {
            absolute_path: root.join("src/main.rs"),
            relative_path: PathBuf::from("src/main.rs"),
            size_bytes: 13,
        }];

        let mut output = String::new();
        render_files_section(&mut output, &files).expect("files section should render");

        assert!(output.contains("FILE: src/main.rs"));
        assert!(output.contains("fn main() {}"));

        fs::remove_dir_all(root).expect("test directory should be removed");
    }

    fn fake_repo(path: &str) -> Repo {
        Repo::from_root_for_test(PathBuf::from(path))
    }
}
