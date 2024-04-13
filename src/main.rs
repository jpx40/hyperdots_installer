use clap::Parser;
use std::string::String;
use std::vec::Vec;
mod menu;
use menu::Group;
mod aur;
mod cli;
mod conf;
mod installer;
mod utils;

#[derive(Parser, Clone, Debug)]
pub struct Cli {
    #[arg(short, long)]
    pub list: Option<String>,
    #[arg(short, long)]
    pub config: Option<String>,
    // apps: Option<Vec<String>>,
    #[arg(long, default_value_t = true)]
    pub backup: bool,
    #[arg(long, default_value_t = String::from("~/backup/"))]
    pub backup_path: String,
    #[arg(short, long)]
    pub deps_file: Option<String>,
    #[arg(short, long, value_delimiter = ',')]
    pub menu: Option<Vec<String>>,
    #[arg(short, long)]
    pub out: Option<String>,
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
    if !app_list.is_empty() & !dep_list.is_empty() {
        if let Ok(mut v) = installer::APP_LIST.lock() {
            *v = utils::read_app_list(&app_list).unwrap_or_else(|err| panic!("{}", err))
        }
        if let Ok(mut v) = installer::DEPENDENCIES.lock() {
            *v = utils::read_dep_list(&dep_list).unwrap_or_else(|err| panic!("{}", err))
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
    if let Some(extra) = installer::APP_LIST.lock().unwrap().extra.clone() {
        for (k, _v) in extra {
            check_groups.push(k.clone())
        }
    }

    group_names.iter().for_each(|mut n| {
        let mut default: Option<String> = None;
        let mut g: String = n.to_string();
        if g.contains("=") {
            let s: Vec<String> = n.split("=").map(|s| s.to_string()).collect();
            assert_eq!(s.len(), 2);
            // println!("{:?}", s)
            g = s[0].clone();
            default = Some(s[1].clone());
        }
        if check_groups.contains(&g) {
            let mut group = menu::Group::new_with_name(&g);
            installer::APP_LIST
                .lock()
                .unwrap()
                .extra
                .clone()
                .iter()
                .for_each(|a| {
                    for (k, v) in a.clone() {
                        if k == g {
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
                                if let Some(d) = default.clone() {
                                    if app.name == d {
                                        group.add_default(app)
                                    } else {
                                        group.add(app);
                                    }
                                } else {
                                    group.add(app);
                                }
                            });
                        }
                    }
                });
            //   println!("{:?}", group);

            menu.entry(group).unwrap_or_else(|err| panic!("{}", err));
        }
    });
    installer::install_from_config();
    //menu::run(cli, group);
    match installer::install(out) {
        Ok(()) => println!("continue"),
        Err(_) => println!("failed to write installer file"),
    }
}
//struct Commands {}
fn main() {
    println!("\n");
    let mut feature = Feature::new();
    let cli = Cli::parse();
    cli::check_arguments(cli.clone());
    exec_command(cli);

    //println!(insller::app_list()
}

const _CHECK_OS: () = if cfg!(all(
    not(target_os = "linux"),
    not(feature = "unsupported-os")
)) {
    panic!("Sorry, only Linux is currently supported.");
};
