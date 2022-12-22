use std::path::Path;
use std::process::exit;

use clap::{Arg, ArgAction, Command};
use log::error;

use crate::config::load_config_from_file;
use crate::feature::perms::get::get_accounts_with_empty_permissions;
use crate::feature::perms::set::set_permissions_for_accounts_in_syspass;
use crate::logging::logging::get_logging_config;

pub mod config;
pub mod types;
pub mod logging;
pub mod xml;
pub mod feature;
pub mod syspass;

#[cfg(test)]
pub mod tests;

pub const APP_NAME: &str = "app";

pub const CONFIG_FILE: &str = "spt.yml";

pub const SET_CMD: &str = "set";
pub const GET_EMPTY_CMD: &str = "get-empty";

pub const XML_FILE_OPTION: &str = "xml-file";

const EXIT_CODE_ERROR: i32 = 1;

#[tokio::main]
async fn main() {
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
                        .default_value("import.xml")
                        .help("xml file with accounts")
                        .action(ArgAction::Set)
                        .required(false),
                )
        )
        .subcommand(
            Command::new(GET_EMPTY_CMD)
                .about("Get accounts with empty permissions")
        )
        .get_matches();

    let logging_config = get_logging_config("debug");

    match log4rs::init_config(logging_config) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e)
    }

    let config_file = Path::new(CONFIG_FILE);

    match matches.subcommand() {
        Some((SET_CMD, set_matches)) => {
            let xml_file_option = set_matches.get_one::<String>(XML_FILE_OPTION);

            match xml_file_option {
                Some(path) => {
                    let xml_file = Path::new(path);

                    if xml_file.is_file() && xml_file.exists() {

                        match load_config_from_file(config_file) {
                            Ok(config) => {

                                match set_permissions_for_accounts_in_syspass(&config, xml_file).await {
                                    Ok(_) => println!("complete"),
                                    Err(e) => {
                                        eprintln!("error: {}", e.root_cause());
                                        exit(EXIT_CODE_ERROR)
                                    }
                                }

                            }
                            Err(e) => {
                                eprintln!("couldn't load config: {}", e);
                                exit(EXIT_CODE_ERROR)
                            }
                        }

                    } else {
                        eprintln!("xml file wasn't found '{}'", xml_file.display());
                        exit(EXIT_CODE_ERROR)
                    }
                }
                None => {}
            }
        },
        Some((GET_EMPTY_CMD, _)) => {

            match load_config_from_file(config_file) {
                Ok(config) => {

                    match get_accounts_with_empty_permissions(&config).await {
                        Ok(accounts) => {
                            match serde_json::to_string(&accounts) {
                                Ok(accounts_str) => println!("{}", accounts_str),
                                Err(e) => {
                                    error!("{}", e);
                                    exit(EXIT_CODE_ERROR)
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("error: {}", e.root_cause());
                            exit(EXIT_CODE_ERROR)
                        }
                    }

                }
                Err(e) => {
                    eprintln!("couldn't load config: {}", e);
                    exit(EXIT_CODE_ERROR)
                }
            }

        },
        _ => println!("Use -h for help")
    }
}
