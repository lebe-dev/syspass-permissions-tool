use serde::Deserialize;

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct AuthConfig {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ApiAuthConfig {
    pub token: String,
    #[serde(rename(deserialize = "token-pass"))]
    pub token_pass: String
}

#[derive(Deserialize)]
pub struct PermissionsConfig {
    pub view: Vec<String>,
    pub edit: Vec<String>,
}
