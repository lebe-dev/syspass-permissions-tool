use std::fmt::{Display, Formatter};
use std::path::Path;
use std::thread;
use std::time::Duration;

use anyhow::anyhow;
use log::{debug, error, info, trace};
use thirtyfour::{By, DesiredCapabilities, WebDriver, WebElement};

use crate::cache::{ACCOUNTS_GET_CACHE_FILENAME, save_cache_data_into_file};
use crate::config::AppConfig;
use crate::syspass::{Account, ELEMENT_NOT_FOUND_ERROR, UNSUPPORTED_UI_VERSION_ERROR};
use crate::syspass::login::login_to_syspass;
use crate::syspass::perms::{get_tags_from_list_box_in_view_mode, go_to_account_view_page, open_permissions_tab};
use crate::syspass::search::{clear_search_input, get_search_item_category, get_search_item_client, get_search_item_login, get_search_item_name, next_page_available};
use crate::types::OperationResult;

pub struct AccountFilterOptions {
    pub category_name: String,
    pub client_name: String,
    pub login_starts_with: String,
    pub name_starts_with: String,
}

impl Display for AccountFilterOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<AccountFilterOptions> category-name '{}',", self.category_name)?;
        write!(f, "client-name '{}', login-starts-with '{}'", self.client_name, self.login_starts_with)?;
        write!(f, "name-starts-with '{}' </AccountFilterOptions>", self.name_starts_with)
    }
}

