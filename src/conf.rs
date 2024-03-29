use config;
use serde::{Deserialize, Serialize};
use std::env;
pub fn check_arguments_from_config() {}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Config {
    pub config_file: String,
    pub config_dir: String,
}

// struct App {
//     name: String,
//     path: String,
//     group: String,
// }
// struct Config {
//     apps: Vec<App>,
// }

impl Config {
    // pub fn new_from_env() -> Config {
    //     Config {
    //         config_file: env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| config::get_config_dir()),
    //         config_dir: env::var("XDG_DATA_HOME").unwrap_or_else(|_| config::get_data_dir()),
    //     }
    //         pub fn get_config_dir() -> String {
    //             config::get_config_dir()
    //         }
    pub fn new(file: &str, path: &str) -> Config {
        Config {
            config_file: file.to_string(),
            config_dir: path.to_string(),
        }
    }
}
