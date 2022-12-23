use std::fs;
use std::path::Path;

use log::info;

use crate::syspass::Account;
use crate::types::{EmptyResult, OperationResult};

pub const ACCOUNTS_CACHE_FILENAME: &str = "accounts.cache";

pub fn save_accounts_into_file(accounts: &Vec<Account>, filepath: &Path) -> EmptyResult {
    info!("save accounts into cache-file '{}'", filepath.display());
    let content = serde_json::to_string(accounts)?;

    fs::write(&filepath, &content)?;

    Ok(())
}

pub fn load_accounts_from_file(filepath: &Path) -> OperationResult<Vec<Account>> {
    info!("load accounts from cache-file '{}'", filepath.display());
    let content = fs::read_to_string(filepath)?;

    let accounts = serde_json::from_str::<Vec<Account>>(&content)?;

    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use crate::cache::{load_accounts_from_file, save_accounts_into_file};
    use crate::tests::account::get_sample_account;

    #[test]
    fn save_and_load() {
        let account1 = get_sample_account();
        let account2 = get_sample_account();
        let account3 = get_sample_account();

        let accounts = vec![account1, account2, account3];

        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();

        save_accounts_into_file(&accounts, &file_path).unwrap();
        let results = load_accounts_from_file(&file_path).unwrap();

        assert_eq!(accounts, results);
    }
}