pub async fn get_accounts_with_empty_permissions(config: &AppConfig,
                         accounts_from_cache: &mut Vec<Account>,
                         filter_options: &AccountFilterOptions) -> OperationResult<Vec<Account>> {

    info!("get accounts with empty permissions from syspass instance");
    debug!("accounts in cache: {:?}", accounts_from_cache);
    debug!("filter options: {}", filter_options);

    let mut caps = DesiredCapabilities::chrome();

    for arg in config.webdriver.args.iter() {
        caps.add_chrome_arg(&arg)?;
    }

    let driver = WebDriver::new(&config.webdriver.url, caps).await?;

    login_to_syspass(&driver, &config.syspass_url,
                     &config.auth.login, &config.auth.password).await?;

    debug!("wait after login redirect {} ms", config.delays.after_login);
    thread::sleep(Duration::from_millis(config.delays.after_login));

    clear_search_input(&driver).await?;
    thread::sleep(Duration::from_millis(config.delays.after_login));

    let mut last_page = false;

    let mut has_errors = false;

    let accounts_from_cache_clone = accounts_from_cache.clone();
    let resume_cache_item = accounts_from_cache_clone.last();
    let mut resumed_from_cache = resume_cache_item.is_none();
    debug!("process resumed from cache: {}", resumed_from_cache);

    let mut accounts: Vec<Account> = vec![];
    accounts.append(accounts_from_cache);

    let mut search_item_offset = 0;

    let mut cache_items_counter: u16 = 0;

    let cache_file_path = Path::new(ACCOUNTS_GET_CACHE_FILENAME);

    while !last_page {

        let mut search_items = driver.find_all(By::ClassName("account-label")).await?;

        debug!("search item: {}", search_items.len());
        debug!("search item offset: {}", search_item_offset);

        while search_items.len() > search_item_offset {
            let item = search_items.iter().skip(search_item_offset)
                                                   .into_iter().collect::<Vec<&WebElement>>();

            match item.iter().next() {
                Some(search_item) => {
                    let account_category = get_search_item_category(&search_item).await?;
                    let account_client = get_search_item_client(&search_item).await?;
                    let account_login = get_search_item_login(&search_item).await?;
                    let account_name = get_search_item_name(&search_item).await?;

                    info!("processing account '{}' (login '{}')", account_name, account_login);

                    let account = Account {
                        name: account_name.to_string(),
                        login: account_login.to_string(),
                        category: account_category.to_string(),
                        client: account_client.to_string(),
                    };

                    if !resumed_from_cache {
                        match resume_cache_item {
                            Some(last_account_from_cache) => {
                                debug!("expect account '{}' with login '{}'",
                                    last_account_from_cache.name, last_account_from_cache.login);

                                if last_account_from_cache == &account {
                                    info!("resume process from account name '{}' and login '{}'",
                                          account_name, account_login);
                                    resumed_from_cache = true;

                                } else {
                                    info!("skip account, looking for account from cache");
                                }
                            }
                            None => {}
                        }

                        search_item_offset += 1;
                        continue;
                    }

                    if !account_matches_filters(&account, &filter_options) {
                        info!("account '{}' (login '{}') doesn't match filter options, skip",
                            account.name, account.login);
                        continue;
                    }

                    search_item.scroll_into_view().await?;

                    go_to_account_view_page(&search_item).await?;

                    thread::sleep(Duration::from_millis(config.delays.after_redirect_to_edit));

                    open_permissions_tab(&driver).await?;

                    let permissions_panel = driver.find(By::Id("permission-panel")).await?;

                    match account_has_empty_permissions(&permissions_panel).await {
                        Ok(has_empty_permissions) => {

                            if has_empty_permissions {
                                let account = Account {
                                    name: account_name,
                                    login: account_login,
                                    category: account_category,
                                    client: account_client,
                                };

                                info!("add account: {:?}", account);

                                accounts.push(account);

                                cache_items_counter += 1;
                                debug!("cache items counter: {}", cache_items_counter);

                                if cache_items_counter >= config.cache.save_accounts {
                                    match save_cache_data_into_file(&accounts, cache_file_path) {
                                        Ok(_) => {
                                            info!("accounts cache has been updated");
                                            cache_items_counter = 0;
                                        },
                                        Err(e) => error!("cannot update accounts cache: {}", e)
                                    }
                                }
                            }

                        },
                        Err(_) => {
                            has_errors = true;
                            break;
                        }
                    }

                    info!("back to search page");
                    let back_button = driver.find(By::Id("btnBack")).await?;
                    back_button.click().await?;

                    debug!("wait after redirect {} ms", config.delays.after_redirect_to_edit);
                    thread::sleep(Duration::from_millis(config.delays.after_redirect_to_edit));

                    search_item_offset += 1;

                    search_items = driver.find_all(By::ClassName("account-label")).await?;
                }
                None => {
                    search_item_offset = 0;
                    break
                }
            }


        }

        if has_errors {
            error!("interrupt process due error(s). check logs for details.");
            break;
        }

        last_page = !next_page_available(&driver).await;
        debug!("is it last page: {}", last_page);

        if !last_page {
            search_item_offset = 0;
            info!("go to next search results page..");
            let next_page_button = driver.find(By::Id("btn-pager-next")).await?;
            next_page_button.scroll_into_view().await?;
            next_page_button.click().await?;
            thread::sleep(Duration::from_millis(1000));
        }
    }

    Ok(accounts)
}

fn account_matches_filters(account: &Account, filter_options: &AccountFilterOptions) -> bool {
    let mut account_match = true;

    if !filter_options.login_starts_with.is_empty() {
        if !account.login.starts_with(&filter_options.login_starts_with) {
            account_match = false;
            trace!("login-start-with '{}' doesn't match with '{}'",
                filter_options.login_starts_with, account.login);
        }
    }

    if !filter_options.name_starts_with.is_empty() {
        if !account.name.starts_with(&filter_options.name_starts_with) {
            account_match = false;
            trace!("name-start-with '{}' doesn't match with '{}'",
                filter_options.name_starts_with, account.name);
        }
    }

    if !filter_options.category_name.is_empty() {
        if account.category != filter_options.category_name {
            account_match = false;
            trace!("category-name '{}' doesn't match with '{}'",
                filter_options.category_name, account.category);
        }
    }

    if !filter_options.client_name.is_empty() {
        if account.client != filter_options.client_name {
            account_match = false;
            trace!("client-name '{}' doesn't match with '{}'",
                filter_options.client_name, account.client);
        }
    }

    return account_match
}

