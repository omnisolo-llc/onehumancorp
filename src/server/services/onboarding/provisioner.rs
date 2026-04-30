use std::fs;
use std::path::Path;

pub fn provision_environment(is_cloud: bool) -> Result<(), String> {
    let base_dir = if is_cloud { ".ohc-cloud-data" } else { ".ohc-local-data" };
    
    let dirs = vec![
        format!("{}/db", base_dir),
        format!("{}/blob", base_dir),
        format!("{}/config", base_dir),
    ];

    for dir in dirs {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).map_err(|e| e.to_string())?;
        }
    }





    if !is_cloud {
        let db_path = format!("{}/db/ohc-standalone.db", base_dir);
        // Load encryption key from config or env, fallback to secure generation but since we don't persist it,
        // fallback to env var or config
        let enc_key = crate::config::get().sqlite_encryption_key.clone().unwrap_or_else(|| std::env::var("OHC_SQLITE_KEY").unwrap_or_else(|_| "default_secure_key".to_string()));

        let opts = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true)
            .pragma("key", enc_key);

        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
            let _ = rt.block_on(
                sqlx::sqlite::SqlitePoolOptions::new().connect_with(opts)
            );
        }).join().unwrap();
    }

    // TODO: Increment metrics
    
    Ok(())
}

pub fn check_environment(is_cloud: bool) -> Result<(), String> {
    let base_dir = if is_cloud { ".ohc-cloud-data" } else { ".ohc-local-data" };
    
    let dirs = vec![
        format!("{}/db", base_dir),
        format!("{}/blob", base_dir),
        format!("{}/config", base_dir),
    ];

    for dir in dirs {
        if !Path::new(&dir).exists() {
            return Err(format!("directory {} does not exist", dir));
        }
    }

    Ok(())
}

pub fn cleanup_environment(is_cloud: bool) -> Result<(), String> {
    let base_dir = if is_cloud { ".ohc-cloud-data" } else { ".ohc-local-data" };
    
    if Path::new(base_dir).exists() {
        fs::remove_dir_all(base_dir).map_err(|e| e.to_string())?;
    }
    



    // TODO: Increment metrics
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_provision_environment_local() {
        let res = provision_environment(false);
        assert!(res.is_ok());

        let expected_dirs = vec![
            ".ohc-local-data/db",
            ".ohc-local-data/blob",
            ".ohc-local-data/config",
        ];

        for dir in expected_dirs {
            assert!(Path::new(&dir).exists());
        }

        fs::remove_dir_all(".ohc-local-data").unwrap();
    }

    #[test]
    fn test_provision_environment_cloud() {
        let res = provision_environment(true);
        assert!(res.is_ok());

        let expected_dirs = vec![
            ".ohc-cloud-data/db",
            ".ohc-cloud-data/blob",
            ".ohc-cloud-data/config",
        ];

        for dir in expected_dirs {
            assert!(Path::new(&dir).exists());
        }

        fs::remove_dir_all(".ohc-cloud-data").unwrap();
    }

    #[test]
    fn test_check_environment_local() {
        let _ = fs::remove_dir_all(".ohc-local-data");

        let res = check_environment(false);
        assert!(res.is_err());

        provision_environment(false).unwrap();
        let res = check_environment(false);
        assert!(res.is_ok());

        fs::remove_dir_all(".ohc-local-data").unwrap();
    }

    #[test]
    fn test_check_environment_cloud() {
        let _ = fs::remove_dir_all(".ohc-cloud-data");

        let res = check_environment(true);
        assert!(res.is_err());

        provision_environment(true).unwrap();
        let res = check_environment(true);
        assert!(res.is_ok());

        fs::remove_dir_all(".ohc-cloud-data").unwrap();
    }

    #[test]
    fn test_cleanup_environment_local() {
        provision_environment(false).unwrap();
        let res = cleanup_environment(false);
        assert!(res.is_ok());
        assert!(check_environment(false).is_err());
    }

    #[test]
    fn test_cleanup_environment_cloud() {
        provision_environment(true).unwrap();
        let res = cleanup_environment(true);
        assert!(res.is_ok());
        assert!(check_environment(true).is_err());
    }
}
