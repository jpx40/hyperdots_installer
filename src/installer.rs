use crate::aur::PkgDB;
use crate::{conf, menu, utils};
use core::panic;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex, OnceLock};
use std::{env, fs::File, path::PathBuf};

type Extra = HashMap<String, HashMap<String, Option<AppConf>>>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AppConf {
    pub install: Option<bool>,
    pub version: Option<String>,
    pub fullname: Option<String>,
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
impl AppList {
    pub fn new() -> AppList {
        AppList {
            extra: None,
            system: None,
            theming: None,
            windowmanager: None,
            dependicies: None,
            hyperdots: None,
            shell: None,
            displaymanager: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub dependency: Option<HashMap<String, Option<AppConf>>>,
    pub extra: Option<HashMap<String, HashMap<String, Option<AppConf>>>>,
}
impl Dependency {
    pub fn new() -> Dependency {
        Dependency {
            dependency: None,
            extra: None,
        }
    }
}

lazy_static! {
    pub static ref APP_LIST: Mutex<AppList> = Mutex::new(AppList::new());
}
lazy_static! {
    pub static ref DEPENDENCIES: Mutex<Dependency> = Mutex::new(Dependency::new());
}

pub fn app_list() -> &'static Mutex<AppList> {
    static ARRAY: OnceLock<Mutex<AppList>> = OnceLock::new();
    ARRAY.get_or_init(|| Mutex::new(AppList::new()))
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
    let line: String = line.to_string() + "\n";
    params_file().lock().unwrap().push_str(&line);
}

pub fn params_file() -> &'static Mutex<String> {
    static PARAMS: OnceLock<Mutex<String>> = OnceLock::new();
    PARAMS.get_or_init(|| Mutex::new(String::new()))
}

pub fn install_all() {}

pub fn install_from_config() {
    let app_list: AppList = APP_LIST.lock().unwrap().clone();
    let dep_list: Dependency = DEPENDENCIES.lock().unwrap().clone();
    check_app_conf(app_list.system);
    check_app_conf(app_list.displaymanager);
    check_app_conf(app_list.theming);
    check_app_conf(app_list.hyperdots);
    check_app_conf(app_list.dependicies);
    check_app_conf(app_list.windowmanager);
    check_app_conf(app_list.shell);
    check_app_conf(dep_list.dependency.clone());
    add_extra_deps(dep_list);
    //  install_applications();
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
pub fn install(out: String) -> Result<(), String> {
    let path: PathBuf = env::current_dir().unwrap();

    // if !Path::new("~/.cache/hyprdots").exists() {
    //     create_dir_all(Path::new("~/.cache/hyprdots")).unwrap_or_else(|err| panic!("{}", err));
    // }
    // let cache_path = Path::new("~/.cache/hyprdots/");
    let out: String = utils::complete_path(out);
    let out_path = camino::Utf8Path::new(&out)
        .parent()
        .unwrap()
        .canonicalize_utf8()
        .unwrap_or_else(|err| panic!("{}", err));
    //
    let mut file: File = File::create(out_path.join(out.clone())).unwrap();
    let mut app_file: File = File::create(path.join(out)).unwrap();
    file.write_all(params_file().lock().unwrap().as_bytes())
        .unwrap_or_else(|err| panic!("{}", err));
    let apps: Vec<String> = app_array().lock().unwrap().clone();
    let mut apps_tmp: Vec<String> = Vec::new();
    //has to be changed later
    if utils::check_distro("arch") {
        let mut count: u32 = 0;
        let mut pkg_db: PkgDB = PkgDB::init().unwrap_or_else(|err| panic!("{}", err));
        apps.iter().for_each(|a| {
            if !pkg_db.is_installed(a.clone()) {
                apps_tmp.push(a.clone());
            }
            count += 1;
        });
    } else {
        apps_tmp = apps;
    }
    for app in apps_tmp {
        println!("Installing: {:?}", app);
        let buf: String = app.clone() + "\n";
        app_file.write_all(buf.as_bytes()).unwrap();
    }
    Ok(())
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
    let app_list = APP_LIST.lock().unwrap().clone();
    add_extra(app_list, "application")
}

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
