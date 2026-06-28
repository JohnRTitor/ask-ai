use anyhow::{Context, Result};
use arboard::Clipboard;

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new().with_context(|| "Failed to access clipboard")?;
    clipboard
        .set_text(text)
        .with_context(|| "Failed to set clipboard text")?;
    Ok(())
}
