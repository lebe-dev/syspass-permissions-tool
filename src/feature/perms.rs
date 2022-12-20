use std::path::Path;

use anyhow::anyhow;
use log::{error, info};
use thirtyfour::{DesiredCapabilities, WebDriver};

use crate::config::AppConfig;
use crate::syspass::login::login_to_syspass;
use crate::syspass::perms::set_permissions_for_account;
use crate::types::EmptyResult;
use crate::xml::get_xml_config_from_file;

pub async fn set_permissions_for_accounts_in_syspass(config: &AppConfig,
                                                     xml_file: &Path) -> EmptyResult {
    let caps = DesiredCapabilities::chrome();

    let driver = WebDriver::new(&config.webdriver_url, caps).await?;

    let xml_config = get_xml_config_from_file(xml_file)?;

    login_to_syspass(&driver, &config.syspass_url,
                     &config.auth.login, &config.auth.password).await?;

    info!("user '{}' logged to syspass", &config.auth.login);

    let mut has_errors = false;

    for account in xml_config.accounts {
        let client_found = xml_config.clients.iter()
            .find(|client|client.id == account.client_id);

        match client_found {
            Some(client) => {

                let category_found = xml_config.categories.iter()
                    .find(|category| category.id == account.category_id);

                match category_found {
                    Some(category) => {
                        match set_permissions_for_account(
                            &driver, &config.syspass_url,
                            &account.login, &client.name,
                            &category.name, &config.permissions
                        ).await {
                            Ok(_) => info!("permissions have been set for account login '{}'", account.login),
                            Err(e) => {
                                error!("{}", e);
                                error!("couldn't find account '{}'", account.login);
                                has_errors = true;
                            },
                        }
                    }
                    None => {
                        error!("account configuration error, client wasn't found by id {}", account.category_id);
                        has_errors = true;
                    }
                }

            }
            None => {
                error!("account configuration error, client wasn't found by id {}", account.client_id);
                has_errors = true;
            }
        }

        if has_errors && !config.ignore_errors {
            info!("process has been interrupted due error")
        }

    }

    if !has_errors {
        info!("permissions have been set for accounts");
        Ok(())

    } else {
        if !config.ignore_errors {
            Err(anyhow!("process has been interrupted due error"))

        } else {
            info!("permissions have been partially set for accounts");
            Ok(())
        }
    }

}
