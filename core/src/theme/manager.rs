use std::collections::HashMap;

use super::Theme;

pub struct ThemeManager {
    themes: HashMap<String, Theme>,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut themes = HashMap::new();

        themes.insert("Dark".to_string(), Theme::dark());
        themes.insert("Light".to_string(), Theme::light());

        Self { themes }
    }

    pub fn theme_by_name(&self, name: String) -> Theme {
        self.themes[&name].clone()
    }

    pub fn themes(&self) -> Vec<Theme> {
        self.themes.clone().into_values().collect()
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
