use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("TOML encode error: {0}")]
    TomlEncodeError(#[from] toml::ser::Error),

    #[error("Missing field: {0}")]
    MissingField(String),
}

pub type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub inference: InferenceConfig,
    #[serde(default)]
    pub models: std::collections::HashMap<String, ModelConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    pub active_model: String,
    pub backend: String,
    pub ollama_endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub source: String,
    pub repo_id: String,
    pub filename: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            inference: InferenceConfig {
                active_model: "mistral-7b".to_string(),
                backend: "ollama".to_string(),
                ollama_endpoint: "http://localhost:11434".to_string(),
            },
            models: Default::default(),
        }
    }
}

impl Config {
    pub fn load() -> ConfigResult<Self> {
        let config_dir = Self::config_dir()?;
        let config_path = config_dir.join("config.toml");

        if !config_path.exists() {
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> ConfigResult<()> {
        let config_dir = Self::config_dir()?;
        fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;

        Ok(())
    }

    pub fn config_dir() -> ConfigResult<PathBuf> {
        let dir = dirs::home_dir()
            .ok_or(ConfigError::MissingField("HOME directory".to_string()))?
            .join(".local-ai");

        Ok(dir)
    }

    pub fn models_dir() -> ConfigResult<PathBuf> {
        let dir = Self::config_dir()?.join("models");
        Ok(dir)
    }

    pub fn model_path(&self, model_id: &str) -> ConfigResult<PathBuf> {
        let models_dir = Self::models_dir()?;
        if let Some(model_config) = self.models.get(model_id) {
            Ok(models_dir.join(&model_config.filename))
        } else {
            Err(ConfigError::MissingField(format!("Model config for {}", model_id)))
        }
    }

    pub fn set_active_model(&mut self, model_id: String) {
        self.inference.active_model = model_id;
    }

    pub fn active_model(&self) -> &str {
        &self.inference.active_model
    }

    pub fn backend(&self) -> &str {
        &self.inference.backend
    }

    pub fn ollama_endpoint(&self) -> &str {
        &self.inference.ollama_endpoint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.inference.active_model, "mistral-7b");
        assert_eq!(config.inference.backend, "ollama");
        assert_eq!(config.inference.ollama_endpoint, "http://localhost:11434");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).expect("Failed to serialize");
        assert!(toml_str.contains("active_model"));
        assert!(toml_str.contains("ollama"));
    }

    #[test]
    fn test_set_active_model() {
        let mut config = Config::default();
        config.set_active_model("llama2".to_string());
        assert_eq!(config.active_model(), "llama2");
    }
}
