pub mod models;

pub use models::{LoginItem, Vault};
use uuid::Uuid;

impl Vault {
    pub fn add_item(username: String, password: String, vault: &mut Vault) {
        let login_item: LoginItem = LoginItem {
            id: (Uuid::new_v4()),
            username: (username),
            password: (password),
            created_at: (jiff::Timestamp::now()),
            last_modified: (jiff::Timestamp::now()),
        };

        vault.item_list.push(login_item.clone());
    }

    pub fn update_item(username: Option<String>, password: Option<String>, item: &mut LoginItem) {
        if !(username.is_none() && password.is_none()) {
            if let Some(usn) = username {
                item.username = usn;
            }
            if let Some(pwd) = password {
                item.password = pwd;
            }

            item.last_modified = jiff::Timestamp::now()
        }
    }

    pub fn delete_item_by_username(username: String, vault: &mut Vault) {
        vault.item_list.retain(|x| x.username != username);
    }
}

#[cfg(test)]
mod tests {

    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn test_from_str() {
        let data = r#"
          {
            "item_list": [
                {
                    "id": "550e8400-e29b-41d4-a716-446655440000",
                    "username": "Gabe1",
                    "password": "password1",
                    "created_at": 1718825365,
                    "last_modified": 1718825366
                },
                {
                    "id": "550e8400-e29b-41d4-a716-446655440000",
                    "username": "Gabe2",
                    "password": "password2",
                    "created_at": 1718825367,
                    "last_modified": 1718825368
                }
            ]
        }
        "#;

        let vault: Vault = serde_json::from_str(data).unwrap();

        assert_eq!(vault.item_list.len(), 2);
        assert_eq!(vault.item_list[0].username, "Gabe1");
        assert_eq!(vault.item_list[0].password, "password1");
        assert_eq!(vault.item_list[1].username, "Gabe2");
        assert_eq!(vault.item_list[1].password, "password2");
    }

    #[test]
    fn test_add_item() {
        let mut vault: Vault = Vault {
            item_list: Vec::new(),
        };
        assert_eq!(vault.item_list.len(), 0);

        Vault::add_item("Gabe".to_string(), "password".to_string(), &mut vault);
        assert_eq!(vault.item_list.len(), 1);
        assert_eq!(vault.item_list[0].username, "Gabe");
        assert_eq!(vault.item_list[0].password, "password");
    }

    #[test]
    fn test_update_item_username_and_password() {
        let mut vault: Vault = Vault {
            item_list: vec![LoginItem {
                id: Uuid::new_v4(),
                username: "Gabe".to_string(),
                password: "password".to_string(),
                last_modified: jiff::Timestamp::now(),
                created_at: jiff::Timestamp::now(),
            }],
        };

        assert_eq!(vault.item_list[0].username, "Gabe");
        assert_eq!(vault.item_list[0].password, "password");
        sleep(Duration::from_millis(100));

        Vault::update_item(
            Some("Leo".to_string()),
            Some("pass word".to_string()),
            &mut vault.item_list[0],
        );

        assert_eq!(vault.item_list[0].username, "Leo");
        assert_eq!(vault.item_list[0].password, "pass word");
        assert_ne!(
            vault.item_list[0].created_at,
            vault.item_list[0].last_modified
        );
    }

    #[test]
    fn test_update_item_username_only() {
        let mut vault: Vault = Vault {
            item_list: vec![LoginItem {
                id: Uuid::new_v4(),
                username: "Gabe".to_string(),
                password: "password".to_string(),
                last_modified: jiff::Timestamp::now(),
                created_at: jiff::Timestamp::now(),
            }],
        };

        assert_eq!(vault.item_list[0].username, "Gabe");
        sleep(Duration::from_millis(100));
        Vault::update_item(Some("Leo".to_string()), None, &mut vault.item_list[0]);
        assert_eq!(vault.item_list[0].username, "Leo");
        assert_ne!(
            vault.item_list[0].created_at,
            vault.item_list[0].last_modified
        );
    }

    #[test]
    fn test_update_item_password_only() {
        let mut vault: Vault = Vault {
            item_list: vec![LoginItem {
                id: Uuid::new_v4(),
                username: "Gabe".to_string(),
                password: "password".to_string(),
                last_modified: jiff::Timestamp::now(),
                created_at: jiff::Timestamp::now(),
            }],
        };

        assert_eq!(vault.item_list[0].password, "password");
        sleep(Duration::from_millis(100));
        Vault::update_item(None, Some("pass word".to_string()), &mut vault.item_list[0]);
        assert_eq!(vault.item_list[0].password, "pass word");
        assert_ne!(
            vault.item_list[0].created_at,
            vault.item_list[0].last_modified
        );
    }

    #[test]
    fn test_delete_by_username() {
        let mut vault: Vault = Vault {
            item_list: vec![
                LoginItem {
                    id: Uuid::new_v4(),
                    username: "Gabe".to_string(),
                    password: "password".to_string(),
                    last_modified: jiff::Timestamp::now(),
                    created_at: jiff::Timestamp::now(),
                },
                LoginItem {
                    id: Uuid::new_v4(),
                    username: "Leo".to_string(),
                    password: "password".to_string(),
                    last_modified: jiff::Timestamp::now(),
                    created_at: jiff::Timestamp::now(),
                },
                LoginItem {
                    id: Uuid::new_v4(),
                    username: "Leo".to_string(),
                    password: "password".to_string(),
                    last_modified: jiff::Timestamp::now(),
                    created_at: jiff::Timestamp::now(),
                },
            ],
        };

        assert_eq!(vault.item_list.len(), 3);
        Vault::delete_item_by_username("Gabe".to_string(), &mut vault);
        assert_eq!(vault.item_list.len(), 2);
        Vault::delete_item_by_username("Leo".to_string(), &mut vault);
        assert_eq!(vault.item_list.len(), 0);
    }
}
