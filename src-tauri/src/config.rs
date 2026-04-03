use crate::models::Config;

#[tauri::command]
pub fn get_default_config() -> Config {
    Config::default()
}

#[tauri::command]
pub fn load_config(path: String) -> Result<Config, String> {
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read config: {}", e))?;
    toml::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))
}

#[tauri::command]
pub fn save_config(path: String, config: Config) -> Result<(), String> {
    let content = toml::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    std::fs::write(&path, content)
        .map_err(|e| format!("Failed to write config: {}", e))
}
