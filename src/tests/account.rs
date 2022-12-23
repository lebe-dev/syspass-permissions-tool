use crate::syspass::Account;
use crate::tests::get_random_string;

pub fn get_sample_account() -> Account {
    Account {
        name: get_random_string(),
        login: get_random_string(),
        category: get_random_string(),
        client: get_random_string(),
    }
}
