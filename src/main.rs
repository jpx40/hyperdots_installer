use chrono::prelude::*;
use clap::{Args, Parser, Subcommand, ValueEnum};
use conf::Config;
use copy_dir::copy_dir;
use core::panic;

use installer::AppConf;

use log::log;
use menu::App;
use serde::de::value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::string::String;
use std::sync::{Mutex, OnceLock};
use std::time;
use std::time::Duration;
use std::vec::Vec;
use toml::{toml, Table};
use walkdir::WalkDir;
mod menu;
mod utils;
use menu::*;
mod cli;
mod conf;
mod installer;
#[derive(Parser, Clone, Debug)]
pub struct Cli {
    #[arg(short, long)]
    pub list: Option<String>,
    #[arg(short, long)]
    pub config: Option<String>,
    // apps: Option<Vec<String>>,
    pub groups: Option<Group>,
    #[arg(long, default_value_t = true)]
    pub backup: bool,
    #[arg(long, default_value_t = String::from("~/backup/"))]
    pub backup_path: String,
    #[arg(short, long)]
    pub deps_file: Option<String>,
    #[arg(short, long, value_delimiter = ',')]
    pub menu: Option<Vec<String>>,
    pub out: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Group {
    Config,
    All,
}

struct Feature {
    backup: bool,
}
impl Feature {
    fn new() -> Self {
        Feature { backup: false }
    }
    fn set_backup(&mut self, backup: bool) {
        self.backup = backup;
    }
}
pub fn exec_command(c: Cli) {
    let mut app_list = String::new();
    let mut dep_list = String::new();
    if let Some(list) = c.list.clone() {
        if list.contains(".toml") {
            app_list = list;
        } else {
            panic!("Wrong file format");
        }
    }
    if let Some(dep) = c.deps_file.clone() {
        if dep.contains(".toml") {
            dep_list = dep;
        } else {
            panic!("Wrong file format");
        }
    }
    if !app_list.is_empty() || !dep_list.is_empty() {
        if let Ok(mut v) = conf::CONF.lock() {
            v.app_file = app_list;
            v.dep_file = dep_list;
        }
    } else {
        panic!("No Arguments");
    }
    let mut group_names: Vec<String> = Vec::new();
    if let Some(apps) = c.menu.clone() {
        group_names = apps;
    }
    let out: String;
    if let Some(out_s) = c.out.clone() {
        out = out_s;
    } else {
        panic!("no output specified");
    }
    let mut menu = menu::Menu::new();
    let mut check_groups: Vec<String> = Vec::new();
    if let Some(extra) = installer::APP_LIST.extra.clone() {
        for (k, _v) in extra {
            check_groups.push(k.clone())
        }
    }

    group_names.iter().for_each(|n| {
        if check_groups.contains(n) {
            let mut group = menu::Group::new_with_name(n);
            installer::APP_LIST.extra.clone().iter().for_each(|a| {
                for (k, v) in a {
                    if k == n {
                        v.clone().iter().for_each(|(k, v)| {
                            let mut app = menu::App::new(&k.clone().to_string());
                            if let Some(conf) = v.clone() {
                                if let Some(fullname) = conf.fullname {
                                    app.add_fullname(fullname);
                                }
                                if let Some(version) = conf.version {
                                    app.add_version(version);
                                }
                            }
                            group.add(app);
                        });
                    }
                }
            });

            menu.entry(group).unwrap_or_else(|err| panic!("{}", err));
        }
    });

    cli::check_arguments(c.clone());
    //menu::run(cli, group);
    match installer::install(out) {
        Ok(()) => println!("continue"),
        Err(_) => println!("failed to write installer file"),
    }
}
//struct Commands {}
fn main() {
    println!("\n");
    let config = conf::Config::new("", "", "app_list.toml", "deps.toml");
    let mut feature = Feature::new();
    let cli = Cli::parse();
    exec_command(cli);

    //println!(insller::app_list()
}

const _CHECK_OS: () = if cfg!(all(
    not(target_os = "linux"),
    not(feature = "unsupported-os")
)) {
    panic!("Sorry, only Linux is currently supported.");
};
