use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const CURRENT_USER_DIR: &str = ".omnisolo";
const LEGACY_USER_DIR: &str = ".ohc";
const STATE_FILE_MIGRATIONS: [(&str, &str); 5] = [
    ("ohc-standalone.db", "omnisolo-standalone.db"),
    ("ohc-standalone.db-wal", "omnisolo-standalone.db-wal"),
    ("ohc-standalone.db-shm", "omnisolo-standalone.db-shm"),
    (".ohc_sqlite_key", ".omnisolo_sqlite_key"),
    (".ohc_jwt_secret", ".omnisolo_jwt_secret"),
];

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub listen_addr: String,
    pub grpc_addr: String,
    pub database_url: Option<String>,
    pub standalone: bool,
    pub sqlite_encryption_key: Option<String>,
    pub redis_url: Option<String>,
    pub multitenant: bool,
    pub headless: bool,
    pub minimax_api_key: Option<String>,
    pub anthropic_api_key: Option<String>,
    pub openai_api_key: Option<String>,
    pub llm_provider: Option<String>,
    pub llm_model: Option<String>,
    pub local_llm_endpoint: Option<String>,
    pub max_tokens: i32,
    pub max_iterations: Option<i32>,
    pub max_context_messages: Option<i32>,
    pub agent_token: Option<String>,
    pub agent_auth_disabled: bool,
    pub agent_cert_file: Option<String>,
    pub agent_key_file: Option<String>,
    pub agent_ca_file: Option<String>,
    pub agent_spiffe_id: Option<String>,
    pub agent_address: String,
    pub agent_id: Option<String>,
    pub builtin_agent_binary: Option<String>,
    pub cloud_autodream_endpoint: Option<String>,
    pub cloud_telemetry_endpoint: Option<String>,
    pub cloud_missions_endpoint: Option<String>,
    pub cloud_context_endpoint: Option<String>,
    pub telemetry_enabled: bool,
    pub registration_enabled: bool,
    pub bootstrap_org_id: String,
    pub bootstrap_org_name: String,
    pub bootstrap_ceo_name: String,
    pub bootstrap_org_domain: Option<String>,
    pub jwt_secret: Option<String>,
    pub s3_endpoint: Option<String>,
    pub s3_bucket_blobs: String,
}

static INSTANCE: OnceLock<AppConfig> = OnceLock::new();

use std::sync::atomic::{AtomicBool, Ordering};

pub static DYNAMIC_TELEMETRY_ENABLED: AtomicBool = AtomicBool::new(false);

#[inline]
pub fn is_telemetry_enabled() -> bool {
    get().telemetry_enabled || DYNAMIC_TELEMETRY_ENABLED.load(Ordering::Relaxed)
}

pub fn get() -> &'static AppConfig {
    INSTANCE.get_or_init(|| load().expect("Failed to load configuration"))
}

pub fn load() -> Result<AppConfig, ::config::ConfigError> {
    let s = ::config::Config::builder()
        // Defaults
        .set_default("listen_addr", ":8080")?
        .set_default("grpc_addr", ":9090")?
        .set_default("agent_address", "127.0.0.1:50051")?
        .set_default("max_tokens", 2048)?
        .set_default("s3_bucket_blobs", "omnisolo-blobs")?
        .set_default("bootstrap_org_id", "bootstrap")?
        .set_default("bootstrap_org_name", "Bootstrap Organization")?
        .set_default("bootstrap_ceo_name", "Platform Admin")?
        .set_default("standalone", false)?
        .set_default("multitenant", false)?
        .set_default("headless", false)?
        .set_default("agent_auth_disabled", false)?
        .set_default("telemetry_enabled", false)?
        .set_default("registration_enabled", false)?
        // Optional file
        .add_source(::config::File::with_name("omnisolo").required(false))
        .add_source(::config::File::with_name("~/.openclaw/omnisolo").required(false))

        // Env vars with OMNISOLO_ prefix
        .add_source(::config::Environment::with_prefix("OmniSolo"))
        // Env vars without prefix (for standard ones like DATABASE_URL)
        .add_source(::config::Environment::default())
        .build()?;

    let mut cfg: AppConfig = s.try_deserialize()?;

    if cfg.max_tokens == 0 {
        cfg.max_tokens = 2048;
    } else if cfg.max_tokens > 4096 {
        cfg.max_tokens = 4096;
    }

    // Standalone enforcement
    cfg = StandaloneModeEnforcer.enforce(cfg);

    Ok(cfg)
}


