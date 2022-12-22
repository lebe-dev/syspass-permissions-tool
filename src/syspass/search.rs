use thirtyfour::{By, WebDriver, WebElement};

use crate::types::{EmptyResult, OperationResult};

pub async fn get_search_item_category(element: &WebElement) -> OperationResult<String> {
    let category_element = element.find(By::ClassName("field-category")).await?;
    let category_text_element = category_element.find(By::ClassName("field-text")).await?;
    let category_text = category_text_element.text().await?;
    Ok(category_text.trim().to_string())
}

pub async fn get_search_item_client(element: &WebElement) -> OperationResult<String> {
    let client_element = element.find(By::ClassName("mdl-chip__text")).await?;
    let client_text = client_element.text().await?;
    Ok(client_text.trim().to_string())
}

pub async fn get_search_item_login(element: &WebElement) -> OperationResult<String> {
    let user_field = element.find(By::ClassName("field-user")).await?;
    let username_field = user_field.find(By::ClassName("field-text")).await?;
    let username = username_field.text().await?;
    Ok(username.trim().to_string())
}

pub async fn get_search_item_name(element: &WebElement) -> OperationResult<String> {
    let name_element = element.find(By::ClassName("field-account")).await?;
    let name_text_element = name_element.find(By::ClassName("field-text")).await?;
    let name_text = name_text_element.text().await?;
    Ok(name_text.trim().to_string())
}

pub async fn clear_search_input(driver: &WebDriver) -> EmptyResult {
    let input = driver.find(By::Id("btn-reset")).await?;
    input.click().await?;
    Ok(())
}

pub async fn next_page_available(driver: &WebDriver) -> bool {
    driver.find(By::Id("btn-pager-last")).await.is_ok()
}
