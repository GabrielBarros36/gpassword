use jiff::Timestamp;
//use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct LoginItem {
    pub username: String, // can be an email
    pub password: String,
    #[serde(with = "jiff::fmt::serde::timestamp::second::required")]
    pub created_at: Timestamp,
    #[serde(with = "jiff::fmt::serde::timestamp::second::required")]
    pub last_modified: Timestamp,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Vault {
    pub item_list: Vec<LoginItem>,
}
