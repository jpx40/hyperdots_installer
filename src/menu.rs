use crate::conf;
use crate::installer;
use crate::utils;
use crate::Cli;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::env;
use std::io;
use std::process;
pub fn menu(c: Cli) -> Result<()> {
    env_logger::init();
    let mut rl = DefaultEditor::new()?;

    // let line = rl.readline(">> ")?; // read
    // println!("Line: {line}"); // eval / print
    //
    //
    //
    println!("\n\nWelcome to the Installer");
    println!("\n");

    loop {
        println!("Choose Software");
        println!("1. From Config, 1. All | Default: 1");
        let readline = rl.readline(">> "); // read
                                           // eval / printy

        match readline {
            Ok(line) => match line.to_lowercase().as_str() {
                "y" => {
                    println!("Backup of Config");
                    let backup = false;
                    utils::backup(&c.backup_path, backup).unwrap();
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
                process::exit(0);
            }
        }
    } // loop
    loop {
        println!("Backup | y/n | Default: y");
        let readline = rl.readline(">> "); // read
                                           // eval / printy

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
                process::exit(0);
            }
        }
    } // loop
    Ok(())
}
