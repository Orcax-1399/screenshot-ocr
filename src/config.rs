use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: String,
    #[serde(default)]
    pub image: ImageConfig,
    #[serde(default)]
    pub api: ApiConfig,
}

#[derive(Debug, Deserialize)]
pub struct ImageConfig {
    #[serde(default = "default_max_edge")]
    pub max_edge: u32,
    #[serde(default = "default_jpeg_quality")]
    pub jpeg_quality: u8,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    #[serde(default = "default_model")]
    pub model: String,
}

fn default_max_edge() -> u32 {
    1920
}

fn default_jpeg_quality() -> u8 {
    85
}

fn default_endpoint() -> String {
    "https://open.bigmodel.cn/api/paas/v4/layout_parsing".to_string()
}

fn default_model() -> String {
    "glm-ocr".to_string()
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            max_edge: default_max_edge(),
            jpeg_quality: default_jpeg_quality(),
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            endpoint: default_endpoint(),
            model: default_model(),
        }
    }
}

pub fn load_config() -> Result<Config> {
    // Try config file first
    if let Ok(config) = load_from_file() {
        return Ok(config);
    }

    // Fall back to environment variable
    if let Ok(api_key) = std::env::var("GLM_API_KEY") {
        return Ok(Config {
            api_key,
            image: ImageConfig::default(),
            api: ApiConfig::default(),
        });
    }

    anyhow::bail!("API key not configured. Set GLM_API_KEY environment variable or create ~/.config/screenshot-ocr/config.toml")
}

fn load_from_file() -> Result<Config> {
    let config_path = get_config_path()?;
    let content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

    toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", config_path.display()))
}

fn get_config_path() -> Result<PathBuf> {
    let home = std::env::var("HOME").context("HOME environment variable not set")?;
    Ok(PathBuf::from(home).join(".config/screenshot-ocr/config.toml"))
}
