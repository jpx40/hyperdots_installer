use crate::conf;
use crate::installer;
use crate::installer::EditorList;
use crate::utils;
use crate::Cli;
use crate::Feature;
use futures::future;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Editor, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::default;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Debug)]
pub struct Menu {
    pub groups: HashMap<String, Group>,
    pub config: Option<conf::Config>,
    pub editor: DefaultEditor,
    pub menu_entry: Option<MenuEntry>,
}

#[derive(Debug)]
pub struct Config {
    pub config: conf::Config,
    pub editor: DefaultEditor,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuEntry {
    pub groups: Vec<Group>,
}

impl MenuEntry {
    fn new() -> Self {
        Self { groups: vec![] }
    }
    fn add(&mut self, group: Group) {
        self.groups.push(group);
    }
}
impl Menu {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            config: None,
            editor: DefaultEditor::new().unwrap_or_else(|err| panic!("{:?}", err)),
            menu_entry: None,
        }
    }
    pub fn new_with_config(config: conf::Config) -> Self {
        Self {
            groups: HashMap::new(),
            config: Some(config),
            editor: DefaultEditor::new().unwrap_or_else(|err| panic!("{:?}", err)),
            menu_entry: None,
        }
    }
    fn add_menu_entry(&mut self, menu_entry: MenuEntry) {
        self.menu_entry = Some(menu_entry);
    }
    pub fn add_config(&mut self, config: conf::Config) {
        self.config = Some(config);
    }

    pub fn add(&mut self, group: Group) {
        if let Some(menu_entry) = &mut self.menu_entry {
            menu_entry.add(group);
            self.menu_entry = Some(menu_entry.clone());
        } else {
            let menu_entry = Some(MenuEntry::new());
            menu_entry.clone().unwrap().add(group);
            self.menu_entry = menu_entry;
        }
    }
    pub fn add_group(&mut self, group: Group) {
        self.groups
            .insert(group.name.clone().unwrap_or_default(), group);
    }
    pub fn remove_group(&mut self, group: &str) {
        self.groups.remove(group);
    }

    pub fn new_group(&mut self, name: String) {
        self.new_group_with_name(name)
    }
    pub fn new_group_with_name(&mut self, name: String) {
        self.groups.insert(name.clone(), Group::new_with_name(name));
    }
    pub fn editor_config(&mut self) {}
    pub fn entry(&mut self, group: Group) {
        let mut group = group;
        let mut text: String = String::new();
        let mut lines: Vec<Line> = Vec::new();
        let mut count: i32 = 0;
        let mut default: App = App::new("Neovim".to_string());
        let mut default_str: String = String::new();

        match group.default.clone() {
            Some(d) => {
                default = d;
                default_str = format!("Default: 1.");
                if group.bin.contains_key(&default.name) {
                    group.remove_app(&default.name);
                }
            }
            None => {
                let mut count = 0;
                for (k, v) in group.bin.iter() {
                    count += 1;
                    default = v.clone();
                    default_str = format!("Default: 1.");

                    group.to_owned().bin.remove(&default.name);
                    if count == 1 {
                        break;
                    }
                }
            }
        }

        for (k, v) in group.bin.iter() {
            count += 1;
            // let line = format!("{count}. {k} ");
            //lines.push(Line::new(text, v.clone()));
            text.push_str(&format!("{count}. {k} "));
        }
        if let Some(name) = group.name {
            println!("{name}");
        }
        println!("{text}");
        println!("{default_str}");
        loop {
            let readline = &self.editor.readline(">> "); // read
            match readline {
                Ok(line) => {
                    let mut count: i32 = 2;

                    if line == "1" {
                        match default.fullname.clone() {
                            Some(n) => installer::add_app(&n),
                            None => installer::add_app(&default.name),
                        }
                    } else {
                        for (_k, v) in group.bin.iter() {
                            if line == &count.to_string() {
                                match v.fullname.clone() {
                                    Some(n) => installer::add_app(&n),
                                    None => installer::add_app(&default.name),
                                }
                                count += 1;
                            }
                        }
                    }
                }
                //  for (k, v) in group.bin.iter() {},
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    process::exit(0);
                }

                Err(err) => {
                    println!("Error: {:?}", err);
                    //         process::exit(0);
                }
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    text: String,
    app: App,
}
impl Line {
    pub fn new(text: String, app: App) -> Self {
        Self { text, app }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub name: Option<String>,
    pub bin: HashMap<String, App>,
    pub description: Option<String>,
    pub default: Option<App>,
    pub category: Option<String>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            name: None,
            bin: HashMap::new(),
            description: None,
            default: None,
            category: None,
        }
    }
    pub fn new_with_name(name: String) -> Self {
        Self {
            name: Some(name),
            bin: HashMap::new(),
            description: None,
            default: None,
            category: None,
        }
    }
    pub fn new_with_name_and_category(name: String, category: String) -> Self {
        Self {
            name: Some(name),
            bin: HashMap::new(),
            description: None,
            default: None,
            category: Some(category),
        }
    }
    pub fn new_with_category(category: String) -> Self {
        Self {
            name: None,
            bin: HashMap::new(),
            description: None,
            default: None,
            category: Some(category),
        }
    }

    pub fn add(&mut self, app: App) {
        self.bin.insert(app.name.clone(), app);
    }
    pub fn add_default(&mut self, app: App) {
        self.default = Some(app);
        if let Some(app) = &self.default {
            if self.bin.contains_key(&app.name) {
                self.bin.remove(&app.name);
            }
        }
    }
    pub fn add_app(&mut self, name: String) {
        self.bin.insert(name.clone(), App::new(name));
    }
    pub fn remove_app(&mut self, name: &str) {
        self.bin.remove(name);
    }
    pub fn add_description(&mut self, description: String) {
        self.description = Some(description);
    }
    pub fn add_name(&mut self, group: String) {
        self.name = Some(group);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub fullname: Option<String>,
}

impl App {
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: None,
            description: None,
            fullname: None,
        }
    }
    pub fn add_version(&mut self, version: String) {
        self.version = Some(version);
    }
    pub fn add_fullname(&mut self, fullname: String) {
        self.fullname = Some(fullname);
    }

    pub fn add_description(&mut self, description: String) {
        self.description = Some(description);
    }
}

