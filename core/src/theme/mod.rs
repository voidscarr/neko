pub mod manager;

pub use manager::ThemeManager;

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub background: String,
    pub foreground: String,
    pub selection: String,
    pub cursor: String,
}

impl Theme {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            background: "#000000".to_string(),
            foreground: "#ffffff".to_string(),
            selection: "#40a162a1".to_string(),
            cursor: "#a162a1".to_string(),
        }
    }

    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            background: "#ffffff".to_string(),
            foreground: "#000000".to_string(),
            selection: "#40a162a1".to_string(),
            cursor: "#a162a1".to_string(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
