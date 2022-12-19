use std::thread;
use std::time::Duration;

use log::{debug, error, info};
use thirtyfour::{By, Key, WebDriver, WebElement};

use crate::config::EntityPermissionsConfig;
use crate::types::EmptyResult;

pub async fn set_permissions_for_account_in_syspass(
    driver: &WebDriver, syspass_base_url: &str, login: &str,
    user_permissions: &EntityPermissionsConfig, group_permissions: &EntityPermissionsConfig) -> EmptyResult {

    info!("set permissions for syspass account '{}'", login);

    let url = format!("{}/index.php?r=index", syspass_base_url);

    driver.goto(&url).await?;

    thread::sleep(Duration::new(2, 0));

    let search_input = driver.find(By::Id("search")).await?;
    search_input.clear().await?;
    search_input.send_keys(login + Key::Enter).await?;

    thread::sleep(Duration::new(1, 0));

    let elements = driver.find_all(By::ClassName("account-label")).await?;

    for element in elements {
        let user_field = element.find(By::ClassName("field-user")).await?;

        let username_field = user_field.find(By::ClassName("field-text")).await?;

        let username = username_field.text().await?;
        let username_trimmed = username.trim();
        debug!("username: '{}'", username_trimmed);

        if username == login {
            let actions_block = element.find(By::ClassName("account-actions")).await?;

            let more_actions = actions_block.find(By::Tag("button")).await?;
            more_actions.click().await?;

            thread::sleep(Duration::from_millis(300));

            let menu = element.find(By::ClassName("mdl-menu__container")).await?;

            let menu_items = menu.find_all(By::ClassName("btn-action")).await?;

            let edit_item = menu_items.first().unwrap();

            edit_item.click().await?;

            thread::sleep(Duration::new(1, 0));

            let tabs = driver.find_all(By::ClassName("mdl-tabs__tab")).await?;

            if tabs.len() == 3 {
                let perms_tab = tabs.get(1).unwrap();
                perms_tab.click().await?;

                let click_for_close_element = driver.find(By::Id("frmAccount")).await?;

                let perm_inputs = driver.find_all(By::ClassName("tag-list-box")).await?;

                if perm_inputs.len() == 4 {
                    info!("add user view permissions");
                    set_permissions_for_security_entity(&perm_inputs, 0,
                                                        &user_permissions.view).await?;

                    click_for_close_element.click().await?;

                    info!("add user edit permissions");
                    set_permissions_for_security_entity(&perm_inputs, 1,
                                                        &user_permissions.edit).await?;

                    click_for_close_element.click().await?;

                    info!("add group view permissions");
                    set_permissions_for_security_entity(&perm_inputs, 2,
                                                        &group_permissions.view).await?;

                    click_for_close_element.click().await?;

                    info!("add group edit permissions");
                    set_permissions_for_security_entity(&perm_inputs, 3,
                                                        &group_permissions.edit).await?;

                    click_for_close_element.click().await?;

                    let form_rows = driver.find_all(By::ClassName("valField")).await?;

                    if form_rows.len() >= 6 {


                    } else {
                        error!("unsupported syspass ui version, at least six form rows expected")
                    }


                } else {
                    error!("unsupported syspass ui version, 4 divs expected with class 'tag-list-box'")
                }

            } else {
                error!("unsupported syspass ui version, 3 tabs expected")
            }

            break;
        }
    }

    Ok(())
}

pub async fn set_permissions_for_security_entity(perm_inputs: &Vec<WebElement>,
                                                 perm_input_index: usize,
                                                 permissions: &Vec<String>) -> EmptyResult {
    debug!("set permissions for security entity: {:?}", permissions);

    if !permissions.is_empty() {
        let perms_input = perm_inputs.get(perm_input_index).unwrap();
        perms_input.click().await?;

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
    }

    Ok(())
}
