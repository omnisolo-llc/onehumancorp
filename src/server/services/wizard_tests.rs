pub use super::wizard::*;

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;
    use ::server_ohc::orchestration::EmptyRequest;
    use std::sync::Mutex;
    use std::sync::OnceLock;

    static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn test_verify_onboarding_standalone_sqlite_ok() {
        let _guard = env_lock();
        temp_env::with_vars(vec![("STANDALONE_MODE", Some("true")), ("DATABASE_URL", Some("sqlite://local.db"))], || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
                let service = MyWizardService::new();
                let request = Request::new(EmptyRequest {});
                let response = service.verify_onboarding(request).await.unwrap().into_inner();
                assert_eq!(response.status, "healthy");
                assert_eq!(response.mode, "standalone");
                let has_ok_db = response.diagnostics.iter().any(|d| d.check == "DATABASE_URL" && d.status == "ok");
                assert!(has_ok_db);
                let has_hybrid_check = response.diagnostics.iter().any(|d| d.check == "HYBRID_MODE_SWITCHING" && d.status == "ok");
                assert!(has_hybrid_check);
                let has_local_sync_check = response.diagnostics.iter().any(|d| d.check == "LOCAL_TO_CLOUD_SYNC" && d.status == "ok");
                assert!(has_local_sync_check);
            });
        });
    }

    #[test]
    fn test_verify_onboarding_standalone_sqlite_missing() {
        let _guard = env_lock();
        temp_env::with_vars(vec![("STANDALONE_MODE", Some("true")), ("DATABASE_URL", None::<&str>)], || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
                let service = MyWizardService::new();
                let request = Request::new(EmptyRequest {});
                let response = service.verify_onboarding(request).await.unwrap().into_inner();
                assert_eq!(response.status, "degraded");
                assert_eq!(response.mode, "standalone");
                let has_missing_db = response.diagnostics.iter().any(|d| d.check == "DATABASE_URL" && d.status == "missing");
                assert!(has_missing_db);
            });
        });
    }

    #[test]
    fn test_verify_onboarding_standalone_sqlite_invalid() {
        let _guard = env_lock();
        temp_env::with_vars(vec![("STANDALONE_MODE", Some("true")), ("DATABASE_URL", Some("postgres://localhost/db"))], || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
                let service = MyWizardService::new();
                let request = Request::new(EmptyRequest {});
                let response = service.verify_onboarding(request).await.unwrap().into_inner();
                assert_eq!(response.status, "degraded");
                assert_eq!(response.mode, "standalone");
                let has_invalid_db = response.diagnostics.iter().any(|d| d.check == "DATABASE_URL" && d.status == "invalid");
                assert!(has_invalid_db);
            });
        });
    }

    #[test]
    fn test_verify_onboarding_hybrid_mode_probes() {
        let _guard = env_lock();
        temp_env::with_vars(vec![("STANDALONE_MODE", Some("false")), ("DATABASE_URL", Some("postgres://db")), ("REDIS_URL", Some("redis://cache"))], || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
                let service = MyWizardService::new();
                let request = Request::new(EmptyRequest {});
                let response = service.verify_onboarding(request).await.unwrap().into_inner();
                assert_eq!(response.status, "healthy");
                assert_eq!(response.mode, "cloud");
                let has_hybrid_check = response.diagnostics.iter().any(|d| d.check == "HYBRID_MODE_SWITCHING" && d.status == "ok");
                assert!(has_hybrid_check);
                let has_local_sync_check = response.diagnostics.iter().any(|d| d.check == "LOCAL_TO_CLOUD_SYNC" && d.status == "ok");
                assert!(has_local_sync_check);
            });
        });
    }
}
