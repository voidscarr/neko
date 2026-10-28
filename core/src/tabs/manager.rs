use super::Tab;
use std::collections::HashMap;

pub struct TabManager {
    pub tabs: HashMap<String, Tab>,
}

impl TabManager {
    pub fn new() -> Self {
        let mut tabs = HashMap::new();

        tabs.insert("Untitled".to_string(), Tab::default());
        Self { tabs }
    }

    pub fn tabs(&self) -> Vec<Tab> {
        self.tabs.clone().into_values().collect()
    }

    pub fn tab_by_path(&self, path: String) -> Tab {
        self.tabs[&path].clone()
    }
}
