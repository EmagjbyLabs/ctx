use anyhow::{Context, Result};
use arboard::Clipboard;

pub fn copy_to_clipboard(contents: &str) -> Result<()> {
    let mut clipboard = Clipboard::new()
        .context("failed to access system clipboard; try running with --stdout instead")?;

    clipboard
        .set_text(contents.to_owned())
        .context("failed to copy digest to clipboard")
}