fn is_real_directory(path: &Path) -> bool {
    std::fs::symlink_metadata(path)
        .is_ok_and(|metadata| metadata.is_dir() && !metadata.file_type().is_symlink())
}

fn create_private_directory(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;

        let _ = std::fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(path);
    }
    #[cfg(not(unix))]
    {
        let _ = std::fs::create_dir_all(path);
    }
}

fn migrate_state_files(source: &Path, destination: &Path) {
    for (legacy_name, current_name) in STATE_FILE_MIGRATIONS {
        let legacy = source.join(legacy_name);
        let current = destination.join(current_name);
        if !legacy.exists() || current.exists() {
            continue;
        }
        if let Err(error) = std::fs::rename(&legacy, &current) {
            tracing::warn!(
                source = %legacy.display(),
                destination = %current.display(),
                %error,
                "could not migrate legacy OmniSolo state file"
            );
        }
    }
}

fn prepare_user_dir(base: &Path) -> PathBuf {
    let current = base.join(CURRENT_USER_DIR);
    let legacy = base.join(LEGACY_USER_DIR);
    let selected = if current.exists() {
        current.clone()
    } else if is_real_directory(&legacy) {
        match std::fs::rename(&legacy, &current) {
            Ok(()) => current.clone(),
            Err(error) => {
                tracing::warn!(
                    source = %legacy.display(),
                    destination = %current.display(),
                    %error,
                    "could not rename legacy OmniSolo state directory"
                );
                legacy.clone()
            }
        }
    } else {
        current.clone()
    };

    create_private_directory(&selected);
    migrate_state_files(&selected, &selected);
    if selected == current && is_real_directory(&legacy) {
        migrate_state_files(&legacy, &current);
    }
    selected
}

fn state_file_path(current_name: &str, legacy_name: &str) -> PathBuf {
    let directory = get_safe_user_dir();
    let current = directory.join(current_name);
    let legacy = directory.join(legacy_name);
    if current.exists() || !legacy.exists() {
        current
    } else {
        legacy
    }
}

pub fn standalone_database_path() -> PathBuf {
    state_file_path("omnisolo-standalone.db", "ohc-standalone.db")
}

pub fn sqlite_key_path() -> PathBuf {
    state_file_path(".omnisolo_sqlite_key", ".ohc_sqlite_key")
}

pub fn jwt_secret_path() -> PathBuf {
    state_file_path(".omnisolo_jwt_secret", ".ohc_jwt_secret")
}

pub fn get_safe_user_dir() -> PathBuf {
    if let Ok(home) = std::env::var("USERPROFILE") {
        prepare_user_dir(&PathBuf::from(home))
    } else if let Ok(home) = std::env::var("HOME") {
        prepare_user_dir(&PathBuf::from(home))
    } else {
        prepare_user_dir(Path::new("."))
    }
}
pub trait ModeEnforcer {
    fn enforce(&self, cfg: AppConfig) -> AppConfig;
}

pub struct StandaloneModeEnforcer;