pub fn menu(c: Cli, f: Feature) -> Result<()> {
    env_logger::init();
    let mut rl = DefaultEditor::new()?;

    // let line = rl.readline(">> ")?; // read
    // println!("Line: {line}"); // eval / print
    //
    //
    //
    println!("\n\nWelcome to the Installer");
    println!("\n");
    if f.backup {
        loop {
            println!("Backup | y/n | Default: y");

            let readline = rl.readline(">> "); // read
                                               // eval / printy

            match readline {
                Ok(line) => match line.to_lowercase().as_str() {
                    "y" => {
                        println!("Backup of Config");

                        utils::backup(&c.backup_path, c.backup).unwrap();
                        break;
                    }
                    "n" => {
                        println!("No Backup");
                        break;
                    }

                    "" => {
                        println!("Backup of Config");
                        let backup = false;
                        utils::backup(&c.backup_path, backup).unwrap();
                        break;
                    }
                    _ => {
                        println!("Invalid Input")
                    }
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    process::exit(0);
                }

                Err(err) => {
                    println!("Error: {:?}", err);
                    //           process::exit(0);
                }
            }
        } // loop
    }
    loop {
        println!("Choose Software");
        println!("1. From Config, 2. All | Default: 1");
        let readline = rl.readline(">> "); // read
        match readline {
            Ok(line) => match line.as_str() {
                "1" => {
                    println!("Installing from Config");
                    installer::install_from_config();
                    break;
                }
                "2" => {
                    println!("Installing All");
                    installer::install_all();
                    break;
                }

                "" => {
                    println!("Installing from Config");
                    installer::install_from_config();
                    break;
                }
                _ => {
                    println!("Invalid Input")
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                process::exit(0);
            }

            Err(err) => {
                println!("Error: {:?}", err);
                //         process::exit(0);
            }
        }
    }
    loop {
        println!("Choose Software");
        println!("1. Code (VScode Open Source Edition), 2. Intellij, 3. VScode, 4. Neovim  5. All | Default: 1");
        let readline = rl.readline(">> "); // read
                                           // eval / printy

        match readline {
            Ok(line) => match line.as_str() {
                "1" => {
                    println!("Installing Code");
                    installer::install_editor(EditorList::Code);
                    break;
                }
                "2" => {
                    println!("Installing Intellij");
                    installer::install_editor(EditorList::Intellij);
                    break;
                }

                "3" => {
                    println!("Installing VScode");
                    installer::install_editor(EditorList::VsCode);
                    break;
                }
                "4" => {
                    println!("Installing Neovim");
                    installer::install_editor(EditorList::Neovim);
                    break;
                }
                "5" => {
                    println!("Installing All");
                    installer::install_editor(EditorList::Any);
                    break;
                }

                _ => {
                    println!("Invalid Input")
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                process::exit(0);
            }

            Err(err) => {
                println!("Error: {:?}", err);
                //  process::exit(0);
            }
        }
    } // loop
    Ok(())
}
