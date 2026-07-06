use std::fs;
use std::path::Path;
use opentelemetry::global;

pub fn provision_environment(is_cloud: bool) -> Result<(), String> {
    let base_dir = if is_cloud { ".ohc-cloud-data" } else { ".ohc-local-data" };
    
    let dirs = vec![
        format!("{}/db", base_dir),
        format!("{}/blob", base_dir),
        format!("{}/config", base_dir),
    ];

    for dir in dirs {
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create directory {}: {}", dir, e))?;
        tracing::debug!("Successfully created directory: {}", dir);
    }

    global::meter("ohc.onboarding")
        .u64_counter("ohc.provisioner.environments_created")
        .build()
        .add(1, &[]);
    
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
            tracing::debug!("Environment check failed: directory {} does not exist", dir);
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
    
    global::meter("ohc.onboarding")
        .u64_counter("ohc.provisioner.environments_cleaned")
        .build()
        .add(1, &[]);
    
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
        if std::path::Path::new(".ohc-local-data").exists() { fs::remove_dir_all(".ohc-local-data").unwrap(); }

        let res = check_environment(false);
        assert!(res.is_err());

        provision_environment(false).unwrap();
        let res = check_environment(false);
        assert!(res.is_ok());

        fs::remove_dir_all(".ohc-local-data").unwrap();
    }

    #[test]
    fn test_check_environment_cloud() {
        if std::path::Path::new(".ohc-cloud-data").exists() { fs::remove_dir_all(".ohc-cloud-data").unwrap(); }

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
