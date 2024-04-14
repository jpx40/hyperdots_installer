use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::{Arc, RwLock};
use std::sync::{Mutex, OnceLock};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub config_file: Option<String>,
    pub config_dir: Option<String>,
    pub app_file: Option<String>,
    pub dep_file: Option<String>,
}

// struct App {
//     name: String,
//     path: String,
//     group: String,
// }
// struct Config {
//     apps: Vec<App>,
// }

// lazy_static! {
//     pub static ref CONF: Mutex<Config> = Mutex::new(Config::new("", "", "", ""));
// }

static GLOBAL_CONF: Mutex<Option<Config>> = Mutex::new(None);

lazy_static! {
    pub static ref CURRENT_CONFIG: RwLock<Arc<Config>> = RwLock::new(Default::default());
}

impl Config {
    // pub fn new_from_env() -> Config {
    //     Config {
    //         config_file: env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| config::get_config_dir()),
    //         config_dir: env::var("XDG_DATA_HOME").unwrap_or_else(|_| config::get_data_dir()),
    //     }
    //         pub fn get_config_dir() -> String {
    //             config::get_config_dir()
    //         }
    // pub fn new(file: &str, path: &str, app_file: &str, dep_file: &str) -> Config {
    //     Config {
    //         config_file: file.to_string(),
    //         config_dir: path.to_string(),
    //         app_file: app_file.to_string(),
    //         dep_file: dep_file.to_string(),
    //     }
    // }
}
