use std::path::Path;
use std::process::exit;

use clap::{Arg, ArgAction, Command};
use log::info;
use thirtyfour::{DesiredCapabilities, WebDriver};

use crate::config::load_config_from_file;
use crate::logging::logging::get_logging_config;
use crate::syspass::login::login_to_syspass;
use crate::syspass::perms::set_permissions_for_account_in_syspass;
use crate::xml::get_xml_config_from_file;

pub mod config;
pub mod types;
pub mod logging;
pub mod xml;
pub mod syspass;

#[cfg(test)]
pub mod tests;

pub const APP_NAME: &str = "app";

pub const CONFIG_FILE: &str = "spt.yml";

pub const SET_CMD: &str = "set";
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
        ).get_matches();

    let logging_config = get_logging_config("debug");

    match log4rs::init_config(logging_config) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e)
    }

    match matches.subcommand() {
        Some((SET_CMD, set_matches)) => {
            let xml_file_option = set_matches.get_one::<String>(XML_FILE_OPTION);

            match xml_file_option {
                Some(path) => {
                    let xml_file = Path::new(path);

                    if xml_file.is_file() && xml_file.exists() {

                        let config_file = Path::new(CONFIG_FILE);
                        match load_config_from_file(config_file) {
                            Ok(config) => {

                                let caps = DesiredCapabilities::chrome();
                                match WebDriver::new("http://localhost:9515", caps).await {
                                    Ok(driver) => {
                                        match login_to_syspass(&driver, &config.syspass_url,
                                               &config.auth.login, &config.auth.password).await {
                                            Ok(_) => {
                                                println!("user '{}' logged to syspass", &config.auth.login);

                                                match get_xml_config_from_file(xml_file) {
                                                    Ok(xml_config) => {

                                                        for account in xml_config.accounts {
                                                            match set_permissions_for_account_in_syspass(&driver, &config.syspass_url,
                                                                                                         &account.login, &config.user_permissions,
                                                                                                         &config.group_permissions).await {
                                                                Ok(_) => {
                                                                    println!("complete");
                                                                },
                                                                Err(e) => {
                                                                    println!("couldn't find account '{}', skip", account.login)
                                                                },
                                                            }
                                                        }

                                                    }
                                                    Err(e) => {
                                                        eprintln!("read xml error: {}", e.root_cause());
                                                        exit(EXIT_CODE_ERROR)
                                                    }
                                                }
                                            }
                                            Err(_) => {
                                                eprintln!("syspass auth error");
                                                exit(EXIT_CODE_ERROR)
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("webdriver error: {}", e);
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
        _ => println!("Use -h for help")
    }
}
