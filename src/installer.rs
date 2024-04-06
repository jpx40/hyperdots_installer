use crate::utils::{self, read_app_list};
use crate::{conf, menu};
use alpm::{self, Group, Version};
use home::home_dir;
use lazy_static::lazy_static;
use rhai::OptimizationLevel;
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

type Extra = HashMap<String, HashMap<String, Option<AppConf>>>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AppConf {
    install: Option<bool>,
    version: Option<String>,
    fullname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Kategorie(Option<HashMap<String, Option<HashMap<String, bool>>>>);
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AppList {
    pub system: Option<HashMap<String, Option<AppConf>>>,
    pub displaymanager: Option<HashMap<String, Option<AppConf>>>,
    // pub extra: Option<HashMap<String, Option<HashMap<String, bool>>>>,
    pub extra: Option<Extra>,
    pub shell: Option<HashMap<String, Option<AppConf>>>,
    pub windowmanager: Option<HashMap<String, Option<AppConf>>>,
    pub dependicies: Option<HashMap<String, Option<AppConf>>>,
    pub theming: Option<HashMap<String, Option<AppConf>>>,
    pub hyperdots: Option<HashMap<String, Option<AppConf>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub dependency: Option<HashMap<String, Option<AppConf>>>,
    pub extra: Option<HashMap<String, HashMap<String, Option<AppConf>>>>,
}

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
// enum Extra {
//     HashMap,
//     String,
// }

lazy_static! {
    static ref APP_LIST: AppList =
        utils::read_app_list(&conf::CONF.app_file).unwrap_or_else(|err| panic!("{}", err));
}
lazy_static! {
    static ref DEPENDENCIES: Dependency =
        utils::read_dep_list(&conf::CONF.dep_file).unwrap_or_else(|err| panic!("{}", err));
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
    let dep_list = DEPENDENCIES.clone();
    check_app_conf(app_list.system);
    check_app_conf(app_list.displaymanager);
    check_app_conf(app_list.theming);
    check_app_conf(app_list.hyperdots);
    check_app_conf(app_list.dependicies);
    check_app_conf(dep_list.dependency.clone());
    add_extra_deps(dep_list);
    install_applications();
}

fn check_app_conf(app: Option<HashMap<String, Option<AppConf>>>) {
    if let Some(app) = app {
        for (k, v) in app.clone() {
            if let Some(conf) = v.clone() {
                if let Some(install) = conf.install {
                    if install {
                        if let Some(fullname) = conf.fullname {
                            add_app(&fullname)
                        } else {
                            add_app(&k);
                        }
                    }
                } else if let Some(fullname) = conf.fullname {
                    add_app(&fullname)
                } else {
                    add_app(&k);
                }
            } else {
                add_app(&k);
            }
        }
    }
}
pub fn add_app_key(app: Option<HashMap<String, Option<AppConf>>>) {
    if let Some(app) = app {
        for (k, v) in app {}
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
    add_extra(app_list, "application")
}
pub fn install_editor(editor: EditorList) {
    let app_list = APP_LIST.clone();
}
//     match editor {
//         EditorList::Intellij => add_app("intellij-idea-community-edition"),
//         EditorList::VsCode => add_app("visual-studio-code-bin"),
//         EditorList::Code => add_app("code"),
//         EditorList::Neovim => {
//             add_extra_deps(app_list, "neovim", "editor");
//         }
//         EditorList::Any => {
//             add_extra_deps(app_list, "any", "editor");
//         }
//     }
// }

fn extra_menu(app_list: AppList, g: String) -> menu::Group {
    let mut group = menu::Group::new_with_name(&g);

    if let Some(apps) = app_list.extra {
        for (k, v) in apps.clone() {
            if k == g {
                v.iter().for_each(|(k, v)| {
                    if let Some(conf) = v.clone() {
                        if let Some(install) = conf.install {
                            let mut app = menu::App::new(k);
                            if install {
                                if let Some(fullname) = conf.fullname {
                                    app.add_fullname(fullname)
                                }
                                if let Some(version) = conf.version {
                                    app.add_version(version)
                                }
                                group.add(app)
                            }
                        } else {
                            let mut app = menu::App::new(k);
                            if let Some(fullname) = conf.fullname {
                                app.add_fullname(fullname)
                            }
                            if let Some(version) = conf.version {
                                app.add_version(version)
                            }
                            group.add(app)
                        }
                    } else {
                        add_app(k);
                        let app = menu::App::new(k);
                        group.add(app);
                    }
                });
            }
        }
    }
    group
}

fn add_extra(app_list: AppList, app: &str) {
    if let Some(apps) = app_list.extra {
        for (k, v) in apps.clone() {
            if k == app {
                v.iter().for_each(|(k, v)| {
                    if let Some(conf) = v.clone() {
                        if let Some(install) = conf.install {
                            if install {
                                if let Some(fullname) = conf.fullname {
                                    add_app(&fullname)
                                } else {
                                    add_app(k);
                                }
                            }
                        } else if let Some(fullname) = conf.fullname {
                            add_app(&fullname)
                        } else {
                            add_app(k);
                        }
                    } else {
                        add_app(k);
                    }
                });
            }
        }
    }
}

fn add_extra_deps(dep_list: Dependency) {
    if let Some(apps) = dep_list.extra {
        for (k, v) in apps.clone() {
            if app_array().lock().unwrap().contains(&k) {
                v.iter().for_each(|(k, v)| {
                    if let Some(conf) = v.clone() {
                        if let Some(install) = conf.install {
                            if install {
                                if let Some(fullname) = conf.fullname {
                                    add_app(&fullname)
                                } else {
                                    add_app(&k);
                                }
                            }
                        } else if let Some(fullname) = conf.fullname {
                            add_app(&fullname)
                        } else {
                            add_app(&k);
                        }
                    } else {
                        add_app(&k);
                    }
                });
            }
        }
    }
}
