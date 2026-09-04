use crate::config::{Config, ConfigError, ConfigResult};
use crate::ollama::ModelInfo;
use std::fs;
use std::path::PathBuf;

pub struct ModelManager {
    config: Config,
}

impl ModelManager {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn load_config() -> ConfigResult<Self> {
        let config = Config::load()?;
        Ok(Self { config })
    }

    pub fn list_cached_models(&self) -> ConfigResult<Vec<CachedModel>> {
        let models_dir = Config::models_dir()?;
        let mut cached = Vec::new();

        if !models_dir.exists() {
            return Ok(cached);
        }

        for entry in fs::read_dir(&models_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Ok(metadata) = path.metadata() {
                    if let Some(filename) = path.file_name() {
                        let name = filename.to_string_lossy().to_string();
                        let size = metadata.len();

                        cached.push(CachedModel { name, size });
                    }
                }
            }
        }

        Ok(cached)
    }

    pub fn is_model_cached(&self, model_id: &str) -> ConfigResult<bool> {
        if let Ok(path) = self.config.model_path(model_id) {
            Ok(path.exists())
        } else {
            Ok(false)
        }
    }

    pub fn get_cached_models_dir(&self) -> ConfigResult<PathBuf> {
        let dir = Config::models_dir()?;
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    pub fn set_active_model(&mut self, model_id: String) -> ConfigResult<()> {
        self.config.set_active_model(model_id);
        self.config.save()?;
        Ok(())
    }

    pub fn active_model(&self) -> &str {
        self.config.active_model()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }
}

pub struct CachedModel {
    pub name: String,
    pub size: u64,
}

impl CachedModel {
    pub fn size_mb(&self) -> f64 {
        self.size as f64 / (1024.0 * 1024.0)
    }

    pub fn size_gb(&self) -> f64 {
        self.size as f64 / (1024.0 * 1024.0 * 1024.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cached_model_size_conversion() {
        let model = CachedModel {
            name: "test".to_string(),
            size: 1024 * 1024 * 1024, // 1 GB
        };
        assert_eq!(model.size_gb(), 1.0);
    }

    #[test]
    fn test_model_manager_active_model() {
        let config = Config::default();
        let manager = ModelManager::new(config);
        assert_eq!(manager.active_model(), "mistral-7b");
    }
}
