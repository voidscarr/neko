use std::{fs::File, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    path: PathBuf,
    default_path: PathBuf,
    font_size: usize,
    theme_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: Self::determine_path(),
            default_path: Self::determine_path(),
            font_size: 16,
            theme_name: "Dark".to_string(),
        }
    }
}

impl Config {
    fn determine_path() -> PathBuf {
        if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
            PathBuf::from(
                std::env::var("HOME").expect("No HOME directory.") + "/.config/neko/settings.json",
            )
        } else {
            // TODO: Test this
            PathBuf::from(
                std::env::var("APP_DATA").expect("No APP_DATA directory.")
                    + "Roaming/neko/settings.json",
            )
        }
    }

    pub fn load_config(&mut self) {
        let file = std::fs::read_to_string(self.path.clone());

        if file.is_err() {
            println!("Unable to open config file. Creating...");
            self.create_file();
        } else {
            let config: Config =
                serde_json::from_str(&file.unwrap()).expect("Unable to parse json from string.");

            self.path = config.path;
            self.default_path = config.default_path;
            self.font_size = config.font_size;
        }
    }

    pub fn new() -> Self {
        Config::default()
    }

    pub fn create_file(&mut self) {
        // TODO: Support merging of current config + new values
        let _ = File::create(self.path.clone());
        self.save_to_disk();
    }

    pub fn save_to_disk(&mut self) {
        let json = serde_json::to_string_pretty(self).expect("Unable to format config as json.");
        let mut file = File::create(self.path.clone()).expect("Unable to open config file.");
        // TODO: Handle error
        let _res = file.write_all(json.as_bytes());
    }

    pub fn font_size(&self) -> usize {
        self.font_size
    }

    pub fn set_font_size(&mut self, font_size: usize) {
        self.font_size = font_size;
        self.save_to_disk();
    }

    pub fn theme_name(&self) -> String {
        self.theme_name.clone()
    }
}
