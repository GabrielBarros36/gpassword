use std::error::Error;
use std::path::Path;
use std::{fs, process};

fn main() {
    if let Err(_e) = init(None) {
        process::exit(1);
    }
    println!("Hello, world!");
}

fn init(custom_dir: Option<&Path>) -> Result<(), Box<dyn Error>> {
    let vault_dir = match custom_dir {
        Some(dir) => dir.to_path_buf(),
        None => dirs::home_dir().ok_or("Could not determine home directory")?,
    };
    let vault_path = vault_dir.join("vault.json");
    let exists = Path::exists(&vault_path);
    if !exists {
        fs::File::create(&vault_path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_init() {
        assert!(init(None).is_ok());
    }

    #[test]
    fn test_init_creates_vault_file() {
        let temp_dir = TempDir::new().expect("Could not create temp directory");
        let temp_path = temp_dir.path();
        let vault_path = temp_path.join("vault.json");

        assert!(!vault_path.exists());
        assert!(init(Some(temp_path)).is_ok());
        assert!(vault_path.exists());
    }

    #[test]
    fn test_init_does_not_overwrite_existing_file() {
        let temp_dir = TempDir::new().expect("Could not create temp directory");
        let temp_path = temp_dir.path();
        let vault_path = temp_path.join("vault.json");

        fs::write(&vault_path, "existing content").expect("Could not write test file");

        assert!(init(Some(temp_path)).is_ok());

        let content = fs::read_to_string(&vault_path).expect("Could not read vault file");
        assert_eq!(content, "existing content");
    }
}
