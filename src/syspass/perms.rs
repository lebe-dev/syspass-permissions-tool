use std::thread;
use std::time::Duration;

use log::{debug, info};
use thirtyfour::{By, Key, WebDriver};

use crate::config::PermissionsConfig;
use crate::types::EmptyResult;

pub async fn set_permissions_for_account_in_syspass(
    driver: &WebDriver, syspass_base_url: &str, login: &str,
    user_permissions: &PermissionsConfig, group_permissions: &PermissionsConfig) -> EmptyResult {

    info!("set permissions for syspass account '{}'", login);

    let url = format!("{}/index.php?r=index", syspass_base_url);

    driver.goto(&url).await?;
    driver.refresh().await?;

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

            break;
        }
    }

    Ok(())
}
