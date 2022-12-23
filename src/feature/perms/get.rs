use std::thread;
use std::time::Duration;

use log::{debug, error, info};
use thirtyfour::{By, DesiredCapabilities, WebDriver, WebElement};

use crate::config::AppConfig;
use crate::syspass::{Account, ELEMENT_NOT_FOUND_ERROR, UNSUPPORTED_UI_VERSION_ERROR};
use crate::syspass::login::{login_to_syspass, relogin_if_required};
use crate::syspass::perms::{get_tags_from_list_box_in_view_mode, go_to_account_view_page, open_permissions_tab};
use crate::syspass::search::{clear_search_input, get_search_item_category, get_search_item_client, get_search_item_login, get_search_item_name, next_page_available};
use crate::types::OperationResult;

pub async fn get_accounts_with_empty_permissions(config: &AppConfig) -> OperationResult<Vec<Account>> {
    info!("get accounts with empty permissions from syspass instance");

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

    let mut accounts: Vec<Account> = vec![];

    let mut search_item_offset = 0;

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

                    search_item.scroll_into_view().await?;

                    go_to_account_view_page(&search_item).await?;

                    thread::sleep(Duration::from_millis(config.delays.after_redirect_to_edit));

                    open_permissions_tab(&driver).await?;

                    let permissions_panel = driver.find(By::Id("permission-panel")).await?;

                    let permission_rows = permissions_panel.find_all(By::Tag("tr")).await?;

                    if permission_rows.len() >= 2 {

                        let users_block_element = permission_rows.first().expect(ELEMENT_NOT_FOUND_ERROR);
                        let users_perms = users_block_element.find_all(By::ClassName("tag-list-box")).await?;

                        let users_view_perms = users_perms.first().expect(ELEMENT_NOT_FOUND_ERROR);
                        let users_view_tags = get_tags_from_list_box_in_view_mode(&users_view_perms).await?;
                        debug!("users view tags: {:?}", users_view_tags);

                        let users_edit_perms = users_perms.last().expect(ELEMENT_NOT_FOUND_ERROR);
                        let users_edit_tags = get_tags_from_list_box_in_view_mode(&users_edit_perms).await?;
                        debug!("users edit tags: {:?}", users_edit_tags);

                        let groups_block_element = permission_rows.get(1).expect(ELEMENT_NOT_FOUND_ERROR);

                        let groups_perms = groups_block_element.find_all(By::ClassName("tag-list-box")).await?;

                        let groups_view_perms = groups_perms.first().expect(ELEMENT_NOT_FOUND_ERROR);
                        let groups_view_tags = get_tags_from_list_box_in_view_mode(&groups_view_perms).await?;
                        debug!("group view tags: {:?}", groups_view_tags);

                        let groups_edit_perms = groups_perms.last().expect(ELEMENT_NOT_FOUND_ERROR);
                        let groups_edit_tags = get_tags_from_list_box_in_view_mode(&groups_edit_perms).await?;
                        debug!("group edit tags: {:?}", groups_edit_tags);

                        if users_view_tags.is_empty() && users_edit_tags.is_empty() && groups_view_tags.is_empty() &&
                            groups_edit_tags.is_empty() {

                            let account = Account {
                                name: account_name,
                                login: account_login,
                                category: account_category,
                                client: account_client,
                            };

                            info!("add account: {:?}", account);

                            accounts.push(account);
                        }

                    } else {
                        error!("expected at least two 'tr' rows on permissions tab");
                        error!("{}", UNSUPPORTED_UI_VERSION_ERROR);
                        has_errors = true;
                        break;
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
