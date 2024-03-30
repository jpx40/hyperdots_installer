use crate::installer::AppList;
use chrono::Local;
use copy_dir::copy_dir;
use rhai::{Engine, EvalAltResult, Scope};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

pub fn read_app_list() -> Result<AppList, toml::de::Error> {
    let file = Path::new("app_list.toml");
    let mut pkg_str = String::new();
    File::open(file)
        .unwrap()
        .read_to_string(&mut pkg_str)
        .unwrap();
    let app_list: AppList = toml::from_str(&pkg_str)?;
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
