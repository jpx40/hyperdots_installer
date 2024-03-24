use crate::utils::{self, read_app_list};
use alpm;
use home::home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::io::Write;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, OnceLock};
use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
};

type Extra = Option<HashMap<String, HashMap<String, HashMap<String, bool>>>>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Kategorie(Option<HashMap<String, Option<HashMap<String, bool>>>>);
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AppList {
    pub system: Option<HashMap<String, bool>>,
    pub displaymanager: Option<HashMap<String, bool>>,
    // pub extra: Option<HashMap<String, Option<HashMap<String, bool>>>>,
    pub extra: Option<Extra>,
    pub shell: Option<HashMap<String, bool>>,
    pub windowmanager: Option<HashMap<String, bool>>,
    pub dependicies: Option<HashMap<String, bool>>,
    pub theming: Option<HashMap<String, bool>>,
    pub hyperdots: Option<HashMap<String, bool>>,
}

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
// enum Extra {
//     HashMap,
//     String,
// }

lazy_static! {
    static ref APP_LIST: AppList = utils::read_app_list().unwrap_or_else(|err| panic!("{}", err));
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
pub fn add_line(line: &str) {
    let line = line.to_string() + "\n";
    params_file().lock().unwrap().push_str(&line);
}

pub fn params_file() -> &'static Mutex<String> {
    static PARAMS: OnceLock<Mutex<String>> = OnceLock::new();
    PARAMS.get_or_init(|| Mutex::new(String::new()))
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
    install_applications();
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

fn write_to_file() {
    if app_array().lock().unwrap().contains(&"neovim".to_string()) {
        add_line(r#"export INSTALL_NEOVIM="true""#);
    } else {
        add_line(r#"export INSTALL_NEOVIM="false""#);
    }
}
pub fn install() -> Result<(), String> {
    write_to_file();
    let path: PathBuf = env::current_dir().unwrap();

    if !Path::new("~/.cache/hyprdots").exists() {
        create_dir_all(Path::new("~/.cache/hyprdots")).unwrap_or_else(|err| panic!("{}", err));
    }
    let cache_path = Path::new("~/.cache/hyprdots/");
    let mut file: File = File::create(cache_path.join("custom_hypr.lst")).unwrap();
    let mut app_file: File = File::create(path.join("custom_hypr.lst")).unwrap();
    file.write_all(params_file().lock().unwrap().as_bytes())
        .unwrap_or_else(|err| panic!("{}", err));
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

// pub fn is_installed(app: &str) -> bool {
//     let installed = false;
//     if alpm::Dbpkg(app) {
//         installed
//     } else {
//         installed
//     }
//     installed
// }

fn install_applications() {
    let app_list = APP_LIST.clone();
    add_extra_deps(app_list, "application", "apps")
}
pub fn install_editor(editor: EditorList) {
    let app_list = APP_LIST.clone();

    match editor {
        EditorList::Intellij => add_app("intellij-idea-community-edition"),
        EditorList::VsCode => add_app("visual-studio-code-bin"),
        EditorList::Code => add_app("code"),
        EditorList::Neovim => {
            add_extra_deps(app_list, "neovim", "editor");
        }
        EditorList::Any => {
            add_extra_deps(app_list, "any", "editor");
        }
    }
}

fn add_extra_deps(app_list: AppList, app: &str, kategorie: &str) {
    if let Some(apps) = app_list.extra {
        for (k, v) in apps.clone().unwrap() {
            if kategorie == k {
                for (ak, av) in v {
                    if ak == app || app == "any" {
                        av.iter().for_each(|(k, v)| {
                            if *v {
                                add_app(k);
                            }
                        });
                        if app != "any" {
                            break;
                        }
                    }
                }
                break;
            }
        }
    }
}
