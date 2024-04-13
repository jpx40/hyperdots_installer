use crate::installer::{AppList, Dependency};
use chrono::Local;
use copy_dir::copy_dir;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Output};

pub fn read_app_list(file_ref: &str) -> Result<AppList, toml::de::Error> {
    let file_str = complete_path(file_ref.to_string());
    let file = Path::new(&file_str);
    let mut pkg_str = String::new();
    File::open(file)
        .unwrap()
        .read_to_string(&mut pkg_str)
        .unwrap();
    let app_list: AppList = toml::from_str(&pkg_str)?;
    Ok(app_list)
}
pub fn read_dep_list(file_ref: &str) -> Result<Dependency, toml::de::Error> {
    let file_str = complete_path(file_ref.to_string());
    let file = Path::new(&file_str);
    let mut pkg_str = String::new();
    File::open(file)
        .unwrap()
        .read_to_string(&mut pkg_str)
        .unwrap();
    let app_list: Dependency = toml::from_str(&pkg_str)?;
    Ok(app_list)
}

pub fn complete_path(p: String) -> String {
    if !p.contains('/') {
        "./".to_string() + &p
    } else {
        p
    }
}

//
// pub fn check() -> PyResult<()> {
//     let key1 = "key1";
//     let val1 = 1;
//     let key2 = "key2";
//     let val2 = 2;
//
//     Python::with_gil(|py| {
//         let fun: Py<PyAny> = PyModule::from_code_bound(
//             py,
//             "def example(*args, **kwargs):
//                 if args != ():
//                     print('called with args', args)
//                 if kwargs != {}:
//                     print('called with kwargs', kwargs)
//                 if args == () and kwargs == {}:
//                     print('called with no arguments')",
//             "",
//             "",
//         )?
//         .getattr("example")?
//         .into();
//
//         // call object with PyDict
//         let kwargs = [(key1, val1)].into_py_dict_bound(py);
//         fun.call_bound(py, (), Some(&kwargs))?;
//
//         // pass arguments as Vec
//         let kwargs = vec![(key1, val1), (key2, val2)];
//         fun.call_bound(py, (), Some(&kwargs.into_py_dict_bound(py)))?;
//
//         // pass arguments as HashMap
//         let mut kwargs = HashMap::<&str, i32>::new();
//         kwargs.insert(key1, 1);
//         fun.call_bound(py, (), Some(&kwargs.into_py_dict_bound(py)))?;
//
//         Ok(())
//     })
// }

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
pub fn is_number(s: &str) -> bool {
    s.chars().all(|c| c.is_numeric())
}

pub fn check_distro(distro: &str) -> bool {
    let mut sh: Command = Command::new("sh");
    let out: Output = Command::new("cat")
        .arg("/etc/os-release")
        .output()
        .expect("Failed to execute command");

    let s: String = String::from_utf8_lossy(&out.stdout).to_string();
    let split: Vec<&str> = s.split("\n").collect();
    let mut os: String = String::new();
    let s: Vec<String> = split
        .iter()
        .map(|x| x.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    s.iter().for_each(|s| {
        if s.contains("ID_LIKE") {
            let split: Vec<&str> = s.split("=").collect();
            os = split[1].to_string();
            os = os.replace(r#"""#, "");
            os = os.replace("\\", "");
        }
    });
    let mut out: Vec<String> = Vec::new();
    if os.contains(" ") {
        let split: Vec<&str> = os.split(" ").collect();
        for i in split {
            out.push(i.to_string().trim().to_string())
        }
    } else {
        out.push(os)
    }
    out = out.iter().map(|s| s.to_lowercase()).collect();
    out.contains(&distro.to_string())
}
