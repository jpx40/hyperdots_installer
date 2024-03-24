use crate::utils::{self, read_app_list};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, OnceLock};
use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
};
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

lazy_static! {
    static ref APP_LIST: AppList = utils::read_app_list().unwrap();
}

// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
pub fn app_array() -> &'static Mutex<Vec<String>> {
    static ARRAY: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    ARRAY.get_or_init(|| Mutex::new(vec![]))
}

pub fn add_app(app: &str) {
    if !app_array().lock().unwrap().contains(&app.to_string()) {
        app_array().lock().unwrap().push(app.to_string());
    }
}

pub fn install_all() {}

pub fn install_from_config() {
    let app_list = APP_LIST.clone();
    add_app_key(app_list.system);
    add_app_key(app_list.displaymanager);
    add_app_key(app_list.theming);
    add_app_key(app_list.hyperdots);
    add_app_key(app_list.dependicies);
    add_app_key(app_list.shell);
}
fn add_app_key(app: Option<HashMap<String, bool>>) {
    if let Some(app) = app {
        for (k, v) in app {
            if v {
                add_app(&k);
            }
        }
    }
}
pub fn install() -> Result<(), String> {
    let path: PathBuf = env::current_dir().unwrap();

    let mut app_file: File = File::create(path.join("custom_hypr.lst")).unwrap();

    for app in app_array().lock().unwrap().iter() {
        println!("Installing: {:?}", app);
        let buf = app.clone() + "\n";
        app_file.write_all(buf.as_bytes()).unwrap();
    }
    Ok(())
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
    let app_list = APP_LIST.clone();

    match editor {
        EditorList::Intellij => add_app("intellij-idea-community-edition"),
        EditorList::VsCode => add_app("vscode"),
        EditorList::Code => add_app("code"),
        EditorList::Neovim => add_extra_deps(app_list, "neovim"),
        EditorList::Any => {
            add_app("code");
            add_app("neovim");
            add_app("vscode");
            add_app("intellij-idea-community-edition");
        }
    }
}

fn add_extra_deps(app_list: AppList, app: &str) {
    add_app(app);

    if let Some(apps) = app_list.extra {
        for (k, v) in apps {
            if k == app {
                if let Some(v) = v {
                    v.iter().for_each(|(k, v)| {
                        if *v {
                            add_app(k);
                        }
                    });
                }
                break;
            }
        }
    }
}
