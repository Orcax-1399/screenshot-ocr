use super::OcrProvider;
use anyhow::{Context, Result};
use base64::Engine;
use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

pub struct GlmOcrProvider {
    api_key: String,
    endpoint: String,
    model: String,
    max_edge: u32,
    jpeg_quality: u8,
}

#[derive(Serialize)]
struct GlmRequest {
    model: String,
    file: String,
}

#[derive(Deserialize)]
struct GlmResponse {
    #[serde(default)]
    md_results: Option<String>,
    #[serde(default)]
    error: Option<GlmError>,
}

#[derive(Deserialize)]
struct GlmError {
    code: String,
    message: String,
}

impl GlmOcrProvider {
    pub fn new(
        api_key: String,
        endpoint: String,
        model: String,
        max_edge: u32,
        jpeg_quality: u8,
    ) -> Self {
        Self {
            api_key,
            endpoint,
            model,
            max_edge,
            jpeg_quality,
        }
    }

    fn process_image(&self, image_data: &[u8]) -> Result<String> {
        // Decode image
        let img = image::load_from_memory(image_data)
            .context("Failed to decode image")?;

        // Resize if needed
        let img = self.resize_if_needed(img);

        // Convert to JPEG
        let jpeg_data = self.encode_jpeg(&img)?;

        // Base64 encode
        Ok(base64::engine::general_purpose::STANDARD.encode(&jpeg_data))
    }

    fn resize_if_needed(&self, img: DynamicImage) -> DynamicImage {
        let (width, height) = img.dimensions();
        let longest_edge = width.max(height);

        if longest_edge <= self.max_edge {
            return img;
        }

        let scale = self.max_edge as f32 / longest_edge as f32;
        let new_width = (width as f32 * scale) as u32;
        let new_height = (height as f32 * scale) as u32;

        img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
    }

    fn encode_jpeg(&self, img: &DynamicImage) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        let rgb_img = img.to_rgb8();
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, self.jpeg_quality);

        rgb_img
            .write_with_encoder(encoder)
            .context("Failed to encode JPEG")?;

        Ok(buffer)
    }
}

impl OcrProvider for GlmOcrProvider {
    fn extract_text(&self, image_data: &[u8]) -> Result<String> {
        // Process image to base64
        let base64_image = self.process_image(image_data)?;

        // Create data URL
        let data_url = format!("data:image/jpeg;base64,{}", base64_image);

        // Prepare request
        let request = GlmRequest {
            model: self.model.clone(),
            file: data_url,
        };

        // Send request
        let mut response = ureq::post(&self.endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", &self.api_key)
            .send_json(&request)
            .context("Failed to send OCR request")?;

        // Parse response
        let body = response.body_mut().read_to_string()
            .context("Failed to read response body")?;

        let glm_response: GlmResponse = serde_json::from_str(&body)
            .context("Failed to parse OCR response")?;

        // Check for errors
        if let Some(error) = glm_response.error {
            anyhow::bail!("GLM API error: {} - {}", error.code, error.message);
        }

        // Extract result
        glm_response
            .md_results
            .context("No OCR result in response")
    }
}
