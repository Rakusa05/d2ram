use std::{io::ErrorKind, path::PathBuf};

use crate::{AppSettings, AppState, Language, D2R_DEFAULT_PATH};

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            game_path: PathBuf::from(D2R_DEFAULT_PATH),
            game_language: Language::Default,
        }
    }
}

fn app_settings_path() -> Result<PathBuf, String> {
    let mut conf = dirs_next::config_dir().ok_or("No Config Dir")?;
    conf.push("D2RAM");
    std::fs::create_dir_all(&conf).map_err(|e| e.to_string())?;
    conf.push("settings.json");
    Ok(conf)
}

pub fn read_app_settings() -> Result<AppSettings, String> {
    let path = app_settings_path()?;
    match std::fs::read_to_string(&path) {
        Ok(txt) => {
            let s: AppSettings = serde_json::from_str(&txt)
                .map_err(|e| format!("Invalid settings.json: {e}"))?;
            Ok(s)
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let s = AppSettings::default();
            save_app_settings(&s)?;
            Ok(s)
        }
        Err(e) => Err(e.to_string()),
    }
}

fn save_app_settings(settings: &AppSettings) -> Result<(), String> {
    let path = app_settings_path()?;
    let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_settings(state: tauri::State<AppState>) -> Result<AppSettings, String> {
    let s = state.settings.lock().unwrap();
    Ok(s.clone())
}

#[tauri::command]
pub fn save_settings(new_settings: AppSettings, state: tauri::State<AppState>) -> Result<AppSettings, String> {
    {
        let mut s = state.settings.lock().unwrap();
        *s = new_settings.clone();
    }
    save_app_settings(&new_settings)?;
    Ok(new_settings)
}


