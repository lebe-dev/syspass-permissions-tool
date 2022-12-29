use std::fs;
use std::path::Path;

use log::info;
use serde::{Deserialize, Serialize};

use crate::types::{EmptyResult, OperationResult};

pub const ACCOUNTS_SET_CACHE_FILENAME: &str = "accounts-set.cache";
pub const ACCOUNTS_GET_CACHE_FILENAME: &str = "accounts-get.cache";

pub fn save_cache_data_into_file<T: Serialize>(data: &T, filepath: &Path) -> EmptyResult {
    info!("save data into cache-file '{}'", filepath.display());
    let content = serde_json::to_string(data)?;
    fs::write(&filepath, &content)?;
    Ok(())
}

pub fn load_cache_data_from_file<T: for<'a> Deserialize<'a>>(filepath: &Path) -> OperationResult<T> {
    info!("load cache data from file '{}'", filepath.display());
    let content = fs::read_to_string(filepath)?;
    let data = serde_json::from_str::<T>(&content)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use crate::cache::{load_cache_data_from_file, save_cache_data_into_file};
    use crate::syspass::Account;
    use crate::tests::account::get_sample_account;

    #[test]
    fn save_and_load_accounts() {
        let account1 = get_sample_account();
        let account2 = get_sample_account();
        let account3 = get_sample_account();

        let accounts = vec![account1, account2, account3];

        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();

        save_cache_data_into_file(&accounts, &file_path).unwrap();
        let results: Vec<Account> = load_cache_data_from_file(&file_path).unwrap();

        assert_eq!(accounts, results);
    }

    #[test]
    fn save_and_load_account() {
        let account = get_sample_account();

        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();

        save_cache_data_into_file(&account, &file_path).unwrap();
        let results: Account = load_cache_data_from_file(&file_path).unwrap();

        assert_eq!(account, results);
    }
}
