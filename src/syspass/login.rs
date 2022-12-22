use log::info;
use thirtyfour::{By, WebDriver};

use crate::config::AppConfig;
use crate::types::EmptyResult;

pub async fn login_to_syspass(driver: &WebDriver, syspass_base_url: &str,
                        login: &str, password: &str) -> EmptyResult {
    info!("login to syspass '{}' with '{}'", syspass_base_url, login);

    let url = format!("{}/index.php?r=login", syspass_base_url);

    driver.goto(&url).await?;

    let user_input = driver.find(By::Id("user")).await?;
    let password_input = driver.find(By::Id("pass")).await?;
    let login_button = driver.find(By::Id("btnLogin")).await?;

    user_input.send_keys(login).await?;
    password_input.send_keys(password).await?;

    login_button.click().await?;

    driver.find(By::ClassName("mdl-textfield__label")).await?;

    info!("user '{}' logged to syspass", &login);

    Ok(())
}

pub async fn relogin_if_required(driver: &WebDriver, config: &AppConfig) -> EmptyResult {
    let login_forms = driver.find_all(By::Id("frmLogin")).await?;

    if !login_forms.is_empty() {
        info!("relogin..");
        login_to_syspass(&driver, &config.syspass_url,
                         &config.auth.login, &config.auth.password).await?;
    }

    Ok(())
}
