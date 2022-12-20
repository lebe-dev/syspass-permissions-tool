use std::thread;
use std::time::Duration;

use anyhow::anyhow;
use log::{debug, error, info};
use thirtyfour::{By, Key, WebDriver, WebElement};

use crate::config::PermissionsConfig;
use crate::syspass::search::{get_search_item_category, get_search_item_client, get_search_item_login};
use crate::syspass::UNSUPPORTED_UI_VERSION_ERROR;
use crate::types::{EmptyResult, OperationResult};

pub async fn set_permissions_for_account(
    driver: &WebDriver, syspass_base_url: &str, account_login: &str,
    account_client: &str, account_category: &str,
    permissions: &PermissionsConfig) -> EmptyResult {

    info!("set permissions for syspass account '{}'", account_login);

    let url = format!("{}/index.php?r=index", syspass_base_url);

    driver.goto(&url).await?;

    thread::sleep(Duration::new(2, 0));

    let search_input = driver.find(By::Id("search")).await?;
    search_input.clear().await?;
    search_input.send_keys(account_login + Key::Enter).await?;

    thread::sleep(Duration::new(1, 0));

    let search_result_elements = driver.find_all(By::ClassName("account-label")).await?;

    for search_result_element in search_result_elements {
        let item_client = get_search_item_client(&search_result_element).await?;
        debug!("client: '{}'", item_client);

        let item_category = get_search_item_category(&search_result_element).await?;
        debug!("category: '{}'", item_category);

        let item_login = get_search_item_login(&search_result_element, ).await?;
        debug!("username: '{}'", item_login);

        if item_login == account_login && item_client == account_client && item_category == account_category {
            debug!("going to account edit page");
            open_account_actions_menu(&search_result_element).await?;

            thread::sleep(Duration::from_millis(300));

            go_to_account_edit_page(&search_result_element).await?;

            thread::sleep(Duration::new(1, 0));

            let tabs = driver.find_all(By::ClassName("mdl-tabs__tab")).await?;

            let perms_tab = tabs.get(1).unwrap();
            perms_tab.click().await?;

            let click_for_close_element = driver.find(By::Id("frmAccount")).await?;

            let perm_inputs = driver.find_all(By::ClassName("tag-list-box")).await?;

            if perm_inputs.len() == 4 {

                set_permissions_for_security_entities(&perm_inputs, permissions, &click_for_close_element).await?;

                let permission_panel = driver.find(By::Id("permission-panel")).await?;
                let form_rows = permission_panel.find_all(By::Tag("tr")).await?;

                info!("form rows: {}", form_rows.len());

                let owner_row = form_rows.get(2).unwrap();
                info!("set owner");
                set_additional_property_value(&owner_row, &permissions.owner).await?;

                click_for_close_element.click().await?;

                let main_group_row = form_rows.get(3).unwrap();
                info!("set main group");
                set_additional_property_value(&main_group_row, &permissions.main_group).await?;

                let private_account_switch = form_rows.get(4).unwrap();
                debug!("check if 'private account option' enabled");
                let private_account_switch_status = is_checkbox_enabled(private_account_switch).await?;

                if permissions.private_account != private_account_switch_status {
                    let option_switch = private_account_switch.find(By::ClassName("mdl-switch")).await?;
                    option_switch.click().await?;
                }

                let private_account_for_group_switch = form_rows.get(5).unwrap();
                debug!("check if 'private account for group' option enabled");
                let private_account_for_group_switch_status = is_checkbox_enabled(private_account_for_group_switch).await?;

                if permissions.private_account_for_group != private_account_for_group_switch_status {
                    let option_switch = private_account_for_group_switch.find(By::ClassName("mdl-switch")).await?;
                    option_switch.click().await?;
                }

                let save_button = permission_panel.find(By::Id("1")).await?;
                save_button.click().await?;
                info!("settings have been saved");

                driver.goto(&url).await?;
                debug!("returned to index page");

            } else {
                error!("unsupported syspass ui version, 4 divs expected with class 'tag-list-box'")
            }

            break;
        }
    }

    Ok(())
}

pub async fn open_account_actions_menu(element: &WebElement) -> EmptyResult {
    let actions_block = element.find(By::ClassName("account-actions")).await?;

    let more_actions = actions_block.find(By::Tag("button")).await?;
    more_actions.click().await?;

    Ok(())
}

pub async fn go_to_account_edit_page(element: &WebElement) -> EmptyResult {
    let menu = element.find(By::ClassName("mdl-menu__container")).await?;

    let menu_items = menu.find_all(By::ClassName("btn-action")).await?;

    match menu_items.first() {
        Some(edit_item) => {
            edit_item.click().await?;
            Ok(())
        }
        None => {
            error!("couldn't find 'btn-action' element inside 'mdl-menu__container'");
            Err(anyhow!(UNSUPPORTED_UI_VERSION_ERROR))
        }
    }
}

pub async fn is_checkbox_enabled(element: &WebElement) -> OperationResult<bool> {
    let elements = element.find_all(By::ClassName("is-checked")).await?;

    let status = !elements.is_empty();
    debug!("checkbox enabled: {}", status);

    Ok(status)
}

pub async fn set_permissions_for_security_entities(perm_inputs: &Vec<WebElement>,
                                                   permissions: &PermissionsConfig,
                                                   click_for_close_element: &WebElement) -> EmptyResult {
    info!("add user view permissions");
    set_permissions_for_security_entity(&perm_inputs, 0,
                                        &permissions.user.view).await?;

    click_for_close_element.click().await?;

    info!("add user edit permissions");
    set_permissions_for_security_entity(&perm_inputs, 1,
                                        &permissions.user.edit).await?;

    click_for_close_element.click().await?;

    info!("add group view permissions");
    set_permissions_for_security_entity(&perm_inputs, 2,
                                        &permissions.group.view).await?;

    click_for_close_element.click().await?;

    info!("add group edit permissions");
    set_permissions_for_security_entity(&perm_inputs, 3,
                                        &permissions.group.edit).await?;

    click_for_close_element.click().await?;

    Ok(())
}

pub async fn set_permissions_for_security_entity(perm_inputs: &Vec<WebElement>,
                                                 perm_input_index: usize,
                                                 permissions: &Vec<String>) -> EmptyResult {
    debug!("set permissions for security entity: {:?}", permissions);

    if !permissions.is_empty() {
        let perms_input = perm_inputs.get(perm_input_index).unwrap();
        perms_input.click().await?;

        let current_perms = perms_input.find_all(By::ClassName("remove")).await?;
        for current_perm in current_perms {
            current_perm.click().await?;
        }
        debug!("current permissions have been removed");

        for permission in permissions {
            info!("- add '{}'", permission);
            let options = perms_input.find_all(By::ClassName("option")).await?;

            for option in options {
                let text = option.text().await?;

                if &text == permission {
                    info!("- add '{}' - success", permission);
                    option.click().await?;
                }
            }
        }

        perms_input.click().await?;
    }

    Ok(())
}

pub async fn set_additional_property_value(element: &WebElement, value: &str) -> EmptyResult {
    info!("set value '{}'", value);
    let input = element.find(By::ClassName("selectize-control")).await?;
    input.click().await?;

    let options = element.find_all(By::ClassName("option")).await?;

    for option in options {
        let text = option.text().await?;

        if &text == value {
            info!("- set '{}' - success", value);
            option.click().await?;
        }
    }

    Ok(())
}
