#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            run_mission,
            list_missions,
            get_mission,
            doctor,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").map(|window| window.open_devtools());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn run_mission(goal: String) -> Result<String, String> {
    log::info!("Running mission: {}", goal);
    Ok(format!("Mission started: {}", goal))
}

#[tauri::command]
fn list_missions() -> Result<Vec<String>, String> {
    log::info!("Listing missions");
    Ok(vec![
        "mission_1".to_string(),
        "mission_2".to_string(),
    ])
}

#[tauri::command]
fn get_mission(id: String) -> Result<serde_json::Value, String> {
    log::info!("Getting mission: {}", id);
    Ok(serde_json::json!({
        "id": id,
        "goal": "Sample goal",
        "status": "running",
        "progress": 45
    }))
}

#[tauri::command]
fn doctor() -> Result<serde_json::Value, String> {
    log::info!("Running doctor check");
    Ok(serde_json::json!({
        "llm_runtime": "ok",
        "agent_runtime": "ok",
        "memory": "ok",
        "filesystem": "ok"
    }))
}
