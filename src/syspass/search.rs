use thirtyfour::{By, WebElement};

use crate::types::OperationResult;

pub async fn get_search_item_category(element: &WebElement) -> OperationResult<String> {
    let category_element = element.find(By::ClassName("field-category")).await?;
    let category_text_element = category_element.find(By::ClassName("field-text")).await?;
    let category_text = category_text_element.text().await?;
    Ok(category_text.trim().to_string())
}
