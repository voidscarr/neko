use crate::{Cursor, Editor, ThemeManager, core::Config, tabs::TabManager};
use ffi::{CursorFFI, TabFFI, ThemeFFI};

fn new_editor() -> Box<Editor> {
    Box::new(Editor::new())
}

fn new_config() -> Box<Config> {
    Box::new(Config::new())
}

fn new_theme_manager() -> Box<ThemeManager> {
    Box::new(ThemeManager::new())
}

fn new_tab_manager() -> Box<TabManager> {
    Box::new(TabManager::new())
}

#[cxx::bridge]
#[allow(clippy::module_inception)]
mod ffi {
    struct CursorFFI {
        row: usize,
        column: usize,
        sticky_column: usize,
    }

    struct ThemeFFI {
        name: String,
        background: String,
        foreground: String,
        selection: String,
        cursor: String,
    }

    struct TabFFI {
        active: bool,
        dirty: bool,
        title: String,
        path: String,
    }

    extern "Rust" {
        type Editor;
        type Config;
        type ThemeManager;
        type TabManager;

        fn new_editor() -> Box<Editor>;
        fn new_config() -> Box<Config>;
        fn new_theme_manager() -> Box<ThemeManager>;
        fn new_tab_manager() -> Box<TabManager>;

        // Editor
        fn insert_char(self: &mut Editor, index: usize, text: &str);
        fn remove_char(self: &mut Editor, range_start: usize, range_end: usize);
        fn pos_to_index_ffi(editor: &Editor, cursor: &CursorFFI) -> usize;
        fn cursor_col(self: &Editor) -> usize;
        fn cursor_row(self: &Editor) -> usize;
        fn selection_start_ffi(editor: &Box<Editor>) -> CursorFFI;
        fn selection_end_ffi(editor: &Box<Editor>) -> CursorFFI;
        fn selection_active(self: &Editor) -> bool;
        fn delete_selection_and_clear(self: &mut Editor);
        fn content_slice(self: &Editor, range_start: usize, range_end: usize) -> String;
        fn selected_text(self: &Editor) -> String;
        fn select_all(self: &mut Editor);
        fn line(self: &Editor, row: usize) -> String;
        fn line_len(self: &Editor, row: usize) -> usize;
        fn content_len(self: &Editor) -> usize;
        fn move_left(self: &mut Editor, amount: usize, clear_selection: bool);
        fn move_right(self: &mut Editor, amount: usize, clear_selection: bool);
        fn move_down(self: &mut Editor, amount: usize, clear_selection: bool);
        fn move_up(self: &mut Editor, amount: usize, clear_selection: bool);
        fn num_rows(self: &Editor) -> usize;

        // Config
        fn font_size(self: &Config) -> usize;
        fn set_font_size(self: &mut Config, font_size: usize);
        fn load_config(self: &mut Config);
        fn theme_name(self: &Config) -> String;

        // ThemeManager
        fn themes_ffi(theme_manager: &ThemeManager) -> Vec<ThemeFFI>;
        fn theme_by_name_ffi(theme_manager: &ThemeManager, name: String) -> ThemeFFI;

        // TabManager
        fn tabs_ffi(tab_manager: &TabManager) -> Vec<TabFFI>;
        fn tab_by_path_ffi(tab_manager: &TabManager, path: String) -> TabFFI;
    }
}

fn selection_start_ffi(editor: &Editor) -> CursorFFI {
    let start = editor.selection_start();

    CursorFFI {
        row: start.row(),
        column: start.column(),
        sticky_column: start.sticky_column(),
    }
}

fn selection_end_ffi(editor: &Editor) -> CursorFFI {
    let end = editor.selection_end();

    CursorFFI {
        row: end.row(),
        column: end.column(),
        sticky_column: end.sticky_column(),
    }
}

fn pos_to_index_ffi(editor: &Editor, cursor: &CursorFFI) -> usize {
    let mut c = Cursor::new();

    c.move_to(cursor.row, cursor.column);
    editor.pos_to_index(&c)
}

fn themes_ffi(theme_manager: &ThemeManager) -> Vec<ThemeFFI> {
    let mut themes_list: Vec<ThemeFFI> = vec![];

    for theme in theme_manager.themes() {
        let theme_ffi = ThemeFFI {
            name: theme.name.clone(),
            background: theme.background.clone(),
            foreground: theme.foreground.clone(),
            selection: theme.selection.clone(),
            cursor: theme.cursor.clone(),
        };

        themes_list.push(theme_ffi);
    }

    themes_list
}

fn theme_by_name_ffi(theme_manager: &ThemeManager, name: String) -> ThemeFFI {
    let theme = theme_manager.theme_by_name(name);

    ThemeFFI {
        name: theme.name,
        background: theme.background,
        foreground: theme.foreground,
        selection: theme.selection,
        cursor: theme.cursor,
    }
}

fn tabs_ffi(tab_manager: &TabManager) -> Vec<TabFFI> {
    let mut tabs: Vec<TabFFI> = vec![];

    for tab in tab_manager.tabs() {
        let tab_ffi = TabFFI {
            active: tab.active,
            dirty: tab.dirty,
            title: tab.title,
            path: tab.path.to_string_lossy().to_string(),
        };

        tabs.push(tab_ffi);
    }

    tabs
}

fn tab_by_path_ffi(tab_manager: &TabManager, path: String) -> TabFFI {
    let tab = tab_manager.tab_by_path(path.clone());

    TabFFI {
        active: tab.active,
        dirty: tab.dirty,
        title: tab.title,
        path,
    }
}
