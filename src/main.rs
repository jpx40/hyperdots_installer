use chrono::prelude::*;
use clap::{Args, Parser, Subcommand, ValueEnum};
use copy_dir::copy_dir;
use menu::App;
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
use std::process::Command;
use std::string::String;
use std::sync::{Mutex, OnceLock};
use std::time;
use std::time::Duration;
use std::vec::Vec;
use toml::{toml, Table, Value};
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

//struct Commands {}
fn main() {
    let mut feature = Feature::new();
    let cli = Cli::parse();
    cli::check_arguments(cli.clone());
    let mut group = menu::Group::new();
    group.add_name("editor");
    group.add_app("neovim");
    group.add_app("vscode");
    group.add_app("intellij");
    group.add_app("pycharm");
    group.add_app("atom");
    group.add_default(menu::App::new("neovim"));
    let mut menu = menu::Menu::new();
    menu.entry(group).unwrap_or_else(|err| panic!("{}", err));

    // menu::run(cli, group);
    match installer::install() {
        Ok(()) => println!("continue"),
        Err(_) => println!("failed to write installer file"),
    }
    //println!(insller::app_list()
}

const _CHECK_OS: () = if cfg!(all(
    not(target_os = "linux"),
    not(feature = "unsupported-os")
)) {
    panic!("Sorry, only Linux is currently supported.");
};
