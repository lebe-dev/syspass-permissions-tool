use std::path::Path;
use std::process::exit;

use clap::{Arg, ArgAction, ArgMatches, Command};
use log::{error, info};

use crate::cache::{ACCOUNTS_CACHE_FILENAME, load_accounts_from_file};
use crate::config::load_config_from_file;
use crate::feature::perms::get::{AccountFilterOptions, get_accounts_with_empty_permissions};
use crate::feature::perms::set::set_permissions_for_accounts_in_syspass;
use crate::logging::logging::get_logging_config;
use crate::syspass::Account;

pub mod config;
pub mod types;
pub mod logging;
pub mod xml;
pub mod feature;
pub mod syspass;
pub mod cache;

#[cfg(test)]
pub mod tests;

pub const APP_NAME: &str = "spt";

pub const CONFIG_FILE: &str = "spt.yml";

pub const SET_CMD: &str = "set";
pub const GET_EMPTY_CMD: &str = "get-empty";

pub const XML_FILE_OPTION: &str = "xml-file";

pub const RESUME_OPTION: &str = "resume";

pub const CATEGORY_FILTER_OPTION: &str = "category";
pub const CLIENT_FILTER_OPTION: &str = "client";
pub const LOGIN_STARTS_WITH_FILTER_OPTION: &str = "login-starts-with";
pub const NAME_STARTS_WITH_FILTER_OPTION: &str = "name-starts-with";

const EXIT_CODE_ERROR: i32 = 1;

#[tokio::main]
async fn main() {
    let matches = Command::new(APP_NAME)
        .name(APP_NAME)
        .bin_name(APP_NAME)
        .about("Permissions Tool for sysPass")
        .version("0.4.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
        Command::new(SET_CMD)
                .about("Set permissions for accounts")
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
                .arg(
                    Arg::new(RESUME_OPTION)
                        .long(RESUME_OPTION)
                        .help("resume process from last error")
                        .action(ArgAction::SetTrue)
                        .required(false)
                )
                .arg(
                    Arg::new(CATEGORY_FILTER_OPTION)
                        .long(CATEGORY_FILTER_OPTION)
                        .help("filter by category name")
                        .default_value("")
                        .action(ArgAction::Set)
                        .required(false)
                )
                .arg(
                    Arg::new(CLIENT_FILTER_OPTION)
                        .long(CLIENT_FILTER_OPTION)
                        .help("filter by client name")
                        .default_value("")
                        .action(ArgAction::Set)
                        .required(false)
                )
                .arg(
                    Arg::new(LOGIN_STARTS_WITH_FILTER_OPTION)
                        .long(LOGIN_STARTS_WITH_FILTER_OPTION)
                        .help("filter by login starts with")
                        .default_value("")
                        .action(ArgAction::Set)
                        .required(false)
                )
                .arg(
                    Arg::new(NAME_STARTS_WITH_FILTER_OPTION)
                        .long(NAME_STARTS_WITH_FILTER_OPTION)
                        .help("filter by name starts with")
                        .default_value("")
                        .action(ArgAction::Set)
                        .required(false)
                )
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
        Some((GET_EMPTY_CMD, get_matches)) => {

            match load_config_from_file(config_file) {
                Ok(config) => {

                    let mut accounts_from_cache = get_accounts_cache(get_matches);

                    let account_filter_options = get_account_filter_options(get_matches);

                    match get_accounts_with_empty_permissions(&config, &mut accounts_from_cache,
                                                              &account_filter_options).await {
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

fn get_accounts_cache(matches: &ArgMatches) -> Vec<Account> {
    let resume_option = matches.get_flag(RESUME_OPTION);

    if resume_option {
        let cache_file = Path::new(ACCOUNTS_CACHE_FILENAME);

        match load_accounts_from_file(cache_file) {
            Ok(accounts) => accounts,
            Err(e) => {
                info!("couldn't load accounts from cache file: {}, skip", e);
                vec![]
            }
        }

    } else {
        vec![]
    }
}

fn get_account_filter_options(matches: &ArgMatches) -> AccountFilterOptions {
    let category_name = matches.get_one::<String>(CATEGORY_FILTER_OPTION);
    let client_name = matches.get_one::<String>(CLIENT_FILTER_OPTION);
    let login_starts_with = matches.get_one::<String>(LOGIN_STARTS_WITH_FILTER_OPTION);
    let name_starts_with = matches.get_one::<String>(NAME_STARTS_WITH_FILTER_OPTION);

    AccountFilterOptions {
        category_name: get_string_or_blank(category_name),
        client_name: get_string_or_blank(client_name),
        login_starts_with: get_string_or_blank(login_starts_with),
        name_starts_with: get_string_or_blank(name_starts_with),
    }
}

fn get_string_or_blank(value: Option<&String>) -> String {
    match value {
        Some(value_string) => value_string.to_string(),
        None => String::new()
    }
}
