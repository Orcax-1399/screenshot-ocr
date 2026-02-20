pub mod glm;

use anyhow::Result;

/// OCR provider trait that accepts image data and returns extracted text
pub trait OcrProvider {
    /// Perform OCR on the given image data
    ///
    /// # Arguments
    /// * `image_data` - Raw image bytes (PNG, JPEG, etc.)
    ///
    /// # Returns
    /// Extracted text content from the image
    fn extract_text(&self, image_data: &[u8]) -> Result<String>;
}
