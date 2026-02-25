mod clipboard;
mod config;
mod notifier;
mod provider;

use anyhow::{Context, Result};
use notifier::NotificationManager;
use provider::{glm::GlmOcrProvider, OcrProvider};
use std::io::Read;

fn main() {
    if let Err(e) = run() {
        let error_msg = format!("{:#}", e);
        eprintln!("Error: {}", error_msg);
        notifier::notify_error(&error_msg);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    // Load configuration
    let config = config::load_config().context("Failed to load configuration")?;

    // Read image data from stdin
    let mut image_data = Vec::new();
    std::io::stdin()
        .read_to_end(&mut image_data)
        .context("Failed to read image from stdin")?;

    if image_data.is_empty() {
        anyhow::bail!("No image data received from stdin");
    }

    // Show processing notification immediately
    let manager = NotificationManager::show_processing();

    // Create OCR provider
    let provider = GlmOcrProvider::new(
        config.api_key,
        config.api.endpoint,
        config.api.model,
        config.image.max_edge,
        config.image.jpeg_quality,
        config.api.timeout_secs,
    );

    // Perform OCR
    let text = match provider.extract_text(&image_data) {
        Ok(text) => text,
        Err(e) => {
            manager.finish_error(&format!("{:#}", e));
            return Err(e.context("Failed to perform OCR"));
        }
    };

    // Check if text is empty
    if text.trim().is_empty() {
        manager.finish_warning("No text detected");
        return Ok(());
    }

    // Copy to clipboard
    if let Err(e) = clipboard::copy_to_clipboard(&text) {
        manager.finish_error(&format!("{:#}", e));
        return Err(e.context("Failed to copy to clipboard"));
    }

    // Show success notification
    manager.finish_success(text.chars().count());

    Ok(())
}
