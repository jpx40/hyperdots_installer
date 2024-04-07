use crate::installer::*;
use crate::utils::read_app_list;
use crate::{Cli, Group};
use clap::{Args, Parser, Subcommand, ValueEnum};
#[derive(Debug, Clone, ValueEnum)]
enum App {
    Config,
    All,
    System,
    Displaymanager,
    Extra,
    Shell,
    Windowmanager,
    Dependicies,
    Theming,
    Hyperdots,
}

fn cli() {}

pub fn check_arguments(c: Cli) {
    // match &c.groups {
    //     Some(Group::All) => install_all(),
    //     Some(Group::Config) => install_from_config(),
    //     None => install_from_config(),
    //     _ => (),
    // }
    if c.out.is_none() {
        println!("No output");
        std::process::exit(0);
    }
    if c.list.is_none() {
        println!("No app list file");
        std::process::exit(0);
    }
    if c.menu.is_none() {
        println!("No menu args");
        std::process::exit(0);
    }
    if c.deps_file.is_none() {
        println!("No menu deps_file");
        std::process::exit(0);
    }
}
