use super::*;

    #[test]
    fn test_settings_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.listen_addr, "0.0.0.0:18789");
        assert_eq!(settings.db_path, Some("ohc.db".to_string()));
    }

    #[test]
    fn test_store_save_and_load() {
        let file_path = PathBuf::from("test_settings.json");

        // Clean up before test
        if file_path.exists() {
            std::fs::remove_file(&file_path).unwrap();
        }

        let store = Store::from_file(file_path.clone()).unwrap();
        store.set_extra("key1".to_string(), "value1".to_string()).unwrap();

        assert!(file_path.exists());

        let store2 = Store::from_file(file_path.clone()).unwrap();
        let settings = store2.get();
        assert_eq!(settings.extras.get("key1").unwrap(), "value1");

        // Clean up after test
        std::fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_store_from_file_errors() {
        // Bad JSON
        let mut file_path = std::env::temp_dir();
        file_path.push("bad_settings.json");
        std::fs::write(&file_path, "{bad json").unwrap();

        let result = Store::from_file(file_path.clone());
        assert!(result.is_err());

        std::fs::remove_file(&file_path).unwrap();

        // Unreadable file (directory)
        let dir_path = std::env::temp_dir().join("some_dir");
        std::fs::create_dir(&dir_path).unwrap();
        let result = Store::from_file(dir_path.clone());
        assert!(result.is_err());
        std::fs::remove_dir(&dir_path).unwrap();
    }

    #[test]
    fn test_store_save_errors() {
        let store = Store {
            data: RwLock::new(AppSettings::default()),
            path: Some(PathBuf::from("/root/unauthorized/file.json")),
        };
        let result = store.save();
        assert!(result.is_err());
    }
