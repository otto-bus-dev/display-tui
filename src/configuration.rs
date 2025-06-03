use std::path::{Path, PathBuf};
use std::fs;
use serde::Deserialize;
#[derive(Debug,Default, Clone, Deserialize)]
pub struct Configuration {
    pub monitors_config_path: String,
}
impl Configuration {
    pub fn get() -> Self {
        let config_json_path = dirs::home_dir()
             .map(|p| p.join(".config/display-tui/config.json"))
             .unwrap_or_else(|| Path::new("~/.config/display-tui/config.json").to_path_buf());
        match !config_json_path.exists() {
            true => {
                Configuration::create_default_config(&config_json_path)
            },
            false => {
                Configuration::load_config()
            }
        }


    }
    fn create_default_config(config_json_path: &PathBuf) -> Self {
        let default_monitors_config_path = "~/.config/hypr/hyprland/monitors.conf";
        let default_config =format!("{{\n  \"monitors_config_path\": \"{}\"\n}}", default_monitors_config_path);
        fs::create_dir_all(config_json_path.parent().unwrap()).expect("Failed to create config directory");
        fs::write(config_json_path, default_config).expect("Failed to write default config file");
        Configuration {
            monitors_config_path: default_monitors_config_path.to_string(),
        } 
    }
    fn load_config() -> Self {
        let config_json_path = dirs::home_dir()
            .map(|p| p.join(".config/display-tui/config.json"))
            .unwrap_or_else(|| Path::new("~/.config/display-tui/config.json").to_path_buf());
        
        let config_content = fs::read_to_string(config_json_path)
            .expect("Failed to read config file");
        
        let config: Configuration = serde_json::from_str(&config_content)
            .expect("Failed to parse config file");
        
        config
    }
}
