use std::path::PathBuf;

pub mod manager;

pub use manager::TabManager;

#[derive(Clone)]
pub struct Tab {
    pub active: bool,
    pub dirty: bool,
    pub title: String,
    pub path: PathBuf,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::new()
    }
}

impl Tab {
    pub fn new() -> Self {
        // TODO: Mark other tabs as inactive
        // TODO: Add config setting for auto activating a new tab?
        Self {
            active: true,
            dirty: false,
            title: "Untitled".to_string(),
            path: "".into(),
        }
    }

    pub fn with_path(path: PathBuf) -> Self {
        Self {
            active: true,
            dirty: false,
            title: path
                .file_name()
                .expect("Unable to read tab file name.")
                .to_string_lossy()
                .to_string(),
            path,
        }
    }
}
