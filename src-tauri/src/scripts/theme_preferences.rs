use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use tauri::command;
use tauri::Manager::path::app_config_dir;


fn get_theme_config_file_path() -> Result<PathBuf, String> {
    let config_dir = app_config_dir(&tauri::Config::default())
        .ok_or_else(|| "Impossible d'obtenir le répertoire de configuration de l'application".to_string())?;
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }
    let config_file = config_dir.join("theme_selected.json");
    Ok(config_file)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThemeSelected {
    primary_color: String,
}

#[command]
pub fn save_theme_selected(data: ThemeSelected) -> Result<(), String> {
    let config_path = get_theme_config_file_path().map_err(|e| e.to_string())?;
    let json_data = serde_json::to_string(&data).map_err(|e| e.to_string())?;
    fs::write(config_path, json_data).map_err(|e| e.to_string())
}

#[command]
pub fn load_theme_selected() -> Result<ThemeSelected, String> {
    let config_path = get_theme_config_file_path().map_err(|e| e.to_string())?;
    if !config_path.exists() {
        return Ok(ThemeSelected {
            primary_color: "#6463b6".to_string(),
        });
    }
    let json_data = fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    let data: ThemeSelected = serde_json::from_str(&json_data).map_err(|e| e.to_string())?;

    Ok(data)
}