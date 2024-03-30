use crate::installer::{AppList, Dependency};
use chrono::Local;
use copy_dir::copy_dir;
use rhai::{Engine, EvalAltResult, Scope};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

pub fn read_app_list(file: &str) -> Result<AppList, toml::de::Error> {
    let file = Path::new(file);
    let mut pkg_str = String::new();
    File::open(file)
        .unwrap()
        .read_to_string(&mut pkg_str)
        .unwrap();
    let app_list: AppList = toml::from_str(&pkg_str)?;
    Ok(app_list)
}
pub fn read_dep_list(file: &str) -> Result<Dependency, toml::de::Error> {
    let file = Path::new(file);
    let mut pkg_str = String::new();
    File::open(file)
        .unwrap()
        .read_to_string(&mut pkg_str)
        .unwrap();
    let app_list: Dependency = toml::from_str(&pkg_str)?;
    Ok(app_list)
}

pub fn check(line: String, position: String) -> Result<bool, Box<EvalAltResult>>
//                          ^^^^^^^^^^^^^^^^^^
//                          Rhai API error type
{
    // Create an 'Engine'
    let engine = Engine::new();

    // Your first Rhai Script
    let script = "fn check(x, y) {if x == y { return true } else { return false } };";
    let mut scope = Scope::new();
    let ast = engine.compile(script)?;
    scope.push(line.clone(), position.clone());
    let name = "check";
    engine.call_fn::<bool>(&mut scope, &ast, name, (line, position))
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
pub fn into_string(n: u32) -> Result<String, String> {
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
