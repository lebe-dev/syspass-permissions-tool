use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;

use log::info;
use serde::Deserialize;

use crate::types::OperationResult;

#[derive(Deserialize,PartialEq,Debug)]
pub struct AppConfig {
    #[serde(rename(deserialize = "syspass-url"))]
    pub syspass_url: String,

    pub webdriver: WebDriverConfig,

    pub auth: AuthConfig,

    #[serde(rename(deserialize = "ignore-errors"))]
    pub ignore_errors: bool,

    pub permissions: PermissionsConfig,

    pub delays: DelaysConfig
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<AppConfig>")?;
        write!(f, "syspass-url: '{}', webdriver-url: '{}', ", self.syspass_url, self.webdriver)?;
        write!(f, "ignore-errors: {}, ", self.ignore_errors)?;
        write!(f, "auth: {}", self.auth)?;
        write!(f, "permissions: {}", self.permissions)?;
        write!(f, "delays: {}", self.delays)?;
        write!(f, "</AppConfig>")
    }
}

#[derive(Deserialize,PartialEq,Debug)]
pub struct WebDriverConfig {
    pub url: String,
    pub args: Vec<String>,
}

impl Display for WebDriverConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<WebDriverConfig> url '{}', args: '{:?}'</WebDriverConfig>", self.url, self.args)
    }
}

#[derive(Deserialize,PartialEq,Debug)]
pub struct AuthConfig {
    pub login: String,
    pub password: String,
}

impl Display for AuthConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "login: '{}', password: '*********'", self.login)
    }
}

#[derive(Deserialize,PartialEq,Debug)]
pub struct PermissionsConfig {
    pub user: EntityPermissionsConfig,
    pub group: EntityPermissionsConfig,

    pub owner: String,

    #[serde(rename(deserialize = "main-group"))]
    pub main_group: String,

    #[serde(rename(deserialize = "private-account"))]
    pub private_account: bool,

    #[serde(rename(deserialize = "private-account-for-group"))]
    pub private_account_for_group: bool
}

impl Display for PermissionsConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<PermissionsConfig>")?;
        write!(f, "user: {:?}, ", self.user)?;
        write!(f, "group: {:?}, ", self.group)?;
        write!(f, "owner: '{}', ", self.owner)?;
        write!(f, "main-group: '{}', ", self.main_group)?;
        write!(f, "private-account: {}, ", self.private_account)?;
        write!(f, "private-account-for-group: {}, ", self.private_account_for_group)?;
        write!(f, "</PermissionsConfig>")
    }
}

#[derive(Deserialize,PartialEq,Debug)]
pub struct EntityPermissionsConfig {
    pub view: Vec<String>,
    pub edit: Vec<String>,
}

#[derive(Deserialize,PartialEq,Debug)]
pub struct DelaysConfig {

    /// Delay after success login into sysPass
    #[serde(rename(deserialize = "after-login"))]
    pub after_login: u64,

    /// Delay after redirect to index page
    #[serde(rename(deserialize = "after-redirect-to-index"))]
    pub after_redirect_to_index: u64,

    /// Delay after redirect to edit page
    #[serde(rename(deserialize = "after-redirect-to-edit"))]
    pub after_redirect_to_edit: u64,

    /// Delay after search
    #[serde(rename(deserialize = "after-search"))]
    pub after_search: u64,

    /// Delay after menu open
    #[serde(rename(deserialize = "menu-open"))]
    pub menu_open: u64
}

impl Display for DelaysConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Delays>")?;
        write!(f, "after-login: {} (ms)", self.after_login)?;
        write!(f, "after-redirect-to-index: {} (ms)", self.after_redirect_to_index)?;
        write!(f, "after-redirect-to-edit: {} (ms)", self.after_redirect_to_edit)?;
        write!(f, "after-search: {} (ms)", self.after_search)?;
        write!(f, "menu-open: {} (ms)", self.menu_open)?;
        write!(f, "</Delays>")
    }
}

// ---

pub fn load_config_from_file(file_path: &Path) -> OperationResult<AppConfig> {
    info!("load config from file '{}'", file_path.display());
    let content = fs::read_to_string(file_path)?;
    let config: AppConfig = serde_yaml::from_str(&content)?;
    info!("config:");
    info!("{:?}", config);
    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use fake::{Fake, Faker};

    use crate::config::{AppConfig, AuthConfig, DelaysConfig, EntityPermissionsConfig, load_config_from_file, PermissionsConfig, WebDriverConfig};
    use crate::CONFIG_FILE;

    #[test]
    fn load_config_test() {
        let config_file = Path::new("test-data").join(CONFIG_FILE);

        let config = load_config_from_file(config_file.as_path()).unwrap();

        let expected_config = AppConfig {
            syspass_url: "http://localhost:18080".to_string(),

            auth: AuthConfig {
                login: "b2y63nu46n456".to_string(),
                password: "2b34t45ynn968m".to_string(),
            },
            ignore_errors: true,
            permissions: PermissionsConfig {
                user: EntityPermissionsConfig {
                    view: vec!["sysPass Admin".to_string()],
                    edit: vec![
                        "Mr.Editor".to_string(),
                        "sysPass Admin".to_string()
                    ],
                },
                group: EntityPermissionsConfig {
                    view: vec!["Admins".to_string()],
                    edit: vec![
                        "Beta Group".to_string(),
                        "Demo group 1".to_string()
                    ],
                },
                owner: "Mr.Editor".to_string(),
                main_group: "Demo group 1".to_string(),
                private_account: false,
                private_account_for_group: true
            },
            delays: DelaysConfig {
                after_login: 300,
                after_redirect_to_index: 500,
                after_redirect_to_edit: 500,
                after_search: 500,
                menu_open: 300,
            },
            webdriver: WebDriverConfig {
                url: "http://localhost:9515".to_string(),
                args: vec![
                    "--headless".to_string()
                ],
            },
        };

        assert_eq!(config, expected_config);
    }

    #[test]
    fn return_error_for_unknown_file() {
        let filename = Faker.fake::<String>();
        let config_file = Path::new("test-data").join(filename);

        assert!(load_config_from_file(config_file.as_path()).is_err());
    }
}