async fn account_has_empty_permissions(permissions_panel_element: &WebElement) -> OperationResult<bool> {
    let permission_rows = permissions_panel_element.find_all(By::Tag("tr")).await?;

    if permission_rows.len() >= 2 {

        let users_block_element = permission_rows.first()
            .expect(ELEMENT_NOT_FOUND_ERROR);
        let users_perms = users_block_element.find_all(By::ClassName("tag-list-box")).await?;

        let users_view_perms = users_perms.first()
            .expect(ELEMENT_NOT_FOUND_ERROR);
        let users_view_tags = get_tags_from_list_box_in_view_mode(&users_view_perms).await?;
        debug!("users view tags: {:?}", users_view_tags);

        let users_edit_perms = users_perms.last()
            .expect(ELEMENT_NOT_FOUND_ERROR);
        let users_edit_tags = get_tags_from_list_box_in_view_mode(&users_edit_perms).await?;
        debug!("users edit tags: {:?}", users_edit_tags);

        let groups_block_element = permission_rows.get(1)
            .expect(ELEMENT_NOT_FOUND_ERROR);

        let groups_perms = groups_block_element.find_all(By::ClassName("tag-list-box")).await?;

        let groups_view_perms = groups_perms.first()
            .expect(ELEMENT_NOT_FOUND_ERROR);
        let groups_view_tags = get_tags_from_list_box_in_view_mode(&groups_view_perms).await?;
        debug!("group view tags: {:?}", groups_view_tags);

        let groups_edit_perms = groups_perms.last()
            .expect(ELEMENT_NOT_FOUND_ERROR);
        let groups_edit_tags = get_tags_from_list_box_in_view_mode(&groups_edit_perms).await?;
        debug!("group edit tags: {:?}", groups_edit_tags);

        let has_empty_permissions = users_view_tags.is_empty() &&
            users_edit_tags.is_empty() &&
            groups_view_tags.is_empty() &&
            groups_edit_tags.is_empty();

        Ok(has_empty_permissions)

    } else {
        error!("expected at least two 'tr' rows on permissions tab");
        Err(anyhow!("{}", UNSUPPORTED_UI_VERSION_ERROR))
    }
}

#[cfg(test)]
mod tests {
    use crate::feature::perms::get::{account_matches_filters, AccountFilterOptions};
    use crate::syspass::Account;
    use crate::tests::{get_random_string, init_logging};

    #[test]
    fn return_true_for_match() {
        init_logging();

        let filter_options = get_account_filter_options();

        let account = get_account();

        assert!(account_matches_filters(&account, &filter_options));
    }

    #[test]
    fn return_false_for_category_mismatch() {
        let filter_options = get_account_filter_options();

        let mut account = get_account();
        account.category = get_random_string();

        assert!(!account_matches_filters(&account, &filter_options));
    }

    #[test]
    fn return_false_for_client_mismatch() {
        let filter_options = get_account_filter_options();

        let mut account = get_account();
        account.client = get_random_string();

        assert!(!account_matches_filters(&account, &filter_options));
    }

    #[test]
    fn return_false_for_name_mask_mismatch() {
        let filter_options = get_account_filter_options();

        let mut account = get_account();
        account.name = get_random_string();

        assert!(!account_matches_filters(&account, &filter_options));
    }

    #[test]
    fn return_false_for_login_mask_mismatch() {
        let filter_options = get_account_filter_options();

        let mut account = get_account();
        account.login = get_random_string();

        assert!(!account_matches_filters(&account, &filter_options));
    }

    #[test]
    fn ignore_filters_with_blank_values() {
        let mut filter_options = get_account_filter_options();
        filter_options.category_name = String::new();
        filter_options.client_name = String::new();
        filter_options.login_starts_with = String::new();
        filter_options.name_starts_with = String::new();

        let account = get_account();

        assert!(account_matches_filters(&account, &filter_options));
    }

    fn get_account_filter_options() -> AccountFilterOptions {
        AccountFilterOptions {
            category_name: "Apps".to_string(),
            client_name: "BirchShop".to_string(),
            login_starts_with: "demo".to_string(),
            name_starts_with: "Hercules".to_string(),
        }
    }

    fn get_account() -> Account {
        Account {
            name: "Hercules II".to_string(),
            login: "demo-acc".to_string(),
            category: "Apps".to_string(),
            client: "BirchShop".to_string(),
        }
    }
}
