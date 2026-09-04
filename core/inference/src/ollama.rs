use crate::runtime::{HealthStatus, InferenceOptions, LLMRuntime, RuntimeError, RuntimeResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct OllamaRuntime {
    endpoint: String,
    client: Client,
    model: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

#[derive(Debug, Deserialize)]
struct TagsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub size: u64,
    pub digest: String,
}

impl OllamaRuntime {
    pub fn new(endpoint: String, model: String) -> Self {
        Self {
            endpoint,
            client: Client::new(),
            model,
        }
    }

    pub async fn list_models(&self) -> RuntimeResult<Vec<ModelInfo>> {
        let url = format!("{}/api/tags", self.endpoint);
        let response = self.client.get(&url).send().await?;
        let body: TagsResponse = response.json().await?;
        Ok(body.models)
    }

    pub fn set_model(&mut self, model: String) {
        self.model = model;
    }
}

impl LLMRuntime for OllamaRuntime {
    fn initialize(&mut self) -> RuntimeResult<()> {
        match self.health_check() {
            HealthStatus::Ok => Ok(()),
            HealthStatus::Warning(msg) => Err(RuntimeError::InitError(msg)),
            HealthStatus::Error(msg) => Err(RuntimeError::InitError(msg)),
        }
    }

    fn infer(&self, prompt: &str, opts: InferenceOptions) -> RuntimeResult<String> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| RuntimeError::InferenceError(format!("Failed to create async runtime: {}", e)))?;
        rt.block_on(self.infer_async(prompt, opts))
    }

    fn health_check(&self) -> HealthStatus {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return HealthStatus::Error("Failed to create async runtime".to_string()),
        };

        match rt.block_on(self.health_check_async()) {
            Ok(()) => HealthStatus::Ok,
            Err(e) => {
                log::warn!("Ollama health check failed: {}", e);
                HealthStatus::Error(format!("Ollama unreachable: {}", e))
            }
        }
    }

    fn name(&self) -> &str {
        "ollama"
    }
}

impl OllamaRuntime {
    async fn infer_async(&self, prompt: &str, opts: InferenceOptions) -> RuntimeResult<String> {
        let url = format!("{}/api/generate", self.endpoint);

        let request = GenerateRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
            num_predict: Some(opts.max_tokens as i32),
            temperature: Some(opts.temperature),
            top_p: Some(opts.top_p),
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(RuntimeError::InferenceError(format!(
                "Ollama returned status: {}",
                response.status()
            )));
        }

        let body: GenerateResponse = response.json().await?;
        Ok(body.response)
    }

    async fn health_check_async(&self) -> RuntimeResult<()> {
        let url = format!("{}/api/tags", self.endpoint);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(RuntimeError::HealthCheckFailed(format!(
                "Ollama returned status: {}",
                response.status()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_runtime_creation() {
        let runtime = OllamaRuntime::new("http://localhost:11434".to_string(), "mistral".to_string());
        assert_eq!(runtime.name(), "ollama");
        assert_eq!(runtime.model, "mistral");
    }

    #[test]
    fn test_inference_options_default() {
        let opts = InferenceOptions::default();
        assert_eq!(opts.max_tokens, 512);
        assert_eq!(opts.temperature, 0.7);
        assert_eq!(opts.top_p, 0.95);
    }
}
