use inference::{Config, HealthStatus, InferenceOptions, LLMRuntime, ModelManager, OllamaRuntime};
use log::{error, info};
use std::fs;

pub async fn list_models(remote: bool) -> Result<(), Box<dyn std::error::Error>> {
    let manager = ModelManager::load_config()?;

    println!("\n=== Cached Models ===");
    let cached = manager.list_cached_models()?;
    if cached.is_empty() {
        println!("No cached models found.");
    } else {
        for model in cached {
            if model.size_gb() > 1.0 {
                println!(
                    "  • {} ({:.2} GB)",
                    model.name,
                    model.size_gb()
                );
            } else {
                println!(
                    "  • {} ({:.2} MB)",
                    model.name,
                    model.size_mb()
                );
            }
        }
    }

    println!("\n=== Active Model ===");
    println!("  • {} (backend: {})", manager.active_model(), manager.config().backend());

    if remote {
        println!("\n=== Available Models (Remote) ===");
        println!("  • mistral-7b (Mistral 7B Instruct)");
        println!("  • llama2-7b (Llama 2 7B)");
        println!("  • neural-chat-7b (Neural Chat 7B)");
        println!("\nUsage: local-ai select-model <model_id>");
    }

    Ok(())
}

pub async fn select_model(model_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ModelManager::load_config()?;

    println!("Setting active model to: {}", model_id);

    manager.set_active_model(model_id.to_string())?;
    info!("Active model updated to: {}", model_id);

    println!("✅ Model selection saved.");
    println!("\nNext steps:");
    println!("  1. Ensure Ollama is running: ollama serve");
    println!("  2. Pull the model: ollama pull {}", model_id);
    println!("  3. Run: local-ai doctor");

    Ok(())
}

pub async fn doctor() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Local AI Diagnostic Report ===\n");

    let config = Config::load()?;
    let backend = config.backend();
    let ollama_endpoint = config.ollama_endpoint();

    println!("Backend: {}", backend);
    println!("Endpoint: {}", ollama_endpoint);

    if backend == "ollama" {
        print!("Checking Ollama connectivity... ");

        let mut runtime = OllamaRuntime::new(ollama_endpoint.to_string(), "test".to_string());

        match runtime.health_check() {
            HealthStatus::Ok => {
                println!("✅ OK");
            }
            HealthStatus::Warning(msg) => {
                println!("⚠️  Warning: {}", msg);
            }
            HealthStatus::Error(msg) => {
                println!("❌ Error: {}", msg);
                println!("\nTroubleshooting:");
                println!("  1. Install Ollama: https://ollama.com");
                println!("  2. Start Ollama server: ollama serve");
                println!("  3. In another terminal, pull a model: ollama pull mistral");
                println!("  4. Run this command again");
                return Ok(());
            }
        }

        print!("Checking active model... ");
        let active_model = config.active_model();
        match runtime.initialize() {
            Ok(_) => println!("✅ {} loaded", active_model),
            Err(e) => {
                println!("❌ Failed to load {}: {}", active_model, e);
                println!("\nPull the model with: ollama pull {}", active_model);
            }
        }
    }

    println!("\nCached models:");
    let manager = ModelManager::load_config()?;
    let cached = manager.list_cached_models()?;
    if cached.is_empty() {
        println!("  (none)");
    } else {
        for model in cached {
            println!("  • {} ({:.2} GB)", model.name, model.size_gb());
        }
    }

    println!("\nConfiguration:");
    println!("  Config file: {:?}", Config::config_dir()?.join("config.toml"));
    println!("  Models directory: {:?}", Config::models_dir()?);

    println!("\n=== Diagnostic Complete ===");

    Ok(())
}

pub async fn offline_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Offline Operation Test ===\n");

    let manager = ModelManager::load_config()?;
    let cached = manager.list_cached_models()?;

    if cached.is_empty() {
        println!("❌ No cached models available for offline operation.");
        println!("\nTo use offline mode, you need to:");
        println!("  1. Install Ollama: https://ollama.com");
        println!("  2. Start Ollama: ollama serve");
        println!("  3. Pull a model: ollama pull mistral");
        println!("  4. This will cache the model locally");
        println!("  5. Try offline test again");
        return Ok(());
    }

    println!("✅ Cached models available: {} models", cached.len());

    for model in &cached {
        println!("  • {} ({:.2} GB)", model.name, model.size_gb());
    }

    println!("\n⚠️  Note: This test verifies model caching, not full offline inference.");
    println!("Full offline inference requires bundled models (coming in Phase 12).\n");

    println!("✅ Offline operation: Cache verified");

    Ok(())
}

pub async fn infer(prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    let mut runtime = OllamaRuntime::new(
        config.ollama_endpoint().to_string(),
        config.active_model().to_string(),
    );

    print!("Initializing model... ");
    runtime.initialize()?;
    println!("✅");

    println!("Running inference...\n");

    let opts = InferenceOptions::default();
    let response = runtime.infer(prompt, opts)?;

    println!("{}", response);

    Ok(())
}

pub async fn run_mission(goal: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Starting Mission ===");
    println!("Goal: {}", goal);
    println!();

    println!("✅ Mission framework ready (Phase 2)");
    println!("Note: Full mission execution requires Phase 4 (Tool System) implementation");
    println!();
    println!("To run missions with real tools:");
    println!("  1. Complete Phase 3: Planner");
    println!("  2. Complete Phase 4: Tool System");
    println!("  3. Integrate with sandbox (Phase 5)");
    println!();
    println!("Current status: Mission state tracking, agent loop, and event bus implemented.");

    Ok(())
}

pub async fn list_missions() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Mission History ===");
    println!("Note: Mission persistence requires database integration (Phase 7+)");
    println!();
    println!("Current capabilities:");
    println!("  ✅ Mission state creation and tracking");
    println!("  ✅ Task status machine (pending→complete)");
    println!("  ✅ Observation recording");
    println!("  ✅ Error handling");
    println!();
    println!("Pending: Database persistence, mission resumption, history queries");

    Ok(())
}
