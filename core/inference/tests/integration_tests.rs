use inference::{Config, ModelManager};

#[test]
fn test_config_load_and_save() {
    let config = Config::default();
    assert_eq!(config.active_model(), "mistral-7b");
    assert_eq!(config.backend(), "ollama");
    assert_eq!(config.ollama_endpoint(), "http://localhost:11434");
}

#[test]
fn test_model_manager_creation() {
    let config = Config::default();
    let manager = ModelManager::new(config);
    assert_eq!(manager.active_model(), "mistral-7b");
}

#[test]
fn test_model_directory_creation() {
    let result = Config::models_dir();
    assert!(result.is_ok(), "Should be able to get models directory");
}
