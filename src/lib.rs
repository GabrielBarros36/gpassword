pub mod models;

use std::error::Error;

pub use models::{LoginItem, Vault};

pub fn get_vault(vault: &str) -> Result<Vault, Box<dyn Error>> {
    let vault: Vault = serde_json::from_str(vault)?;
    Ok(vault)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_vault() {
        let data = r#"
          {
            "item_list": [
                {
                    "username": "Gabe1",
                    "password": "password1",
                    "created_at": 1718825365,
                    "last_modified": 1718825366
                },
                {
                    "username": "Gabe2",
                    "password": "password2",
                    "created_at": 1718825367,
                    "last_modified": 1718825368
                }
            ]
        }
        "#;

        let vault: Vault = get_vault(&data).unwrap();

        assert_eq!(vault.item_list.len(), 2);
        assert_eq!(vault.item_list[0].username, "Gabe1");
        assert_eq!(vault.item_list[0].password, "password1");
        assert_eq!(vault.item_list[1].username, "Gabe2");
        assert_eq!(vault.item_list[1].password, "password2");
    }
}
