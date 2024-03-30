use crate::conf;
use crate::installer;
use crate::installer::EditorList;
use crate::utils;
use crate::Cli;
use crate::Feature;
use futures::future;
use reedline;
use reedline::{DefaultPrompt, Reedline, Signal};
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
    pub fn entry(&mut self, group: Group) -> reedline::Result<()> {
        let mut group = group;
        let mut text: String = String::new();
        //  let mut lines: Vec<Line> = Vec::new();
        let mut default: App = App::new("Neovim");
        let mut default_str: String = String::new();

        match group.default.clone() {
            Some(d) => {
                default = d;
                default_str = "Default: 1.".to_string();
                if default.position != 1 {
                    default.to_owned().set_position(1);
                    default.to_owned().set_position_str("1".to_string());
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
            match into_string(count) {
                Ok(s) => v.to_owned().set_position_str(s),
                Err(e) => {
                    panic!("{:?}", e)
                }
            }

            text.push_str(&format!("{count}. {k} "));
        }
        if let Some(name) = group.name {
            println!("{name}");
        }
        println!("1. {} {text}", group.default.unwrap().name);
        println!("{default_str}");

        let mut line_editor = Reedline::create();
        let prompt = DefaultPrompt::default();

        'outer: loop {
            //let mut rl = self.editor.readline(">> "); // read
            //   let mut readline = ::new()?.readline(">> "); // read
            let sig = line_editor.read_line(&prompt);
            println!("Line: {sig:?}"); // eval / print
            match sig {
                Ok(Signal::Success(line)) => {
                    // let mut count: i32 = 2;
                    let line: String = line.trim().to_string();
                    println!("{line}");
                    if is_number(&line) {
                        let len = group.bin.len() + 1;
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
                        } else {
                            println!("{line}");
                            if line == default.position.to_string() || line == default.position_str
                            {
                                match default.fullname.clone() {
                                    Some(n) => {
                                        installer::add_app(&n);
                                    }
                                    None => {
                                        installer::add_app(&default.name);
                                    }
                                }
                                break 'outer;
                            } else {
                                for (_k, v) in group.bin.iter() {
                                    // if v.position == 0 || v.position == 1 {
                                    //     panic!("invalid position, of app {}", v.name)
                                    // }
                                    let test: String = format!("{}", v.position);
                                    if format!("{line}") == v.position.to_string() {
                                        match v.fullname.clone() {
                                            Some(n) => {
                                                installer::add_app(&n);
                                            }
                                            None => {
                                                installer::add_app(&default.name);
                                            }
                                        }
                                        break 'outer;
                                    }
                                    count += 1;
                                }
                            }
                        }
                    } else if line.is_empty() {
                        match default.fullname.clone() {
                            Some(n) => installer::add_app(&n),
                            None => installer::add_app(&default.name),
                        }
                    } else {
                        println!("Invalid input2");
                        continue;
                    }
                }
                //  for (k, v) in group.bin.iter() {},
                Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
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

fn into_string(n: u32) -> Result<String, String> {
    let mut s = String::new();
    match n {
        1 => s.push_str("1"),
        2 => s.push_str("2"),
        3 => s.push_str("3"),
        4 => s.push_str("4"),
        5 => s.push_str("5"),
        6 => s.push_str("6"),
        7 => s.push_str("7"),
        8 => s.push_str("8"),
        9 => s.push_str("9"),
        10 => s.push_str("10"),
        11 => s.push_str("11"),
        12 => s.push_str("12"),
        13 => s.push_str("13"),
        14 => s.push_str("14"),
        15 => s.push_str("15"),
        16 => s.push_str("16"),
        17 => s.push_str("17"),
        18 => s.push_str("18"),
        19 => s.push_str("19"),
        20 => s.push_str("20"),
        21 => s.push_str("21"),
        22 => s.push_str("22"),
        23 => s.push_str("23"),
        24 => s.push_str("24"),
        25 => s.push_str("25"),
        26 => s.push_str("26"),
        27 => s.push_str("27"),
        28 => s.push_str("28"),
        29 => s.push_str("29"),
        30 => s.push_str("30"),
        _ => return Err("invalid number".to_string()),
    }
    Ok(s)
}

fn is_number(s: &str) -> bool {
    s.chars().all(|c| c.is_numeric())
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
    pub fn new_with_name(name: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            bin: HashMap::new(),
            description: None,
            default: None,
            category: None,
        }
    }
    pub fn new_with_name_and_category(name: &str, category: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            bin: HashMap::new(),
            description: None,
            default: None,
            category: Some(category.to_string()),
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

pub fn menu(c: Cli, f: Feature) -> rustyline::Result<()> {
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
