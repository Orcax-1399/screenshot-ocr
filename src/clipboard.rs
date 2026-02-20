use anyhow::{Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut child = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn()
        .context("Failed to spawn wl-copy")?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(text.as_bytes())
            .context("Failed to write to wl-copy stdin")?;
    }

    let status = child.wait().context("Failed to wait for wl-copy")?;

    if !status.success() {
        anyhow::bail!("wl-copy exited with non-zero status");
    }

    Ok(())
}
