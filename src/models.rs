use jiff::Timestamp;
use uuid::Uuid;
//use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct LoginItem {
    pub id: Uuid,
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
