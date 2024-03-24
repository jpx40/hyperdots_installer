use crate::utils::read_app_list;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AppList {
    pub system: Option<HashMap<String, bool>>,
    pub displaymanager: Option<HashMap<String, bool>>,
    pub extra: Option<HashMap<String, Option<HashMap<String, bool>>>>,
    pub shell: Option<HashMap<String, bool>>,
    pub windowmanager: Option<HashMap<String, bool>>,
    pub dependicies: Option<HashMap<String, bool>>,
    pub theming: Option<HashMap<String, bool>>,
    pub hyperdots: Option<HashMap<String, bool>>,
}

pub fn app_array() -> &'static Mutex<Vec<String>> {
    static ARRAY: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    ARRAY.get_or_init(|| Mutex::new(vec![]))
}

pub fn add_app(app: String) {
    app_array().lock().unwrap().push(app);
}

pub fn install_all() {}
pub fn check_arguments_from_config() {}

pub fn install_from_config() {
    let _ = read_app_list();
}
pub fn install() {
    println!("Installing: {:?}", app_array().lock().unwrap());
}

pub enum EditorList {
    VsCode,
    Code,
    Neovim,
    Intellij,
    //    Unknown,
    Any,
}
pub fn install_editor(editor: EditorList) {
    let _ = read_app_list();

    match editor {
        EditorList::Intellij => add_app("intellij-idea-community-edition".to_string()),
        EditorList::VsCode => add_app("vscode".to_string()),
        EditorList::Code => add_app("code".to_string()),
        EditorList::Neovim => add_app("neovim".to_string()),
        EditorList::Any => {
            add_app("code".to_string());
            add_app("neovim".to_string());
            add_app("vscode".to_string());
            add_app("intellij-idea-community-edition".to_string());
        }
    }
}
