use crate::installer::AppList;
use chrono::Local;
use copy_dir::copy_dir;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

pub fn read_app_list() -> Result<AppList, toml::de::Error> {
    let file = Path::new("app_list.toml");
    let mut pkg_str = String::new();
    File::open(file)
        .unwrap()
        .read_to_string(&mut pkg_str)
        .unwrap();
    let app_list: AppList = toml::from_str(&pkg_str)?;
    Ok(app_list)
}

pub fn backup(backup_path: &str, backup: bool) -> Result<(), io::Error> {
    let time = Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();
    let homedir: String = home::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let backup_path: String = homedir.clone() + backup_path;
    let backup_dir: String = backup_path + "config" + time.as_str();
    let target: String = homedir + "config";
    if backup {
        copy_dir(target, backup_dir)?;
    }
    Ok(())
}
