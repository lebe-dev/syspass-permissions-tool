use std::fs;
use std::path::Path;

use log::info;
use serde::Deserialize;

use crate::types::OperationResult;

#[derive(Deserialize,PartialEq,Debug)]
pub struct AppConfig {
    #[serde(rename(deserialize = "syspass-url"))]
    pub syspass_url: String,

    #[serde(rename(deserialize = "webdriver-url"))]
    pub webdriver_url: String,

    pub auth: AuthConfig,

    #[serde(rename(deserialize = "ignore-errors"))]
    pub ignore_errors: bool,

    pub permissions: PermissionsConfig
}

#[derive(Deserialize,PartialEq,Debug)]
pub struct AuthConfig {
    pub login: String,
    pub password: String,
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

#[derive(Deserialize,PartialEq,Debug)]
pub struct EntityPermissionsConfig {
    pub view: Vec<String>,
    pub edit: Vec<String>,
}

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

    use crate::config::{AppConfig, AuthConfig, EntityPermissionsConfig, load_config_from_file, PermissionsConfig};
    use crate::CONFIG_FILE;

    #[test]
    fn load_config_test() {
        let config_file = Path::new("test-data").join(CONFIG_FILE);

        let config = load_config_from_file(config_file.as_path()).unwrap();

        let expected_config = AppConfig {
            syspass_url: "http://localhost:18080".to_string(),
            webdriver_url: "http://localhost:9515".to_string(),
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
