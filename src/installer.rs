use crate::utils::read_app_list;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

pub fn install_all() {}
pub fn check_arguments_from_config() {}

pub fn install_from_config() {
    let _ = read_app_list();
}
