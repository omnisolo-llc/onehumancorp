
    use super::*;

    #[test]
    fn test_db_new_fails_without_server() {
        temp_env::with_vars(vec![("DATABASE_URL", Some("postgres://localhost:54321/nonexistent"))], || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
                let db = DB::new().await;
                assert!(db.is_err());
            });
        });
    }
