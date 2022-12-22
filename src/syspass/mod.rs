use serde::Serialize;

pub mod login;
pub mod perms;
pub mod search;

pub const UNSUPPORTED_UI_VERSION_ERROR: &str = "unsupported ui version, check logs for details";
pub const ELEMENT_NOT_FOUND_ERROR: &str = "unexpected error, element wasn't found";

#[derive(Serialize,Debug)]
pub struct Account {
    pub name: String,
    pub login: String,
    pub category: String,
    pub client: String
}
