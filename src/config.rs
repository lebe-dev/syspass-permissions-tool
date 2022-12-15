use std::fs;
use std::path::Path;

use log::info;
use serde::Deserialize;

use crate::types::OperationResult;

#[derive(Deserialize,PartialEq,Debug)]
pub struct AppConfig {
    #[serde(rename(deserialize = "api-auth"))]
    pub api_auth: ApiAuthConfig,
    pub auth: AuthConfig,

    #[serde(rename(deserialize = "ignore-errors"))]
    pub ignore_errors: bool,

    #[serde(rename(deserialize = "user-permissions"))]
    pub user_permissions: PermissionsConfig,
    #[serde(rename(deserialize = "group-permissions"))]
    pub group_permissions: PermissionsConfig,
}

#[derive(Deserialize,PartialEq,Debug)]
pub struct AuthConfig {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize,PartialEq,Debug)]
pub struct ApiAuthConfig {
    pub token: String,
    #[serde(rename(deserialize = "token-pass"))]
    pub token_pass: String
}

#[derive(Deserialize,PartialEq,Debug)]
pub struct PermissionsConfig {
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

    use crate::config::{ApiAuthConfig, AppConfig, AuthConfig, load_config_from_file, PermissionsConfig};
    use crate::CONFIG_FILE;

    #[test]
    fn load_config_test() {
        let config_file = Path::new("test-data").join(CONFIG_FILE);

        let config = load_config_from_file(config_file.as_path()).unwrap();

        let expected_config = AppConfig {
            api_auth: ApiAuthConfig {
                token: "r4ndom0s".to_string(),
                token_pass: "g9285gj9284vfj".to_string(),
            },
            auth: AuthConfig {
                login: "b2y63nu46n456".to_string(),
                password: "2b34t45ynn968m".to_string(),
            },
            ignore_errors: true,
            user_permissions: PermissionsConfig {
                view: vec![],
                edit: vec![],
            },
            group_permissions: PermissionsConfig {
                view: vec!["group1".to_string(), "group2".to_string()],
                edit: vec!["group1".to_string(), "group2".to_string()]
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
