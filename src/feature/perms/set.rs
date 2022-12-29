use std::path::Path;
use std::thread;
use std::time::Duration;

use anyhow::anyhow;
use log::{debug, error, info};
use thirtyfour::{DesiredCapabilities, WebDriver};

use crate::cache::{ACCOUNTS_SET_CACHE_FILENAME, save_cache_data_into_file};
use crate::config::AppConfig;
use crate::syspass::Account;
use crate::syspass::login::{login_to_syspass, relogin_if_required};
use crate::syspass::perms::set_permissions_for_account;
use crate::types::EmptyResult;
use crate::xml::get_xml_config_from_file;

/// Set permissions for accounts from given xml-file
///
/// `latest_processed` - last successfully processed account.
pub async fn set_permissions_for_accounts_in_syspass(config: &AppConfig, xml_file: &Path,
                                               latest_processed_account: &Account) -> EmptyResult {
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

    let cache_file_path = Path::new(ACCOUNTS_SET_CACHE_FILENAME);
    let mut cache_items_counter: u16 = 0;

    let mut process_resumed = false;

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

                        if !process_resumed {
                            if !xml_account_matches_latest_processed(
                                &account.login, &account.name,
                                &category.name, &client.name,
                                &latest_processed_account) {
                                info!("xml account with login '{}' (name '{}') doesn't match with latest processed account, skip", &account.login, &account.name);

                            } else {
                                info!("xml account with login '{}' (name '{}') matched with latest processed account, process resumed", &account.login, &account.name);
                                process_resumed = true;
                            }

                            continue;
                        }

                        match set_permissions_for_account(
                            &config, &driver,
                            &account.login, &client.name,
                            &category.name
                        ).await {
                            Ok(_) => {
                                cache_items_counter += 1;
                                debug!("cache items counter: {}", cache_items_counter);

                                info!("permissions have been set for account login '{}'", account.login);

                                if cache_items_counter >= config.progress_cache.set_accounts {
                                    let account = get_account_from_xml_account(
                                        &account.login, &account.name,
                                        &category.name, &client.name
                                    );

                                    match save_cache_data_into_file(&account, cache_file_path) {
                                        Ok(_) => {
                                            info!("accounts cache has been updated");
                                            cache_items_counter = 0;
                                        },
                                        Err(e) => error!("cannot update accounts cache: {}", e)
                                    }
                                }

                            },
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

fn xml_account_matches_latest_processed(xml_account_login: &str,
                                        xml_account_name: &str,
                                        xml_account_category: &str,
                                        xml_account_client: &str,
                                        latest_processed_account: &Account) -> bool {

    latest_processed_account.login == xml_account_login &&
    latest_processed_account.name == xml_account_name &&
    latest_processed_account.category == xml_account_category &&
    latest_processed_account.client == xml_account_client
}

fn get_account_from_xml_account(xml_account_login: &str,
                                xml_account_name: &str,
                                xml_account_category: &str,
                                xml_account_client: &str) -> Account {
    Account {
        name: xml_account_name.to_string(),
        login: xml_account_login.to_string(),
        category: xml_account_category.to_string(),
        client: xml_account_client.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::feature::perms::set::xml_account_matches_latest_processed;
    use crate::tests::account::get_sample_account;
    use crate::tests::get_random_string;

    #[test]
    fn return_true_if_all_fields_match() {
        let account = get_sample_account();

        let xml_account_login = account.login.to_string();
        let xml_account_name = account.name.to_string();
        let xml_account_category = account.category.to_string();
        let xml_account_client = account.client.to_string();

        assert!(xml_account_matches_latest_processed(
            &xml_account_login, &xml_account_name, &xml_account_category,
            &xml_account_client, &account
        ));
    }

    #[test]
    fn return_false_if_one_field_does_not_match_at_least() {
        let mut account = get_sample_account();

        let xml_account_login = account.login.to_string();
        let xml_account_name = account.name.to_string();
        let xml_account_category = account.category.to_string();
        let xml_account_client = account.client.to_string();

        account.login = get_random_string();

        assert!(!xml_account_matches_latest_processed(
            &xml_account_login, &xml_account_name, &xml_account_category,
            &xml_account_client, &account
        ));

        account.name = get_random_string();

        assert!(!xml_account_matches_latest_processed(
            &xml_account_login, &xml_account_name, &xml_account_category,
            &xml_account_client, &account
        ));

        account.category = get_random_string();

        assert!(!xml_account_matches_latest_processed(
            &xml_account_login, &xml_account_name, &xml_account_category,
            &xml_account_client, &account
        ));

        account.client = get_random_string();

        assert!(!xml_account_matches_latest_processed(
            &xml_account_login, &xml_account_name, &xml_account_category,
            &xml_account_client, &account
        ));
    }
}
