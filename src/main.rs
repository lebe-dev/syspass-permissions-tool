use std::path::Path;
use std::process::exit;

use clap::{Arg, ArgAction, Command};

pub mod config;
pub mod types;
pub mod logging;
pub mod xml;

#[cfg(test)]
pub mod tests;

pub const APP_NAME: &str = "app";

pub const CONFIG_FILE: &str = "spt.yml";

pub const SET_CMD: &str = "set";
pub const XML_FILE_OPTION: &str = "xml-file";

const EXIT_CODE_ERROR: i32 = 1;

fn main() {
    let matches = Command::new(APP_NAME)
        .name(APP_NAME)
        .bin_name(APP_NAME)
        .about("Permissions Tool for sysPass")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(SET_CMD)
                .about("Set permissions for users")
                .arg(
                    Arg::new(XML_FILE_OPTION)
                        .long(XML_FILE_OPTION)
                        .help("xml file with accounts")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
        ).get_matches();


    match matches.subcommand() {
        Some((SET_CMD, set_matches)) => {
            let xml_file_option = set_matches.get_one::<String>(XML_FILE_OPTION);

            match xml_file_option {
                Some(path) => {
                    let xml_file = Path::new(path);

                    if xml_file.is_file() && xml_file.exists() {
                        unimplemented!()

                    } else {
                        eprintln!("xml file wasn't found '{}'", xml_file.display());
                        exit(EXIT_CODE_ERROR)
                    }
                }
                None => {}
            }
        },
        _ => println!("Use -h for help")
    }
}
