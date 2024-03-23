use chrono::prelude::*;
use clap::{Args, Parser, Subcommand, ValueEnum};
use copy_dir::copy_dir;
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

//struct Commands {}
fn main() {
    let cli = Cli::parse();
    cli::check_arguments(cli.clone());
    menu::menu(cli).unwrap();
}
