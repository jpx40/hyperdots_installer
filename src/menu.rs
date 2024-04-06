use crate::conf;
use crate::installer;
use crate::installer::EditorList;
use crate::utils;
use crate::Cli;
use crate::Feature;
use futures::future;
use itertools::Position;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Editor};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::default;
use std::env;
use std::fs;
use std::io::Error;
use std::panic;
use std::path::{Path, PathBuf};
use std::process;
use std::string::String;
use std::u32;
use std::usize;

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

    pub fn new_group(&mut self, name: &str) {
        self.new_group_with_name(name)
    }
    pub fn new_group_with_name(&mut self, name: &str) {
        self.groups
            .insert(name.to_string().clone(), Group::new_with_name(name));
    }
    pub fn editor_config(&mut self) {}
    pub fn entry(&mut self, group: Group) -> rustyline::Result<()> {
        let mut group = group;
        let mut text: String = String::new();
        //  let mut lines: Vec<Line> = Vec::new();
        let mut default: App = App::new("Neovim");
        let mut default_str: String = String::new();
        let mut default_position: u32 = 1;
        let mut default_position_str: String = String::new();
        let mut position_str: HashMap<String, String> = HashMap::new();
        match group.default.clone() {
            Some(d) => {
                default = d;
                default_str = "Default: 1.".to_string();
                if default.position != 1 {
                    default.to_owned().set_position(1);
                    default.to_owned().set_position_str("1".to_string());
                    default_position_str = "1".to_string();
                }
                if group.bin.contains_key(&default.name) {
                    group.remove_app(&default.name);
                }
            }
            None => {
                let mut count = 0;
                for (_k, v) in group.bin.iter() {
                    default = v.clone();

                    v.to_owned().set_position_str("1".to_string());
                    default_position_str = "1".to_string();
                    default_str = "Default: 1.".to_string();
                    count += 1;
                    group.to_owned().bin.remove(&default.name);
                    if count == 1 {
                        break;
                    }
                }
            }
        }

        let mut count: u32 = 1;

        for (k, v) in group.bin.iter() {
            count += 1;
            // let line = format!("{count}. {k} ");
            //lines.push(Line::new(text, v.clone()));
            v.to_owned().set_position(count);
            match utils::into_string(count) {
                Ok(s) => {
                    // v.to_owned().set_position_str(s);
                    position_str.insert(k.to_string(), s);
                }
                Err(e) => {
                    panic!("{:?}", e)
                }
            }

            text.push_str(&format!("{count}. {k}, "));
        }
        if let Some(name) = group.name {
            println!("{}", name.to_uppercase());
        }
        let mut s = &text[0..text.len() - 2];
        //  s.remove(-2);
        println!("1. {}, {s} | {default_str}", group.default.unwrap().name);

        'outer: loop {
            //let mut rl = self.editor.readline(">> "); // read
            //   let mut readline = ::new()?.readline(">> "); // read
            let editor_prompt = ">>";
            let line = self.editor.readline(editor_prompt);

            match line {
                Ok(line) => {
                    // let mut count: i32 = 2;
                    if line.is_empty() {
                        match default.fullname.clone() {
                            Some(n) => {
                                installer::add_app(&n);
                            }
                            None => {
                                installer::add_app(&default.name);
                            }
                        }
                        break 'outer;
                    }
                    let mut result: Vec<String> = Vec::new();
                    if line.contains(",") {
                        let line = line.trim();
                        result = line.split(",").map(|s| s.trim().to_string()).collect();
                    }
                    if line.contains(" ") {
                        result = line.split(" ").map(|s| s.trim().to_string()).collect();
                    } else {
                        result.push(line.clone());
                    }
                    let mut check: u16 = 0;
                    result = result
                        .iter()
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect();
                    for line in result.clone() {
                        if !utils::is_number(&line) {
                            let mut debug = String::new();
                            if &line == "1" {
                                debug = default.name.clone();
                            } else {
                                for (_k, v) in group.bin.clone().iter() {
                                    // if v.position == 0 || v.position == 1 {
                                    //     panic!("invalid position, of app {}", v.name)
                                    // }
                                    let test: String = format!("{}", v.position);
                                    if &line == position_str.get(&v.name).unwrap() {
                                        match v.fullname.clone() {
                                            Some(n) => {
                                                println!("add app {}", n);
                                                debug = n;
                                            }
                                            None => {}
                                        }
                                    }
                                }
                            }

                            //println!("invalid Input {}", debug);
                            //process::exit(0);
                            //     break 'outer;
                        }
                    }

                    for line in result {
                        if utils::is_number(&line) {
                            let check = line
                                .parse::<usize>()
                                .unwrap_or_else(|err| panic!("{:?}", err));

                            if check >= group.bin.len() {
                                println!("invalid Input")
                            }
                            if line == default.position.to_string() || line == "1" {
                                match default.fullname.clone() {
                                    Some(n) => {
                                        installer::add_app(&n);
                                    }
                                    None => {
                                        installer::add_app(&default.name);
                                    }
                                }
                            } else {
                                for (_k, v) in group.bin.clone().iter() {
                                    // if v.position == 0 || v.position == 1 {
                                    //     panic!("invalid position, of app {}", v.name)
                                    // }
                                    let test: String = format!("{}", v.position);
                                    if &line == position_str.get(&v.name).unwrap() {
                                        match v.fullname.clone() {
                                            Some(n) => {
                                                println!("add app {}", n);
                                                installer::add_app(&n);
                                            }
                                            None => {
                                                installer::add_app(&v.name);
                                            }
                                        }
                                    }
                                    count += 1;
                                }
                            }
                        }
                    }
                    break 'outer;
                }
                //  for (k, v) in group.bin.iter() {},
                Err(ReadlineError::Interrupted) => {
                    println!("\nAborted!");
                    process::exit(0);
                }
                Err(ReadlineError::Eof) => {
                    println!("\nAborted!");
                    process::exit(0);
                }

                Err(err) => {
                    println!("Error: {:?}", err);
                    //         process::exit(0);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    text: String,
    app: App,
    position: i32,
}
impl Line {
    pub fn new(text: String, app: App, position: i32) -> Self {
        Self {
            text,
            app,
            position,
        }
    }
}
pub fn run(c: Cli, g: Group) {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub name: Option<String>,
    pub bin: HashMap<String, App>,
    pub description: Option<String>,
    pub default: Option<App>,
    pub category: Option<String>,
    pub installer: Option<String>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            name: None,
            bin: HashMap::new(),
            description: None,
            default: None,
            category: None,
            installer: None,
        }
    }
    pub fn new_with_name(name: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            bin: HashMap::new(),
            description: None,
            default: None,
            category: None,
            installer: None,
        }
    }
    pub fn new_with_name_and_category(name: &str, category: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            bin: HashMap::new(),
            description: None,
            default: None,
            category: Some(category.to_string()),
            installer: None,
        }
    }
    pub fn new_with_category(category: String) -> Self {
        Self {
            name: None,
            bin: HashMap::new(),
            description: None,
            default: None,
            category: Some(category),
            installer: None,
        }
    }
    pub fn set_installer(&mut self, installer: String) {
        self.installer = Some(installer);
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
        self.default.clone().unwrap().set_position(1);
    }
    pub fn add_app(&mut self, name: &str) {
        self.bin.insert(name.to_string().clone(), App::new(name));
    }
    pub fn remove_app(&mut self, name: &str) {
        self.bin.remove(name);
    }
    pub fn add_description(&mut self, description: String) {
        self.description = Some(description);
    }
    pub fn add_name(&mut self, group: &str) {
        self.name = Some(group.to_string());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub fullname: Option<String>,
    position: u32,
    position_str: String,
}

impl App {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: None,
            description: None,
            fullname: None,
            position: 0,
            position_str: String::new(),
        }
    }
    pub fn add_version(&mut self, version: String) {
        self.version = Some(version);
    }
    pub fn set_position_str(&mut self, position_str: String) {
        self.position_str = position_str;
    }
    pub fn add_fullname(&mut self, fullname: String) {
        self.fullname = Some(fullname);
    }
    pub fn set_position(&mut self, position: u32) {
        self.position = position;
    }

    pub fn add_description(&mut self, description: String) {
        self.description = Some(description);
    }
}