impl ModeEnforcer for StandaloneModeEnforcer {
    fn enforce(&self, mut cfg: AppConfig) -> AppConfig {
        let is_test =
            std::env::var("TEST_WORKSPACE").is_ok() || std::env::var("TEST_TMPDIR").is_ok();
        let env_standalone =
            std::env::var("OMNISOLO_STANDALONE_MODE").unwrap_or_else(|_| "false".to_string()) == "true";
        let has_database_source =
            cfg.database_url.is_some() || std::env::var_os("DATABASE_URL_FILE").is_some();
        let is_standalone = env_standalone || cfg.standalone || (!is_test && !has_database_source);

        if !is_standalone {
            return cfg;
        }

        let default_sqlite_path = standalone_database_path();
        let default_sqlite_url = format!("sqlite://{}", default_sqlite_path.to_string_lossy());

        let base_sqlite_url = if let Some(db_url) = &cfg.database_url {
            if db_url.starts_with("sqlite://") {
                db_url.split('?').next().unwrap_or(db_url).to_string()
            } else {
                tracing::info!(
                    "standalone: non-SQLite OMNISOLO_DATABASE_URL is ignored in standalone desktop builds; using SQLite"
                );
                default_sqlite_url
            }
        } else {
            default_sqlite_url
        };

        if let Some(redis_url) = &cfg.redis_url
            && !redis_url.is_empty()
        {
            tracing::info!(
                "standalone: REDIS_URL is ignored in standalone desktop builds; using embedded NATS"
            );
        }

        let sqlite_url = if let Some(key) = &cfg.sqlite_encryption_key {
            if !key.is_empty() {
                base_sqlite_url // Let db.rs handle pragma key via connection options
            } else if let Ok(_fallback_key) = std::env::var("OMNISOLO_SQLITE_KEY") {
                base_sqlite_url
            } else {
                base_sqlite_url
            }
        } else if let Ok(_fallback_key) = std::env::var("OMNISOLO_SQLITE_KEY") {
            base_sqlite_url
        } else {
            base_sqlite_url
        };
        cfg.database_url = Some(sqlite_url.clone());

        // Set proper file permissions for local storage wrapper in standalone mode atomically
        #[cfg(unix)]
        {
            if !is_test {
                use std::fs::OpenOptions;
                use std::os::unix::fs::OpenOptionsExt;
                use std::os::unix::fs::PermissionsExt;

                let db_path = sqlite_url.strip_prefix("sqlite://").unwrap_or(sqlite_url.as_str()).split('?').next().unwrap_or("omnisolo-standalone.db");
                if let Some(parent) = std::path::Path::new(db_path).parent()
                    && !parent.as_os_str().is_empty()
                {
                    #[cfg(unix)]
                    use std::os::unix::fs::DirBuilderExt;
                    let mut builder = std::fs::DirBuilder::new();
                    builder.recursive(true);
                    #[cfg(unix)]
                    builder.mode(0o700);
                    let _ = builder.create(parent);
                }
                let mut opts = OpenOptions::new();
                opts.read(true)
                    .write(true)
                    .create(true)
                    .truncate(false)
                    .mode(0o600);
                #[cfg(target_os = "linux")]
                opts.custom_flags(0x00020000); // O_NOFOLLOW
                #[cfg(target_os = "macos")]
                opts.custom_flags(0x0100); // O_NOFOLLOW

                match opts.open(db_path) {
                    Ok(file) => {
                        if let Ok(metadata) = file.metadata() {
                            let mut perms = metadata.permissions();
                            if perms.mode() & 0o777 != 0o600 {
                                perms.set_mode(0o600);
                                if let Err(e) = file.set_permissions(perms) {
                                    tracing::error!(
                                        "Failed to securely update existing standalone database file permissions: {}",
                                        e
                                    );
                                    panic!(
                                        "Failed to securely update existing standalone database file permissions: {}",
                                        e
                                    ); // Fail-closed gracefully
                                }
                            }
                        }

                        // Pre-create SQLite auxiliary files (-wal and -shm) with secure permissions
                        // to prevent them from inheriting the default umask (e.g. 0644).
                        #[cfg(unix)]
                        {
                            let wal_path = format!("{}-wal", db_path);
                            let shm_path = format!("{}-shm", db_path);

                            for ext_path in [&wal_path, &shm_path] {
                                let mut aux_opts = OpenOptions::new();
                                aux_opts.read(true).write(true).create(true).mode(0o600);
                                #[cfg(target_os = "linux")]
                                aux_opts.custom_flags(0x00020000); // O_NOFOLLOW
                                #[cfg(target_os = "macos")]
                                aux_opts.custom_flags(0x0100); // O_NOFOLLOW

                                if let Ok(aux_file) = aux_opts.open(ext_path)
                                    && let Ok(metadata) = aux_file.metadata()
                                {
                                    let mut p = metadata.permissions();
                                    if (p.mode() & 0o777) != 0o600 {
                                        p.set_mode(0o600);
                                        let _ = aux_file.set_permissions(p);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        panic!(
                            "Failed to securely create or open standalone database file with restricted permissions: {}",
                            e
                        );
                    }
                }
            }
        }
        cfg.standalone = true;
        cfg.redis_url = None;
        cfg.multitenant = false;

        // Strict opt-in constraint for local sovereignty in standalone
        let explicit_opt_in = std::env::var("OMNISOLO_TELEMETRY_ENABLED")
            .map(|s| s.to_lowercase() == "true")
            .unwrap_or(false);
        if explicit_opt_in {
            tracing::info!("standalone: Telemetry explicitly opted-in by user.");
            cfg.telemetry_enabled = true;
        } else {
            cfg.telemetry_enabled = false;
        }
        cfg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn migrates_legacy_standalone_state_without_overwriting_omnisolo_files() {
        let base = std::env::temp_dir().join(format!(
            "omnisolo-user-state-migration-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let legacy = base.join(".ohc");
        std::fs::create_dir_all(&legacy).unwrap();
        for (name, value) in [
            ("ohc-standalone.db", "database"),
            ("ohc-standalone.db-wal", "wal"),
            ("ohc-standalone.db-shm", "shm"),
            (".ohc_sqlite_key", "sqlite-key"),
            (".ohc_jwt_secret", "jwt-secret"),
        ] {
            std::fs::write(legacy.join(name), value).unwrap();
        }

        let migrated = prepare_user_dir(&base);

        assert_eq!(migrated, base.join(".omnisolo"));
        assert!(!legacy.exists());
        for (name, value) in [
            ("omnisolo-standalone.db", "database"),
            ("omnisolo-standalone.db-wal", "wal"),
            ("omnisolo-standalone.db-shm", "shm"),
            (".omnisolo_sqlite_key", "sqlite-key"),
            (".omnisolo_jwt_secret", "jwt-secret"),
        ] {
            assert_eq!(std::fs::read_to_string(migrated.join(name)).unwrap(), value);
        }

        std::fs::remove_dir_all(&base).unwrap();
    }

    #[test]
    fn preserves_existing_omnisolo_state_when_legacy_state_is_also_present() {
        let base = std::env::temp_dir().join(format!(
            "omnisolo-user-state-collision-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let current = base.join(".omnisolo");
        let legacy = base.join(".ohc");
        std::fs::create_dir_all(&current).unwrap();
        std::fs::create_dir_all(&legacy).unwrap();
        std::fs::write(current.join("omnisolo-standalone.db"), "current").unwrap();
        std::fs::write(legacy.join("ohc-standalone.db"), "legacy").unwrap();

        let selected = prepare_user_dir(&base);

        assert_eq!(selected, current);
        assert_eq!(
            std::fs::read_to_string(selected.join("omnisolo-standalone.db")).unwrap(),
            "current"
        );
        assert_eq!(
            std::fs::read_to_string(legacy.join("ohc-standalone.db")).unwrap(),
            "legacy"
        );
        std::fs::remove_dir_all(&base).unwrap();
    }

    #[test]
    fn test_load_defaults() {
        let _lock = ENV_MUTEX.lock().unwrap();
        // Ensure environment doesn't interfere
        // SAFETY: Test-only code removing environment variables
        unsafe {
            env::remove_var("OMNISOLO_LISTEN_ADDR");
            env::remove_var("OMNISOLO_DATABASE_URL");
        }

        let cfg = load().unwrap();
        assert_eq!(cfg.listen_addr, ":8080");
        assert_eq!(cfg.max_tokens, 2048);
        assert_eq!(cfg.s3_bucket_blobs, "omnisolo-blobs");
    }

    #[test]
    fn test_load_env_vars() {
        let _lock = ENV_MUTEX.lock().unwrap();
        // SAFETY: Test-only code setting/removing environment variables
        unsafe {
            env::set_var("OMNISOLO_LISTEN_ADDR", ":9999");
            env::set_var("OMNISOLO_DATABASE_URL", "postgres://localhost/testdb");
        }

        let cfg = load().unwrap();
        assert_eq!(cfg.listen_addr, ":9999");
        assert_eq!(cfg.database_url.unwrap(), "postgres://localhost/testdb");

        // SAFETY: Test-only code setting/removing environment variables
        unsafe {
            env::remove_var("OMNISOLO_LISTEN_ADDR");
            env::remove_var("OMNISOLO_DATABASE_URL");
        }
    }

    #[test]
    fn database_url_file_prevents_implicit_standalone_default() {
        let _lock = ENV_MUTEX.lock().unwrap();
        temp_env::with_vars(
            [
                ("TEST_WORKSPACE", None::<&str>),
                ("TEST_TMPDIR", None::<&str>),
                ("DATABASE_URL", None::<&str>),
                ("DATABASE_URL_FILE", Some("/run/secrets/database-url")),
                ("OMNISOLO_DATABASE_URL", None::<&str>),
                ("OMNISOLO_STANDALONE_MODE", None::<&str>),
            ],
            || {
                let cfg = load().unwrap();
                assert!(!cfg.standalone);
            },
        );
    }

    #[test]
    fn test_telemetry_enabled_override() {
        let _lock = ENV_MUTEX.lock().unwrap();
        // SAFETY: Test-only code setting environment variables
        unsafe {
            env::set_var("OMNISOLO_TELEMETRY_ENABLED", "true");
        }

        let cfg = load().unwrap();
        assert!(cfg.telemetry_enabled);

        // SAFETY: Test-only code setting/removing environment variables
        unsafe {
            env::set_var("OMNISOLO_TELEMETRY_ENABLED", "false");
        }

        let cfg2 = load().unwrap();
        assert!(!(cfg2.telemetry_enabled));

        unsafe {
            env::remove_var("OMNISOLO_TELEMETRY_ENABLED");
        }
    }

    #[test]
    fn test_standalone_mode_enforcer_default() {
        let _lock = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("OMNISOLO_STANDALONE_MODE", "true");
            env::remove_var("OMNISOLO_DATABASE_URL");
            env::remove_var("OMNISOLO_TELEMETRY_ENABLED");
        }

        let cfg = load().unwrap();
        assert!(cfg.standalone);
        assert!(!cfg.multitenant);
        assert!(cfg.redis_url.is_none());
        assert!(!cfg.telemetry_enabled); // Default to off in standalone
        assert!(cfg.database_url.unwrap().starts_with("sqlite://"));

        unsafe {
            env::remove_var("OMNISOLO_STANDALONE_MODE");
        }
    }

    #[test]
    fn test_standalone_mode_enforcer_with_telemetry_opt_in() {
        let _lock = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("OMNISOLO_STANDALONE_MODE", "true");
            env::set_var("OMNISOLO_TELEMETRY_ENABLED", "true");
        }

        let cfg = load().unwrap();
        assert!(cfg.standalone);
        assert!(cfg.telemetry_enabled); // Opted-in!

        unsafe {
            env::remove_var("OMNISOLO_STANDALONE_MODE");
            env::remove_var("OMNISOLO_TELEMETRY_ENABLED");
        }
    }

    #[test]
    fn test_standalone_mode_enforcer_with_redis_ignored() {
        let _lock = ENV_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("OMNISOLO_STANDALONE_MODE", "true");
            env::set_var("REDIS_URL", "redis://localhost:6379");
        }

        let cfg = load().unwrap();
        assert!(cfg.standalone);
        // Redis should be forced to None in standalone mode
        assert!(cfg.redis_url.is_none());

        unsafe {
            env::remove_var("OMNISOLO_STANDALONE_MODE");
            env::remove_var("REDIS_URL");
        }
    }
}
