// System Update Manager - Cross-platform update manager
// Backend modules
pub mod backend;
pub mod adapters;

use backend::services::{scanner, engine};
use adapters::create_platform_adapter;

// Tauri commands
#[tauri::command]
async fn scan_system() -> Result<scanner::ScanResult, String> {
    let adapter = create_platform_adapter();
    let scanner = scanner::UpdateScanner::new(adapter);
    
    scanner.scan_system()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_updates(scan_result: scanner::ScanResult) -> Result<Vec<scanner::UpdateInfo>, String> {
    let adapter = create_platform_adapter();
    let scanner = scanner::UpdateScanner::new(adapter);
    
    scanner.check_updates(&scan_result)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn apply_update(update: scanner::UpdateInfo) -> Result<backend::models::UpdateResult, String> {
    let adapter = create_platform_adapter();
    let engine = engine::UpdateEngine::new(adapter);
    
    engine.apply_update(&update)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn rollback_update(package: backend::models::Package, target_version: String) -> Result<backend::models::UpdateResult, String> {
    let adapter = create_platform_adapter();
    // Assuming we add rollback to engine, or just call adapter directly here for simplicity
    adapter.rollback_package(&package, &target_version)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn apply_batch_updates(updates: Vec<scanner::UpdateInfo>) -> Result<engine::BatchUpdateResult, String> {
    let adapter = create_platform_adapter();
    let engine = engine::UpdateEngine::new(adapter);
    
    engine.apply_batch_updates(updates)
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scan_system,
            check_updates,
            apply_update,
            rollback_update,
            apply_batch_updates
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
