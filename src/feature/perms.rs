use std::path::Path;
use std::thread;
use std::time::Duration;

use anyhow::anyhow;
use log::{debug, error, info};
use thirtyfour::{By, DesiredCapabilities, WebDriver};

use crate::config::AppConfig;
use crate::syspass::login::login_to_syspass;
use crate::syspass::perms::set_permissions_for_account;
use crate::types::EmptyResult;
use crate::xml::get_xml_config_from_file;

pub async fn set_permissions_for_accounts_in_syspass(config: &AppConfig,
                                                     xml_file: &Path) -> EmptyResult {
    let mut caps = DesiredCapabilities::chrome();

    for arg in config.webdriver.args.iter() {
        caps.add_chrome_arg(&arg)?;
    }

    let driver = WebDriver::new(&config.webdriver.url, caps).await?;

    let xml_config = get_xml_config_from_file(xml_file)?;

    login_to_syspass(&driver, &config.syspass_url,
                     &config.auth.login, &config.auth.password).await?;

    info!("user '{}' logged to syspass", &config.auth.login);

    debug!("wait after login redirect {} ms", config.delays.after_login);
    thread::sleep(Duration::from_millis(config.delays.after_login));

    let mut has_errors = false;

    let accounts_count = xml_config.accounts.len();

    let separator = "-".repeat(128);

    for (i, account) in xml_config.accounts.iter().enumerate() {
        info!("{}", separator);
        info!("PROCESSING '{}' [{}/{}]", account, i, accounts_count);
        info!("{}", separator);
        let client_found = xml_config.clients.iter()
            .find(|client|client.id == account.client_id);

        match client_found {
            Some(client) => {

                let category_found = xml_config.categories.iter()
                                .find(|category| category.id == account.category_id);

                match category_found {
                    Some(category) => {

                        relogin_if_required(&driver, &config).await?;

                        match set_permissions_for_account(
                            &config, &driver,
                            &account.login, &client.name,
                            &category.name
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
            info!("process has been interrupted due error");
            break;
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

async fn relogin_if_required(driver: &WebDriver, config: &AppConfig) -> EmptyResult {
    let login_forms = driver.find_all(By::Id("frmLogin")).await?;

    if !login_forms.is_empty() {
        info!("relogin..");
        login_to_syspass(&driver, &config.syspass_url,
                         &config.auth.login, &config.auth.password).await?;
    }

    Ok(())
}
